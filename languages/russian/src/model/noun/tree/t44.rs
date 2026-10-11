use flexion::Path;

/// Words ending -нушка.
pub(crate) fn n22390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22258);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48914);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77712);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 77713);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 101606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77714);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101607);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118483);
									s.is_empty()
								}
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
			super::push(out, 48915);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77715);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 77716);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 48916);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77717);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 48917);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77718);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101611);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118484);
									s.is_empty()
								}
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
			super::push(out, 48918);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 77719);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 77720);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101612);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 101613);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118485);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77721);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101614);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77722);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101615);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118487);
									s.is_empty()
								}
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
			super::push(out, 48919);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77723);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 77724);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101616);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77725);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77726);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101618);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77727);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101619);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 118490);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129057);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 101620);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77728);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101621);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101622);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118493);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129058);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 118494);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129059);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135313);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t49::n140349(s, out),
														_ => false,
													}
												}
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
							super::push(out, 101623);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118495);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 129060);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135314);
													match s {
														[] => true,
														['х', s @ ..] => super::t49::n140350(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77729);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 101624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 77730);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 48920);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77731);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101626);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 48921);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77732);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 48922);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77733);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 48923);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77734);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 77735);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101629);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118496);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 129061);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135315);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48924);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77736);
					match s {
						[] => true,
						['о', s @ ..] => super::t49::n102717(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t49::n49430(s, out),
		['с', s @ ..] => super::t49::n49431(s, out),
		['ч', s @ ..] => super::t49::n49432(s, out),
		['я', s @ ..] => super::t49::n49433(s, out),
		_ => false,
	}
}

/// Words ending -оушка.
pub(crate) fn n22391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22259);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 48929);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 48930);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пушка.
pub(crate) fn n22392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22260);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 48931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77744);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 101637);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118501);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129063);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 135316);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140351(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77745);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 77746);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101639);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118502);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 129064);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 48932);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77747);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101641);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 101642);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118503);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129065);
											s.is_empty()
										}
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
					super::push(out, 77749);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 77750);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101643);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118504);
									s.is_empty()
								}
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
			super::push(out, 48933);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77751);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48934);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 77752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 77753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48935);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77754);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 101646);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118505);
									s.is_empty()
								}
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
			super::push(out, 48936);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77755);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77756);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 48937);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101649);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 101650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рушка.
pub(crate) fn n22393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22261);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48938);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77758);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101651);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 77759);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101652);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77760);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101653);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118508);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129066);
											s.is_empty()
										}
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
					super::push(out, 77761);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 48939);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77762);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101655);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118509);
									s.is_empty()
								}
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
			super::push(out, 48940);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77763);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 77764);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 77765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 48941);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 77766);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 101656);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118510);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129067);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 135317);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n140352(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48942);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77767);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48943);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77769);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101658);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101659);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 77770);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 77771);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77772);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48944);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77773);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101662);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118513);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129068);
											s.is_empty()
										}
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
					super::push(out, 77774);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 77775);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101663);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 101664);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118514);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129069);
											match s {
												[] => true,
												['ш', s @ ..] => {
													super::push(out, 135318);
													match s {
														[] => true,
														['ы', s @ ..] => super::t49::n140353(s, out),
														_ => false,
													}
												}
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
							super::push(out, 101665);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118515);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129070);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135319);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 77776);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 101666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 77777);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77778);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 48945);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77779);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 77780);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77781);
					match s {
						[] => true,
						['е', s @ ..] => super::t49::n102758(s, out),
						['о', s @ ..] => {
							super::push(out, 101672);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t49::n49451(s, out),
		_ => false,
	}
}

/// Words ending -сушка.
pub(crate) fn n22394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22262);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 48947);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77783);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48948);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тушка.
pub(crate) fn n22395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22263);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48949);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77784);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77785);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101675);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118520);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129072);
											s.is_empty()
										}
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
					super::push(out, 77786);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77787);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101677);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 77788);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101678);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118521);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129073);
											s.is_empty()
										}
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
					super::push(out, 77789);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118522);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77790);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101680);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118523);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48950);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77792);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101682);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77793);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48951);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77794);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101683);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118524);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 101684);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118525);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129074);
											s.is_empty()
										}
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
					super::push(out, 77795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101685);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 77796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 48952);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77797);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 101686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 77798);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48953);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77799);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101688);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118526);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129075);
											s.is_empty()
										}
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
					super::push(out, 77800);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101689);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118527);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77801);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 77802);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 77803);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101690);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 118528);
									s.is_empty()
								}
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
			super::push(out, 48954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 77804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48955);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77805);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 77806);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101692);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118529);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 118530);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129076);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135321);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140355(s, out),
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
									super::push(out, 118531);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129077);
											match s {
												[] => true,
												['ч', s @ ..] => {
													super::push(out, 135322);
													s.is_empty()
												}
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
							super::push(out, 101693);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118532);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129078);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 101694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => super::t49::n49461(s, out),
		['ь', s @ ..] => {
			super::push(out, 48957);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77813);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101702);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => super::t49::n49463(s, out),
		_ => false,
	}
}

/// Words ending -фушка.
pub(crate) fn n22396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22264);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48959);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 77815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48960);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77816);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101704);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хушка.
pub(crate) fn n22397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22265);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 48961);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 77817);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48962);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77818);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101706);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 48963);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77819);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чушка.
pub(crate) fn n22398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22266);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48964);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 48965);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77820);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 101708);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 77821);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101709);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 48966);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 48967);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77823);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101710);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118540);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129081);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135324);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 48968);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101711);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77825);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 77826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 48969);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77827);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шушка.
pub(crate) fn n22399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22267);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 48970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -обиль-вышка.
pub(crate) fn n136731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135325);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 138921);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 140966);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мпа-вспышка.
pub(crate) fn n136732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135326);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138922);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140967);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тышка.
pub(crate) fn n22408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22276);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 48992);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 77849);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 48993);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77850);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 48994);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 77851);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101725);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118547);
									s.is_empty()
								}
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
			super::push(out, 48995);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77852);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101726);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 48996);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77853);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нюшка.
pub(crate) fn n22423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22291);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 49025);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 49026);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77881);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101741);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118556);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129090);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49027);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 49028);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 77882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101742);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 77883);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101743);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118557);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 77884);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 77885);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101744);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118558);
									s.is_empty()
								}
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
			super::push(out, 49029);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77886);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 77887);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 101746);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118559);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129091);
											s.is_empty()
										}
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
					super::push(out, 77888);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101747);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118560);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 129092);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49030);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 77889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 49031);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 77890);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 77891);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101749);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рюшка.
pub(crate) fn n22424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22292);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 49032);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 49033);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77892);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101750);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 49034);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77893);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 77894);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 101752);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118561);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129093);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135327);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 49035);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 77895);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 77896);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 77897);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 77898);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101754);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118562);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129094);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49036);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тюшка.
pub(crate) fn n22425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22293);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 49037);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 77899);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 101755);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 118563);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129095);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135328);
													match s {
														[] => true,
														['ц', s @ ..] => super::t49::n140358(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 77900);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 49038);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 77901);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101757);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 49039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77902);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 77903);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 77904);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 101760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 49040);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 77905);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьюшка.
pub(crate) fn n22426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22294);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 49041);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 77906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 49042);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 49043);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 77907);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 101761);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 77908);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101762);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озяюшка.
pub(crate) fn n78734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77909);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 101763);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -беременяшка.
pub(crate) fn n136735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135329);
	s.is_empty()
}

/// Words ending -ряшка.
pub(crate) fn n22437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22305);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 49065);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 77927);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 49066);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 77928);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101779);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 101780);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118580);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129098);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49067);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 77929);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101781);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118581);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -льяшка.
pub(crate) fn n49575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49070);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 77930);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 101782);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 118582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рия-туфелька.
pub(crate) fn n140359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138924);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 140968);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 142161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ресло-люлька.
pub(crate) fn n140360(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138925);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 140969);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ка-встанька.
pub(crate) fn n136741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135335);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 138926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -енька.
pub(crate) fn n22463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22331);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 49135);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 78006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 101842);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 49136);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78007);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 49137);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 78008);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101844);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118616);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 129116);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135336);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 78009);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101845);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118617);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 129117);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135337);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 49138);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78010);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101846);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 78011);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 101847);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 49139);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78012);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101848);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 101849);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 101850);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118619);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 129118);
											s.is_empty()
										}
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
					super::push(out, 78013);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 101851);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118620);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129119);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135338);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 49140);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78014);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101852);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118621);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129120);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 135339);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 78015);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 101853);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118622);
									s.is_empty()
								}
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
			super::push(out, 49141);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78016);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 78017);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101854);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118623);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129121);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 101855);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 101856);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 118624);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129122);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 101857);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118625);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129123);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49142);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78018);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 49143);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 78019);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101859);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118626);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129124);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 49144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78020);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 78021);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 101860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 78022);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 101861);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118627);
									s.is_empty()
								}
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
			super::push(out, 49145);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78023);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 101862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 78024);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 101863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 49146);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 78025);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101864);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118629);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129125);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t49::n49656(s, out),
		['ф', s @ ..] => {
			super::push(out, 49148);
			s.is_empty()
		}
		['ч', s @ ..] => super::t49::n49658(s, out),
		['ш', s @ ..] => super::t49::n49659(s, out),
		['щ', s @ ..] => {
			super::push(out, 49151);
			match s {
				[] => true,
				['е', s @ ..] => super::t49::n78867(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -резонька.
pub(crate) fn n102989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101901);
	s.is_empty()
}

/// Words ending -изонька.
pub(crate) fn n78879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78052);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 101902);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лонька.
pub(crate) fn n49668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49159);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 78053);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 101903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 78054);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 101904);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 118647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -монька.
pub(crate) fn n49669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49160);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78055);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 101905);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 78056);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 101906);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 78057);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -понька.
pub(crate) fn n49670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49161);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 78058);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 101907);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сонька.
pub(crate) fn n49671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49162);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 78059);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 101908);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 101909);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 78060);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 101910);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 101911);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118648);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 129132);
									s.is_empty()
								}
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
			super::push(out, 78061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тонька.
pub(crate) fn n49672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49163);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 78062);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 101912);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118649);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хонька.
pub(crate) fn n49673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49164);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78063);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 101913);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118650);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 78064);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 101914);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -унька.
pub(crate) fn n22466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22334);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 49165);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 49166);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 49167);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ынька.
pub(crate) fn n22467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22335);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 49168);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 49169);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78065);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 101915);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49170);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78066);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 101916);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 101917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 49171);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78067);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 101918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 78068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 49172);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78069);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 101919);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118651);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129134);
											s.is_empty()
										}
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
					super::push(out, 78070);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 101920);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118652);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129135);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135341);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янька.
pub(crate) fn n22468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22336);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 49173);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 78071);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 101921);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118653);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 129136);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135342);
													match s {
														[] => true,
														['а', s @ ..] => super::t49::n140362(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -екинг-смола.
pub(crate) fn n136753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135347);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 138929);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word полубластула.
pub(crate) fn n142415(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140970);
	s.is_empty()
}

/// Words ending -трипаносома.
pub(crate) fn n136759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135353);
	s.is_empty()
}

/// The word анкилостома.
pub(crate) fn n136760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135354);
	s.is_empty()
}

/// Words ending -ахипзадовна.
pub(crate) fn n136766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135360);
	s.is_empty()
}

/// The word германовна.
pub(crate) fn n136767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135361);
	s.is_empty()
}

/// Words ending -тельмановна.
pub(crate) fn n136768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135362);
	s.is_empty()
}

/// Words ending -харитоновна.
pub(crate) fn n136769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135363);
	s.is_empty()
}

/// Words ending -ровна.
pub(crate) fn n23088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22953);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 49754);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 78469);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102145);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 49755);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 78470);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102146);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118762);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 129179);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135364);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 78471);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102147);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118763);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129180);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135365);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 78472);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 102148);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118764);
									s.is_empty()
								}
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
			super::push(out, 49756);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78473);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 102149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 78474);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 78475);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102150);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118765);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129181);
											s.is_empty()
										}
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
					super::push(out, 78476);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 78477);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 78478);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102153);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 49757);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 78479);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118766);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 78480);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 102155);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товна.
pub(crate) fn n23090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22955);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 49759);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 49760);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 78481);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 102156);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118768);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129182);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фовна.
pub(crate) fn n23091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22956);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 49761);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 78482);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102157);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118769);
									s.is_empty()
								}
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
			super::push(out, 49762);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78483);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховна.
pub(crate) fn n23092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22957);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 49763);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78484);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 102158);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118770);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 129183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пролегомена.
pub(crate) fn n136772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135366);
	s.is_empty()
}

/// Words ending -довина.
pub(crate) fn n50402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49890);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 78586);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 102211);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 118800);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 78587);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 102212);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ковина.
pub(crate) fn n50403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78588);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 102213);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 102214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 78589);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 102215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 78590);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 102216);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 102217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 78591);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102218);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 118802);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129196);
									s.is_empty()
								}
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
			super::push(out, 78592);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 78593);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 102220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 78594);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 102221);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 78595);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 102222);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 78596);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 102223);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 102224);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 102225);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 118804);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 78597);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102226);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 118805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 78598);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 102227);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 118806);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 129197);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 135368);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ловина.
pub(crate) fn n50404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49892);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 78599);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 78600);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 102228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 102229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 78601);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 78602);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мовина.
pub(crate) fn n50405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49893);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 78603);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 102230);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 102231);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118808);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новина.
pub(crate) fn n50406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49894);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 78604);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 78605);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 102232);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 118809);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 78606);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102233);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 118810);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повина.
pub(crate) fn n50407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49895);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 78607);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 102234);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровина.
pub(crate) fn n50408(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49896);
	match s {
		[] => true,
		['б', 'о', s @ ..] => {
			super::push(out, 78608);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 102235);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 78609);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 102236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совина.
pub(crate) fn n50409(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 78610);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 78611);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 78612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 102237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товина.
pub(crate) fn n50410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49897);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 102238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 118813);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 102239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 78613);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 102240);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 118814);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 78614);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102241);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 118815);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ховина.
pub(crate) fn n50411(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49898);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 78615);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 102242);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118816);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 129198);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 135369);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твина.
pub(crate) fn n23186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23051);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 49899);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьвина.
pub(crate) fn n23187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23052);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 49900);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78617);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 102243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -явина.
pub(crate) fn n23188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23053);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 49901);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78618);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102244);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -подина.
pub(crate) fn n50442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49929);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 78650);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -родина.
pub(crate) fn n50443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49930);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78651);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 102262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 78652);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 102263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 78653);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 102264);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рдина.
pub(crate) fn n23199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23064);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 49931);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78654);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 78655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 49932);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 78656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 49933);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 78657);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удина.
pub(crate) fn n23200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23065);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 49934);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49935);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 49936);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78660);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдина.
pub(crate) fn n23201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23066);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 49937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юдина.
pub(crate) fn n23202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23067);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 49938);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78661);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 102266);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118823);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129200);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135370);
													match s {
														[] => true,
														['с', s @ ..] => super::t49::n140370(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядина.
pub(crate) fn n23203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23068);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 49939);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78662);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 102267);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 118824);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129201);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135371);
													match s {
														[] => true,
														['и', s @ ..] => super::t49::n140371(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 49940);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 78663);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 49941);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 78664);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102268);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118825);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -баблина.
pub(crate) fn n79560(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78732);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 102322);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раблина.
pub(crate) fn n79561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78733);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 102323);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еблина.
pub(crate) fn n50520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50007);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 78734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -облина.
pub(crate) fn n50521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50008);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 78735);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 102324);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -влина.
pub(crate) fn n23235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23100);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50009);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 78736);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 102325);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 78737);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -глина.
pub(crate) fn n23236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23101);
	s.is_empty()
}

/// Words ending -длина.
pub(crate) fn n23237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23102);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 50010);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -елина.
pub(crate) fn n23238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23103);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 50011);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 50012);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78738);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 102326);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118855);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129205);
											s.is_empty()
										}
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
					super::push(out, 78739);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 102327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 78740);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 50013);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78741);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 78742);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 102329);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 118856);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129206);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135372);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 78743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 50014);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 78744);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 50015);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 50016);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 78746);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 50017);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 50018);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 78747);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102332);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 78748);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 102333);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118858);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129207);
											s.is_empty()
										}
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
					super::push(out, 78749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50019);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78750);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 50020);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 78751);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 102335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 78752);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102336);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118859);
									s.is_empty()
								}
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
			super::push(out, 50021);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78753);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 102337);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118860);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129208);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135373);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 50022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78754);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 50023);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78755);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 102338);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118861);
									s.is_empty()
								}
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
			super::push(out, 50024);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78756);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 102339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 78757);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102340);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118862);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 78758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -злина.
pub(crate) fn n23239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23104);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 50025);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 78759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -илина.
pub(crate) fn n23240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23105);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 50026);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 78760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 50027);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78761);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 102341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118863);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йлина.
pub(crate) fn n23241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23106);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50028);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 78762);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102342);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118864);
									s.is_empty()
								}
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
			super::push(out, 50029);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78763);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 102343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -клина.
pub(crate) fn n23242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23107);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50030);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78764);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 78765);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 50031);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 78766);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50032);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 78767);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102346);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 50033);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78768);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 102347);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118865);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олина.
pub(crate) fn n23243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23108);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50034);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 78769);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102348);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118866);
									s.is_empty()
								}
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
			super::push(out, 50035);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 78770);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 50036);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78771);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102350);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118867);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 129209);
											s.is_empty()
										}
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
					super::push(out, 78772);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102351);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 118868);
									s.is_empty()
								}
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
			super::push(out, 50037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50038);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78774);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102352);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 50039);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78775);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 102353);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118869);
									s.is_empty()
								}
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
			super::push(out, 50040);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78776);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102354);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118870);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129210);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плина.
pub(crate) fn n23244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23109);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50041);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 78777);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 50042);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 78778);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102356);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 118871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлина.
pub(crate) fn n23245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23110);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50043);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 78779);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слина.
pub(crate) fn n23246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23111);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50044);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 78780);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 78781);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102357);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118872);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129211);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 50045);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 78782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102358);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50046);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78783);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102359);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 118873);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлина.
pub(crate) fn n23247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23112);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50047);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 78784);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102360);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улина.
pub(crate) fn n23248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50048);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 50049);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 50050);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 50051);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50052);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 78789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50053);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78790);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102363);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118874);
									s.is_empty()
								}
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
			super::push(out, 50054);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 78791);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хлина.
pub(crate) fn n23249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23114);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 50055);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шлина.
pub(crate) fn n23250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23115);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 50056);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78793);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ылина.
pub(crate) fn n23251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23116);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50057);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 50058);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78794);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 102364);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118875);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -элина.
pub(crate) fn n23252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23117);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 50059);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юлина.
pub(crate) fn n23253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23118);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 50060);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 78795);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102365);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялина.
pub(crate) fn n23254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23119);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 50061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -предсовмина.
pub(crate) fn n136780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135374);
	s.is_empty()
}

/// Words ending -ильгельмина.
pub(crate) fn n136781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135375);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 138937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -домотканина.
pub(crate) fn n136782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135376);
	s.is_empty()
}

/// Words ending -еонина.
pub(crate) fn n50617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50104);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 78833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ионина.
pub(crate) fn n50618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50105);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 78834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -конина.
pub(crate) fn n50619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50106);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 78835);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 102386);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 118886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лонина.
pub(crate) fn n50620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50107);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 78836);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 102387);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 102388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 78837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ронина.
pub(crate) fn n50621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50108);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 78838);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 102389);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118887);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 129219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сонина.
pub(crate) fn n50622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50109);
	match s {
		[] => true,
		['ь', s @ ..] => {
			super::push(out, 78839);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 102390);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 118888);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 129220);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тонина.
pub(crate) fn n50623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50110);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 78840);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 78841);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пнина.
pub(crate) fn n23274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23139);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50111);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78842);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнина.
pub(crate) fn n23275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23140);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50112);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 50113);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 78843);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102392);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118889);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129221);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 50114);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -снина.
pub(crate) fn n23276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23141);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50115);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78844);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118890);
									s.is_empty()
								}
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
			super::push(out, 50116);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 78845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чнина.
pub(crate) fn n23277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23142);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 50117);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78846);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102394);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118891);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шнина.
pub(crate) fn n23278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23143);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 50118);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78847);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янина.
pub(crate) fn n23279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23144);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 50119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дрина.
pub(crate) fn n23291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23156);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50151);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 78884);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 78885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 50152);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 78886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 50153);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78887);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 102416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 50154);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78888);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102417);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118905);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129225);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135377);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 50155);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78889);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерина.
pub(crate) fn n23292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23157);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 50156);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 78890);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 50157);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 78891);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102418);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118906);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 129226);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 50158);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 50159);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78893);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102419);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 102420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 78894);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 78895);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 102422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 50160);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 50161);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78896);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 78897);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102423);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 50162);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78898);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ирина.
pub(crate) fn n23293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23158);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 50163);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 50164);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78899);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 78900);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102425);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118907);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129227);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орина.
pub(crate) fn n23294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23159);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50165);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 50166);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78902);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 102426);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 78903);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 102427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 50167);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78904);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 102428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 50168);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78905);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102429);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50169);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78906);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102430);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118908);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 78907);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102431);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 50170);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 78908);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102432);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 50171);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78909);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102433);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 118909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 78910);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 78911);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102435);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 78912);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102436);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 118911);
									s.is_empty()
								}
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
			super::push(out, 50172);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 78913);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102437);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118912);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чуприна.
pub(crate) fn n79742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78914);
	s.is_empty()
}

/// Words ending -трина.
pub(crate) fn n23296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23161);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50174);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 78915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102438);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 78916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 50175);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 78917);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 50176);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 78918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 50177);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78919);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 102440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 50178);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 78920);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 78921);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урина.
pub(crate) fn n23297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23162);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 50179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78922);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 102443);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118914);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129228);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ырина.
pub(crate) fn n23298(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23163);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 50180);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 78923);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ран-укосина.
pub(crate) fn n136784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135378);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 138938);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ососина.
pub(crate) fn n79772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78944);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 102462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -псина.
pub(crate) fn n23303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23168);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 50198);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 50199);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78945);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 50200);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 78946);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рсина.
pub(crate) fn n23304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23169);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 50201);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 78947);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссина.
pub(crate) fn n23305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23170);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50202);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 78948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усина.
pub(crate) fn n23306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23171);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50203);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 50204);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78949);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 50205);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 78950);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102465);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысина.
pub(crate) fn n23307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23172);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 50206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьсина.
pub(crate) fn n23308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23173);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 50207);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 78951);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 102466);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118925);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 129233);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 102467);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118926);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 129234);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135379);
													match s {
														[] => true,
														['о', s @ ..] => super::t49::n140374(s, out),
														_ => false,
													}
												}
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
							super::push(out, 102468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясина.
pub(crate) fn n23309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23174);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 50208);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 78952);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50209);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50210);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 78954);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 78955);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ретина.
pub(crate) fn n79810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78982);
	s.is_empty()
}

/// Words ending -аретина.
pub(crate) fn n79811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78983);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 102487);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шетина.
pub(crate) fn n50742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50229);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 78984);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 102488);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щетина.
pub(crate) fn n50743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50230);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 78985);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лтина.
pub(crate) fn n23314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23179);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50231);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 78986);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50232);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78987);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нтина.
pub(crate) fn n23315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23180);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50233);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 78988);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 50234);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 78989);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 102491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 78990);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102492);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118935);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 78991);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 102493);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118936);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129238);
											s.is_empty()
										}
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
					super::push(out, 78992);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 102494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 50235);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 50236);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 50237);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 78993);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 102495);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отина.
pub(crate) fn n23316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23181);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50238);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 78994);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102496);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118938);
									s.is_empty()
								}
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
			super::push(out, 50239);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 78995);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102497);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 118939);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 78996);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 102498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 50240);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 78997);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 50241);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 78998);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102500);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 118940);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 78999);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102501);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 102502);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118941);
									s.is_empty()
								}
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
			super::push(out, 50242);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 79000);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 102503);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118942);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 129239);
											s.is_empty()
										}
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
					super::push(out, 79001);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102504);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 102505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 79002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50243);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 79003);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 79004);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 102507);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 118943);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 129240);
											s.is_empty()
										}
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
					super::push(out, 79005);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102508);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 118944);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129241);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135380);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 50244);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79006);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50245);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 79007);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 79008);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 102511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 50246);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 79009);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102512);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 118945);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртина.
pub(crate) fn n23317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23182);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50247);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 50248);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79011);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 102513);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 118946);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129242);
											s.is_empty()
										}
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
					super::push(out, 79012);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 102514);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118947);
									match s {
										[] => true,
										['ч', s @ ..] => {
											super::push(out, 129243);
											s.is_empty()
										}
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
					super::push(out, 79013);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 102515);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 118948);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 79014);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 102516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 50249);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79015);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стина.
pub(crate) fn n23318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23183);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50250);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 79016);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102517);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118949);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129244);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 102518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 50251);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 79017);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102519);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 118950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 102520);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 118951);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129245);
											s.is_empty()
										}
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
					super::push(out, 79018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 50252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79019);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 79020);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102521);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 102522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 79021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 50253);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 79022);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 50254);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79023);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 102523);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50255);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 79024);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 102524);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118952);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129246);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 102525);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 118953);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 129247);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 50256);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 79025);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 102526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 79026);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 102527);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 118954);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 129248);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135381);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 50257);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 79027);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 102528);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 118955);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утина.
pub(crate) fn n23319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23184);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50258);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 79028);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 50259);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 79029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102529);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50260);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79030);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фтина.
pub(crate) fn n23320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23185);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50262);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 79031);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50263);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 79032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -штина.
pub(crate) fn n23321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23186);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 50264);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ютина.
pub(crate) fn n23322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23187);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 50265);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятина.
pub(crate) fn n23323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23188);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50266);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 50267);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 50268);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 79033);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50269);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79034);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 50270);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 50271);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50272);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 79036);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 50273);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 79037);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 102534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 79038);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 79039);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ичина.
pub(crate) fn n23344(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23209);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 50306);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 50307);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 79073);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 102561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 50308);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79074);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 102562);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118969);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 79075);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 102563);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 50309);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79076);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 79077);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 79078);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 79079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 50310);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 79080);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 102564);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'и', 'к', '\u{1}', s @ ..] => {
					super::push(out, 79081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50311);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79082);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 102565);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 118971);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129252);
											s.is_empty()
										}
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
					super::push(out, 79083);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118972);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 118973);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129253);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135382);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нчина.
pub(crate) fn n23346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23211);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 50313);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79085);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102566);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -очина.
pub(crate) fn n23347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23212);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50314);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79086);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 79087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 50315);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 50316);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79088);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 102567);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 79089);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пчина.
pub(crate) fn n23348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23213);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 50317);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79090);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 50318);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 79091);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рчина.
pub(crate) fn n23349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23214);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 50319);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 79092);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -счина.
pub(crate) fn n23350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23215);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 50320);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 79093);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тчина.
pub(crate) fn n23351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23216);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50321);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 50322);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 79094);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 79095);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 102569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50323);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 50324);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -учина.
pub(crate) fn n23352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23217);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50325);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 79096);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 50326);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 50327);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 50328);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79099);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 79100);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 50329);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 79101);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 102572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118976);
									s.is_empty()
								}
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
			super::push(out, 50330);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 50331);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 79103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 50332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ычина.
pub(crate) fn n23353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23218);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 50333);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 50334);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 79104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 50335);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79105);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102573);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118977);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 129254);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ючина.
pub(crate) fn n23354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23219);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 50336);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 50337);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 79107);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102574);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ячина.
pub(crate) fn n23355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23220);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50338);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 79108);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50339);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 79109);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 50340);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 79110);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 102577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 50341);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 79111);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 79112);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 102578);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word черепашина.
pub(crate) fn n136789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135383);
	s.is_empty()
}

/// The word пол-аршина.
pub(crate) fn n136790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135384);
	s.is_empty()
}

/// Words ending -це-старшина.
pub(crate) fn n136791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135385);
	s.is_empty()
}

/// Words ending -лавстаршина.
pub(crate) fn n136792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135386);
	s.is_empty()
}

/// Words ending -ребушина.
pub(crate) fn n103680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102590);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 118988);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -душина.
pub(crate) fn n50868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50354);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 79127);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 79128);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 102591);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еушина.
pub(crate) fn n50869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50355);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 79129);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 102592);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 118989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лушина.
pub(crate) fn n50870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50356);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 79130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 102593);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оушина.
pub(crate) fn n50871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50357);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 79131);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 102594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 79132);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 102595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рушина.
pub(crate) fn n50872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50358);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 79133);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 102596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 79134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сушина.
pub(crate) fn n50873(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 79135);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 79136);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 102597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 79137);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 102598);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 118990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тушина.
pub(crate) fn n50874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50359);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 79138);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102599);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 118991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шшина.
pub(crate) fn n23365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23230);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 50360);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 79139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102600);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ышина.
pub(crate) fn n23366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23231);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 50361);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50362);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79141);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102601);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 118992);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юшина.
pub(crate) fn n23367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23232);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 50363);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79142);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 102602);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 102603);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118993);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129260);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вщина.
pub(crate) fn n23369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23234);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50367);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 79146);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 102606);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 118995);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129261);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 102607);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 118996);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129262);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 129263);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135387);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 50368);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79147);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 79148);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 102608);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 118997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 79149);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 102609);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 102610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 102611);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118998);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129264);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 135388);
													s.is_empty()
												}
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
							super::push(out, 102612);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 118999);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129265);
											s.is_empty()
										}
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
					super::push(out, 79150);
					match s {
						[] => true,
						['р', 'т', s @ ..] => {
							super::push(out, 102613);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119000);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129266);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 135389);
													s.is_empty()
												}
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
							super::push(out, 102614);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 79151);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 102615);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119002);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129267);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135390);
													s.is_empty()
												}
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
							super::push(out, 102616);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119003);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129268);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135391);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 79152);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 79153);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 102618);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119004);
									s.is_empty()
								}
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
			super::push(out, 50369);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79154);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 102619);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119005);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129269);
											s.is_empty()
										}
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
					super::push(out, 79155);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102620);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119006);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129270);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135392);
													match s {
														[] => true,
														['п', s @ ..] => super::t49::n140375(s, out),
														_ => false,
													}
												}
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
							super::push(out, 102621);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119007);
									match s {
										[] => true,
										['ъ', s @ ..] => {
											super::push(out, 129271);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135393);
													match s {
														[] => true,
														['у', s @ ..] => super::t49::n140376(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t50::n50885(s, out),
		_ => false,
	}
}

/// Words ending -гвардейщина.
pub(crate) fn n136838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135432);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138957);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 140978);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -азованщина.
pub(crate) fn n130680(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129328);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 135433);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 138958);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 140979);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ганщина.
pub(crate) fn n80013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79183);
	s.is_empty()
}

/// Words ending -занщина.
pub(crate) fn n80014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79184);
	s.is_empty()
}

/// Words ending -канщина.
pub(crate) fn n80015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79185);
	s.is_empty()
}

/// Words ending -манщина.
pub(crate) fn n80016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79186);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 102679);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 119075);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 129329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 102680);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 119076);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 129330);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135434);
									s.is_empty()
								}
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
			super::push(out, 102681);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 119077);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 129331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 135435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -панщина.
pub(crate) fn n80017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79187);
	s.is_empty()
}

/// Words ending -енщина.
pub(crate) fn n50896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50381);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 79188);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102682);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119078);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 129332);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 135436);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 79189);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102683);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119079);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 129333);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 135437);
											s.is_empty()
										}
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
					super::push(out, 102684);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 102685);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 119081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 79190);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 102686);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119082);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 129334);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 102687);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119083);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 79191);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102688);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 119084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 79192);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 102689);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 119085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 79193);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102690);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 79194);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 102691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 79195);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102692);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 119087);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 129335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инщина.
pub(crate) fn n50897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50382);
	match s {
		[] => true,
		['г', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 102693);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119088);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 102694);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119089);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 129337);
									s.is_empty()
								}
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
			super::push(out, 79196);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 102695);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 119090);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 129338);
									s.is_empty()
								}
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
			super::push(out, 79197);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102696);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 119091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 79198);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 102697);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 119092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 79199);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 102698);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 119093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 79200);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 102699);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119094);
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
				['а', s @ ..] => {
					super::push(out, 102700);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 102701);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119096);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 129339);
									s.is_empty()
								}
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
			super::push(out, 79201);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 102702);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 119097);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онщина.
pub(crate) fn n50898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50383);
	s.is_empty()
}

/// Words ending -ынщина.
pub(crate) fn n50899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50384);
	match s {
		[] => true,
		['ц', s @ ..] => {
			super::push(out, 79202);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 102703);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 119098);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 129340);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 135438);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -янщина.
pub(crate) fn n50900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50385);
	s.is_empty()
}

/// Words ending -ощина.
pub(crate) fn n23375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23240);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 50386);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 50387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пщина.
pub(crate) fn n23376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23241);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 50388);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 79203);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рщина.
pub(crate) fn n23377(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23242);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 50389);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79204);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 79205);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 102704);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119099);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129341);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135439);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 79206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 50390);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79207);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102705);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119100);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129342);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 135440);
													s.is_empty()
												}
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
							super::push(out, 102706);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119101);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129343);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135441);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 79208);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 102707);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 79209);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102708);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 50391);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 79210);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 102709);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119103);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 79211);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 102710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 79212);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 102711);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119104);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129344);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 135442);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 50392);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 79213);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 102712);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 79214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 50393);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 79215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 50394);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 79216);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 102713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тщина.
pub(crate) fn n23378(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23243);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 50395);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 79217);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ущина.
pub(crate) fn n23379(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23244);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 50396);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 50397);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 79219);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьщина.
pub(crate) fn n23380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23245);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 50398);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 79220);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 79221);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 102714);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 119105);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129345);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 135443);
													match s {
														[] => true,
														['р', s @ ..] => super::t50::n140394(s, out),
														_ => false,
													}
												}
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
							super::push(out, 102715);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 102716);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 102717);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119106);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 129346);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 135444);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140395(s, out),
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
									super::push(out, 119107);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129347);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 135445);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n140396(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'в', s @ ..] => {
					super::push(out, 79222);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 102718);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 119108);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 129348);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135446);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атье-колонна.
pub(crate) fn n140397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138962);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 140982);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word карлскрона.
pub(crate) fn n136857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135451);
	s.is_empty()
}

/// Words ending -фильм-опера.
pub(crate) fn n136866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135460);
	s.is_empty()
}

/// Words ending -авантюрьера.
pub(crate) fn n136867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135461);
	s.is_empty()
}

/// Words ending -губернатора.
pub(crate) fn n136870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135464);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 138971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -гон-контора.
pub(crate) fn n136871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135465);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138972);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 140990);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ман-метафора.
pub(crate) fn n136872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135466);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138973);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 140991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -замминистра.
pub(crate) fn n136874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135468);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 138974);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 140992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иовидеоаппаратура.
pub(crate) fn n136877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135471);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 138976);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 140993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 142830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word фотоаппаратура.
pub(crate) fn n136878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135472);
	s.is_empty()
}

/// Words ending -чет-фактура.
pub(crate) fn n136879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135473);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138977);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 140994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ниверситета.
pub(crate) fn n136886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135480);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 138981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -верхкислота.
pub(crate) fn n136888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135482);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 138982);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 140995);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пол-оборота.
pub(crate) fn n136889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135483);
	s.is_empty()
}

/// Words ending -треквартиста.
pub(crate) fn n140419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138984);
	s.is_empty()
}

/// Words ending -пол-августа.
pub(crate) fn n136893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135487);
	s.is_empty()
}

/// Words ending -вка-портниха.
pub(crate) fn n140422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138987);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 140997);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142171);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -неразбериха.
pub(crate) fn n136899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135493);
	s.is_empty()
}

/// Words ending -нтан-шутиха.
pub(crate) fn n136900(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135494);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 138988);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 140998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -городничиха.
pub(crate) fn n136901(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135495);
	s.is_empty()
}

/// Words ending -ба-повитуха.
pub(crate) fn n136903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135497);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 138990);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 140999);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ка-повитуха.
pub(crate) fn n136904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135498);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 138991);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141000);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 142172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тарашкевица.
pub(crate) fn n136907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135501);
	s.is_empty()
}

/// Words ending -илостивица.
pub(crate) fn n130817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129465);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 135502);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -квица.
pub(crate) fn n24801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24646);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 51725);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 51726);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80123);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 103215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овица.
pub(crate) fn n24802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24647);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 51727);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 80124);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103216);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119332);
									s.is_empty()
								}
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
			super::push(out, 51728);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80125);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 103217);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80126);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 103218);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 103219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 80127);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 51729);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80128);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 80129);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103222);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119334);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129466);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51730);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80130);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103223);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119335);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129467);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51731);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80131);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 103224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80132);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103225);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 119336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80133);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103226);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119337);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129468);
											s.is_empty()
										}
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
					super::push(out, 80134);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103227);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 103228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 51732);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80135);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103229);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119339);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129469);
											s.is_empty()
										}
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
					super::push(out, 80136);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80137);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103231);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119340);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129470);
											s.is_empty()
										}
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
					super::push(out, 80138);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103232);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 80139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 51733);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80140);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 103234);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119342);
									s.is_empty()
								}
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
			super::push(out, 51734);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80141);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 51735);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 80142);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103236);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119343);
									s.is_empty()
								}
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
			super::push(out, 51736);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80143);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103237);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119344);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 129471);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рвица.
pub(crate) fn n24803(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24648);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 51737);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80144);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -твица.
pub(crate) fn n24804(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24649);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 51738);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80145);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 51739);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80146);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьвица.
pub(crate) fn n24805(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24650);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 51740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -явица.
pub(crate) fn n24806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24651);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 51741);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 51742);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80148);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рмилица.
pub(crate) fn n81031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80195);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 103272);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 119364);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 129481);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 135505);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 138993);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 141002);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n143624(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тилица.
pub(crate) fn n52317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51794);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 80197);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 103273);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 119365);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 129482);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 135506);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 138994);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ллица.
pub(crate) fn n24835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24680);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51795);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 80198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51796);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80199);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103274);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119366);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129483);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 135507);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -емлица.
pub(crate) fn n52320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51797);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 80200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -олица.
pub(crate) fn n24837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24682);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 51798);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 80201);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 103275);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80202);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103276);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119368);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129484);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 135508);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80203);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 103277);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 119369);
									s.is_empty()
								}
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
			super::push(out, 51799);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80204);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 80205);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103278);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 119370);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80206);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103279);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119371);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129485);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51800);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80207);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119372);
									s.is_empty()
								}
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
			super::push(out, 51801);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80208);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80209);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 103282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 51802);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 80210);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103283);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119373);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103284);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119374);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -плица.
pub(crate) fn n24838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24683);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51803);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 51804);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80212);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рлица.
pub(crate) fn n24839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24684);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51805);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80213);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 80214);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 51806);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 80215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 51807);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80216);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 80217);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -слица.
pub(crate) fn n24840(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24685);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 51808);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 51809);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 51810);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 80220);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103286);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119375);
									s.is_empty()
								}
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
			super::push(out, 51811);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80221);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103287);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тлица.
pub(crate) fn n24841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24686);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 51812);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80222);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 80223);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 80224);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -улица.
pub(crate) fn n24842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24687);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 51813);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 51814);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шлица.
pub(crate) fn n24843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24688);
	s.is_empty()
}

/// Words ending -ылица.
pub(crate) fn n24844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24689);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 51815);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80226);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 103288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80227);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 51816);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80228);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ялица.
pub(crate) fn n24845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24690);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 51817);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 51818);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80230);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103291);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119376);
									s.is_empty()
								}
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
			super::push(out, 51819);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -первозимица.
pub(crate) fn n136915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135509);
	s.is_empty()
}

/// Words ending -лобница.
pub(crate) fn n81091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 103310);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 119390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 129491);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гробница.
pub(crate) fn n104401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103311);
	s.is_empty()
}

/// Words ending -дробница.
pub(crate) fn n104402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103312);
	s.is_empty()
}

/// Words ending -собница.
pub(crate) fn n81093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80257);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 103313);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 119391);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 129492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 103314);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 119392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рбница.
pub(crate) fn n52362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51839);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 80258);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 103315);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 119393);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -убница.
pub(crate) fn n52363(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51840);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 80259);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 103316);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 119394);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 129493);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 135510);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 80260);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 103317);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыбница.
pub(crate) fn n52364(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51841);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 80261);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -вница.
pub(crate) fn n24854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24699);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51842);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80262);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103318);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119395);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129494);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135511);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80263);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 103319);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80264);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103320);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 51843);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 80265);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103321);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 103322);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119397);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129495);
											s.is_empty()
										}
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
					super::push(out, 80266);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 80267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103323);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 80268);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103324);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51844);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 80269);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103325);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119398);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80270);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103326);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 103327);
							match s {
								[] => true,
								['р', 'к', '\u{1}', s @ ..] => {
									super::push(out, 119399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80271);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103328);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119400);
									s.is_empty()
								}
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
			super::push(out, 51845);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80272);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103329);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119401);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 129496);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 103330);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 80273);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103331);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 80274);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119404);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129497);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 119405);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129498);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 103332);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119406);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129499);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103333);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119407);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129500);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 103334);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103335);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119409);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 129501);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 103336);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119410);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 129502);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135512);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80275);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103337);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119411);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103338);
							match s {
								[] => true,
								['г', s @ ..] => super::t50::n120674(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t50::n81112(s, out),
				['н', s @ ..] => super::t50::n81113(s, out),
				['р', s @ ..] => super::t50::n81114(s, out),
				['х', s @ ..] => super::t50::n81115(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t50::n52369(s, out),
		['ы', s @ ..] => super::t50::n52370(s, out),
		_ => false,
	}
}

/// Words ending -гница.
pub(crate) fn n24855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24700);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51848);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80282);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дница.
pub(crate) fn n24856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24701);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51849);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 80283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80284);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103349);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 119421);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129511);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135516);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t50::n140430(s, out),
														_ => false,
													}
												}
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
							super::push(out, 103350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80285);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103351);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119422);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129512);
											s.is_empty()
										}
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
					super::push(out, 80286);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 80287);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103352);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 103353);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119423);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129513);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135517);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51850);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80288);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103354);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119424);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 103355);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119425);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 80289);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103356);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119426);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129514);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103357);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80290);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103358);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80291);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103359);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119429);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129515);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135518);
													s.is_empty()
												}
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
							super::push(out, 103360);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119430);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 129516);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 135519);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 119431);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 119432);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129517);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135520);
													match s {
														[] => true,
														['н', s @ ..] => super::t50::n140431(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80292);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103361);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 51851);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80293);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51852);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80294);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 103362);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119433);
									s.is_empty()
								}
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
			super::push(out, 51853);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80295);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103363);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119434);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129518);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 135521);
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', s @ ..] => {
											super::push(out, 129519);
											match s {
												[] => true,
												['т', s @ ..] => super::t50::n136928(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => super::t50::n120697(s, out),
								_ => false,
							}
						}
						['с', s @ ..] => super::t50::n104455(s, out),
						_ => false,
					}
				}
				['г', s @ ..] => super::t50::n81132(s, out),
				['п', s @ ..] => super::t50::n81133(s, out),
				['р', s @ ..] => super::t50::n81134(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t50::n52377(s, out),
		['ы', s @ ..] => super::t50::n52378(s, out),
		['я', s @ ..] => super::t50::n52379(s, out),
		_ => false,
	}
}

/// Words ending -еница.
pub(crate) fn n24857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24702);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 51857);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 80304);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 103373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 51858);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80305);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103374);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 80306);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103375);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 51859);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 80307);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103376);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119441);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129522);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135523);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80308);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103377);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119442);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 119443);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 129523);
											s.is_empty()
										}
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
					super::push(out, 80309);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103378);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 80310);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103379);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119445);
									s.is_empty()
								}
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
			super::push(out, 51860);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 51861);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80311);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103380);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80312);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103381);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 80313);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103382);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119448);
									s.is_empty()
								}
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
			super::push(out, 51862);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80314);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103383);
							match s {
								[] => true,
								['р', 'с', 'с', s @ ..] => {
									super::push(out, 119449);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129524);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135524);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80315);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 103384);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 80316);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103385);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80317);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103386);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119452);
									s.is_empty()
								}
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
			super::push(out, 51863);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80318);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103387);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119453);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 103388);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119454);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 129525);
											match s {
												[] => true,
												['ж', s @ ..] => {
													super::push(out, 135525);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51864);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80319);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103389);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 103390);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119456);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 129526);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51865);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80320);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103391);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 103392);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119457);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103393);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119458);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129527);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 103394);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119459);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129528);
											s.is_empty()
										}
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
					super::push(out, 80321);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103395);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119460);
									match s {
										[] => true,
										['с', s @ ..] => super::t50::n130881(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => super::t50::n52389(s, out),
		_ => false,
	}
}

/// Words ending -жница.
pub(crate) fn n24858(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24703);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51867);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 80324);
					match s {
						[] => true,
						['у', 'б', '\u{1}', s @ ..] => {
							super::push(out, 103397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 80325);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103398);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119461);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129530);
											s.is_empty()
										}
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
					super::push(out, 80326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 51868);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80327);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103399);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119462);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129531);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 103400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 80328);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103401);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119463);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80329);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 103402);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 119464);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129532);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135526);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51869);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 80331);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 51870);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80332);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 103403);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119465);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 119466);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 129533);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51871);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103404);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 103405);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 80334);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 103406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80335);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103407);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 103408);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 103409);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 119468);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129534);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80336);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103411);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 103412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 51872);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80337);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 103413);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119469);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129535);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51873);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80338);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103414);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119470);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129536);
											s.is_empty()
										}
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
					super::push(out, 80339);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103415);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119471);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129537);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103416);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 80340);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 80341);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 51874);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 51875);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зница.
pub(crate) fn n24859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24704);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51876);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80343);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103417);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119472);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 80344);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103418);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119473);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 129538);
											s.is_empty()
										}
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
					super::push(out, 80345);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 103419);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119474);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 119475);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129539);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135527);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103420);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80347);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103421);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 103422);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119476);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129540);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 129541);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135528);
													match s {
														[] => true,
														['б', s @ ..] => super::t50::n140433(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51877);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80348);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 103423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80349);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103424);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 119477);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129542);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103425);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119478);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 103426);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119479);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80350);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103427);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129543);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 129544);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135529);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51878);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80351);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103428);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 103429);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119480);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129545);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51879);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80352);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103430);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 103431);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 119481);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129546);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103432);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 119482);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129547);
											s.is_empty()
										}
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
					super::push(out, 80353);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 80354);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 103433);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 119483);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80355);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 80356);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 80357);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 80358);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103435);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119484);
									s.is_empty()
								}
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
			super::push(out, 51880);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80359);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103436);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119485);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129548);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51881);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80360);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t50::n81197(s, out),
				['л', s @ ..] => super::t50::n81198(s, out),
				['я', s @ ..] => super::t50::n81199(s, out),
				_ => false,
			}
		}
		['ю', s @ ..] => super::t50::n52405(s, out),
		_ => false,
	}
}

/// Words ending -иница.
pub(crate) fn n24860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24705);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 51883);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 51884);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80366);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103442);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 119488);
									s.is_empty()
								}
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
			super::push(out, 51885);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 51886);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 80368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 51887);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80369);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 80370);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103444);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 119489);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 129551);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 135530);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140434(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йница.
pub(crate) fn n24861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24706);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51888);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 80371);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 80372);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103445);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119490);
									s.is_empty()
								}
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
			super::push(out, 51889);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80373);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 80374);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103446);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119491);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129552);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51890);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103447);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 103448);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119492);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129553);
											s.is_empty()
										}
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
					super::push(out, 80376);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103449);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 103450);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 80377);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103451);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119494);
									s.is_empty()
								}
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
			super::push(out, 51891);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80378);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 103452);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кница.
pub(crate) fn n24862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24707);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 51892);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 51893);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80379);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 103453);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 119496);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129554);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 51894);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80380);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103454);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 119497);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 129555);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лница.
pub(crate) fn n24863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24708);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 51895);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 80381);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мница.
pub(crate) fn n24864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24709);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51896);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80382);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103456);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103457);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119499);
									s.is_empty()
								}
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
			super::push(out, 51897);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80383);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 51898);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 80384);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', 'о', 'н', 'н', 'а', 'р', 'т', 'с', '\u{1}', s @ ..] => {
					super::push(out, 80385);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 80386);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103460);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 119500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 80387);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 51899);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 80388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 51900);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 80389);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103462);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119501);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 129556);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135531);
													match s {
														[] => true,
														['ц', s @ ..] => super::t50::n140435(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нница.
pub(crate) fn n24865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24710);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51901);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 80390);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103463);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119502);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 129557);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 103464);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119503);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 80391);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103465);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119504);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129558);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135532);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80392);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103466);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119505);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 80393);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 103467);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119506);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80394);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 103468);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119507);
									match s {
										[] => true,
										['ю', s @ ..] => {
											super::push(out, 129559);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 135533);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80395);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 103469);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 103470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80396);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103471);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119508);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129560);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 103472);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119509);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129561);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135534);
													match s {
														[] => true,
														['в', s @ ..] => super::t50::n140436(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51902);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80397);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103473);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119510);
									match s {
										[] => true,
										['х', s @ ..] => {
											super::push(out, 129562);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135535);
													match s {
														[] => true,
														['е', s @ ..] => super::t50::n140437(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t50::n81234(s, out),
				['д', s @ ..] => {
					super::push(out, 80399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103476);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 103477);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119513);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129568);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 103478);
							match s {
								[] => true,
								['л', s @ ..] => super::t50::n120777(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 80400);
					s.is_empty()
				}
				['л', s @ ..] => super::t50::n81237(s, out),
				['м', s @ ..] => super::t50::n81238(s, out),
				['п', s @ ..] => super::t50::n81239(s, out),
				['р', s @ ..] => super::t50::n81240(s, out),
				['с', s @ ..] => super::t50::n81241(s, out),
				['ш', s @ ..] => super::t50::n81242(s, out),
				['щ', s @ ..] => super::t50::n81243(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t50::n52426(s, out),
		['о', s @ ..] => super::t50::n52427(s, out),
		['у', s @ ..] => super::t50::n52428(s, out),
		['ы', s @ ..] => super::t50::n52429(s, out),
		['я', s @ ..] => super::t50::n52430(s, out),
		_ => false,
	}
}

/// Words ending -оница.
pub(crate) fn n24866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24711);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 51908);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80430);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103524);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 51909);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80431);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 80432);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 103526);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 119549);
									s.is_empty()
								}
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
			super::push(out, 51910);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 80433);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 51911);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 80434);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 103528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пница.
pub(crate) fn n24867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24712);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 51912);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 51913);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 80435);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 80436);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103529);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 119550);
									s.is_empty()
								}
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
			super::push(out, 51914);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 80437);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80438);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103531);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119551);
									match s {
										[] => true,
										['з', '-', s @ ..] => {
											super::push(out, 129592);
											match s {
												[] => true,
												['ь', s @ ..] => {
													super::push(out, 135545);
													match s {
														[] => true,
														['т', s @ ..] => super::t50::n140441(s, out),
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
									super::push(out, 119552);
									s.is_empty()
								}
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
			super::push(out, 51915);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 80439);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рница.
pub(crate) fn n24868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24713);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51916);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80440);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103533);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119553);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129593);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135546);
													s.is_empty()
												}
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
							super::push(out, 103534);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119554);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129594);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135547);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80441);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103535);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119555);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 80442);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 103536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 80443);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103537);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119556);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 80444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 80445);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103539);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 119557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 103540);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119558);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80446);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103541);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119559);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 80447);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103542);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 103543);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119561);
									s.is_empty()
								}
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
			super::push(out, 51917);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 80448);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103544);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119562);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129595);
											s.is_empty()
										}
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
					super::push(out, 80449);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103545);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119563);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 80450);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103546);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119564);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 80451);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 103547);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119565);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129596);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 135548);
													match s {
														[] => true,
														['ч', s @ ..] => super::t50::n140442(s, out),
														_ => false,
													}
												}
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
							super::push(out, 103548);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119566);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 129597);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 135549);
													match s {
														[] => true,
														['п', s @ ..] => super::t50::n140443(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80452);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 103549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 80453);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103550);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119567);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129598);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 135550);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80454);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103551);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 80455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103553);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 103554);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119569);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 80457);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103555);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 103556);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 119570);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t50::n52441(s, out),
		['о', s @ ..] => super::t50::n52442(s, out),
		['у', s @ ..] => super::t50::n52443(s, out),
		['ы', s @ ..] => super::t50::n52444(s, out),
		['ю', s @ ..] => super::t50::n52445(s, out),
		['я', s @ ..] => super::t50::n52446(s, out),
		_ => false,
	}
}

/// Words ending -сница.
pub(crate) fn n24869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24714);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51924);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80474);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103576);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119586);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 80475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 51925);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80476);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103578);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 103579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103580);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 103581);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119587);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80477);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103582);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80478);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 80479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 51926);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 80480);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80481);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103586);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 119589);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129608);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 135551);
													match s {
														[] => true,
														['л', s @ ..] => super::t50::n140444(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80482);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103587);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119590);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 129609);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 135552);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51927);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 80483);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 51928);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80484);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 103589);
							match s {
								[] => true,
								['к', 'х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 119591);
									s.is_empty()
								}
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
			super::push(out, 51929);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80485);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 103590);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 119592);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 129610);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 135553);
													match s {
														[] => true,
														['о', s @ ..] => super::t50::n140445(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80486);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103591);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119593);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 129611);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 135554);
													s.is_empty()
												}
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
							super::push(out, 103592);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 119594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103593);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 119595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80487);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 103594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80488);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 103595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 80489);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 103596);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 119596);
									s.is_empty()
								}
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
			super::push(out, 51930);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 80490);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 80491);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 103598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 80492);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103599);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 119597);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 103600);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тница.
pub(crate) fn n24870(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24715);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 51931);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 80493);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 80494);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103602);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 119599);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 129612);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 119600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 80495);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 103603);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 103604);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119601);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 129613);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135555);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 80496);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 103605);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 119602);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129614);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 135556);
													match s {
														[] => true,
														['с', s @ ..] => super::t50::n140446(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 80497);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 103606);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 119603);
									match s {
										[] => true,
										['а', 'р', '-', s @ ..] => {
											super::push(out, 129615);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135557);
													match s {
														[] => true,
														['ц', s @ ..] => super::t50::n140447(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 51932);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 80498);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 103607);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 119604);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 129616);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 135558);
													s.is_empty()
												}
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
							super::push(out, 103608);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119605);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 80499);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 103609);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 119606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 103610);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 119607);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 129617);
											s.is_empty()
										}
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
						['а', s @ ..] => {
							super::push(out, 103611);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 119608);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129618);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 135559);
													match s {
														[] => true,
														['т', s @ ..] => super::t50::n140448(s, out),
														_ => false,
													}
												}
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
							super::push(out, 103612);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 119609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 103613);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 119610);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 129619);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 103614);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 119611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 80500);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 103615);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 119612);
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
						['а', s @ ..] => {
							super::push(out, 103616);
							match s {
								[] => true,
								['г', s @ ..] => super::t50::n120876(s, out),
								_ => false,
							}
						}
						['к', s @ ..] => super::t50::n104708(s, out),
						_ => false,
					}
				}
				['ф', s @ ..] => super::t50::n81342(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t50::n52456(s, out),
		['к', s @ ..] => super::t50::n52457(s, out),
		['о', s @ ..] => super::t50::n52458(s, out),
		['р', s @ ..] => super::t50::n52459(s, out),
		['с', s @ ..] => super::t50::n52460(s, out),
		['у', s @ ..] => super::t50::n52461(s, out),
		['ы', s @ ..] => super::t50::n52462(s, out),
		['я', s @ ..] => super::t50::n52463(s, out),
		_ => false,
	}
}

/// Words ending -уница.
pub(crate) fn n24871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24716);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 51941);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 80533);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 103657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 80534);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 103658);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 119645);
									s.is_empty()
								}
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
			super::push(out, 51942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цница.
pub(crate) fn n24872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24717);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 51943);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 80535);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 103659);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

