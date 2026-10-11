use flexion::Path;

/// Words ending -нничать.
pub(crate) fn n7744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6537);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13894);
			match s {
				[] => true,
				['г', 'а', 'л', 'а', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'й', 'а', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', 'т', 'р', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'е', 'ж', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 22326);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 30424);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 36147);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 30425);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39687);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 41605);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n86776(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 13895);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22327);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30426);
							match s {
								[] => true,
								['р', 'к', 'т', 'о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39688);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 39689);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22328);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30427);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 22329);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 30428);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 36148);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 39690);
											match s {
												[] => true,
												['е', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 36149);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 39691);
											match s {
												[] => true,
												['о', 'к', 'г', 'е', 'л', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22330);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30429);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22331);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30430);
							match s {
								[] => true,
								['т', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22332);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 30431);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 36150);
									match s {
										[] => true,
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 30432);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 36151);
									match s {
										[] => true,
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 30433);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 36152);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22333);
					match s {
						[] => true,
						['о', 'м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36153);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36154);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 39692);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 36155);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 39693);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36156);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39694);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 36157);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t26::n18595(s, out),
		['о', s @ ..] => super::t26::n18596(s, out),
		['у', s @ ..] => super::t26::n18597(s, out),
		['ы', s @ ..] => super::t26::n18598(s, out),
		['я', s @ ..] => super::t26::n18599(s, out),
		_ => false,
	}
}

/// Words ending -рничать.
pub(crate) fn n7745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6538);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13901);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 22338);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 36160);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22339);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30437);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 36161);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', 'а', 'б', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 22340);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 36162);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39696);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'т', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'е', 'л', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'с', 'у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['х', 'у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 22341);
					match s {
						[] => true,
						['н', 'о', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13902);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 22342);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 30438);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36163);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 39697);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22343);
					match s {
						[] => true,
						['л', 'а', 'ш', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22344);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30439);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 36164);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22345);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30440);
							match s {
								[] => true,
								['в', 'и', 'ж', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'а', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 22346);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 30441);
							match s {
								[] => true,
								['а', 'м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 30442);
							match s {
								[] => true,
								['ш', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22347);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30443);
							match s {
								[] => true,
								['ц', 'и', 'л', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 39698);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 30444);
							match s {
								[] => true,
								['к', 'о', 'с', 'ы', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 30445);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 36165);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39699);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22348);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30446);
							match s {
								[] => true,
								['м', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 22349);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30447);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 22350);
					match s {
						[] => true,
						['о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36166);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39700);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t26::n33991(s, out),
				['ш', s @ ..] => {
					super::push(out, 22352);
					match s {
						[] => true,
						['у', 'к', 'а', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 22353);
					match s {
						[] => true,
						['с', s @ ..] => super::t26::n51077(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t26::n18602(s, out),
		['о', s @ ..] => super::t26::n18603(s, out),
		['у', s @ ..] => super::t26::n18604(s, out),
		['ы', s @ ..] => super::t26::n18605(s, out),
		['ю', 'т', 'н', 'а', 'в', 'а', '\u{1}', s @ ..] => super::t26::n18606(s, out),
		['я', s @ ..] => super::t26::n18607(s, out),
		_ => false,
	}
}

/// Words ending -сничать.
pub(crate) fn n7746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6539);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13908);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22360);
					match s {
						[] => true,
						['е', 'в', 'о', 'л', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22361);
					match s {
						[] => true,
						['и', 'р', 'г', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36174);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36175);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36176);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 36177);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22362);
					match s {
						[] => true,
						['у', 'р', 'к', 'ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13909);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22363);
					match s {
						[] => true,
						['л', 'а', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 22364);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 22365);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 30459);
							match s {
								[] => true,
								['к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 22366);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22367);
					match s {
						[] => true,
						['е', 'т', 'н', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13910);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22368);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30461);
							match s {
								[] => true,
								['н', 'е', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'о', 'г', 'д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', 'е', 'у', 'х', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13911);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 22369);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30462);
							match s {
								[] => true,
								['ф', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39704);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39705);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 41606);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n86777(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 39706);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 30463);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 36178);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22370);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 13912);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22371);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 36179);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22372);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'б', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'т', 'о', 'б', 'о', 'л', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тничать.
pub(crate) fn n7747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6540);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13913);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22373);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30464);
							match s {
								[] => true,
								['р', 'к', 'а', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 22374);
					match s {
						[] => true,
						['е', 'в', 'д', 'е', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 22375);
					match s {
						[] => true,
						['г', 'а', 'л', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 30465);
							match s {
								[] => true,
								['л', 'е', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39707);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 41607);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n86778(s, out),
														_ => false,
													}
												}
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
								['в', s @ ..] => {
									super::push(out, 36180);
									match s {
										[] => true,
										['д', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 36181);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 39708);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22376);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 30466);
							match s {
								[] => true,
								['х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 22377);
					match s {
						[] => true,
						['о', 'л', 'п', 'и', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30467);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 30468);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 36182);
									match s {
										[] => true,
										['т', 'с', 'и', 'р', 'а', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13914);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22378);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30469);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 36183);
									match s {
										[] => true,
										['х', 'а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22379);
					match s {
						[] => true,
						['о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36184);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36185);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 39709);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36186);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39710);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22380);
					match s {
						[] => true,
						['п', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36187);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36188);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 39711);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 22381);
					match s {
						[] => true,
						['к', 'е', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36189);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36190);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36191);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 13915);
			match s {
				[] => true,
				['б', 'о', 'т', 'с', 'р', 'е', 'ш', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', 'а', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'а', 'б', 'и', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', 'з', 'е', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => super::t26::n18616(s, out),
		['о', s @ ..] => super::t26::n18617(s, out),
		['р', s @ ..] => super::t26::n18618(s, out),
		['с', s @ ..] => super::t26::n18619(s, out),
		['у', s @ ..] => super::t26::n18620(s, out),
		['ы', s @ ..] => super::t26::n18621(s, out),
		['я', s @ ..] => super::t26::n18622(s, out),
		_ => false,
	}
}

/// Words ending -уничать.
pub(crate) fn n7748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6541);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 13923);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 22399);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чничать.
pub(crate) fn n7749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6542);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', 'а', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', 'у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 22400);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 30484);
							match s {
								[] => true,
								['а', 'б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 13924);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 22401);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 30485);
							match s {
								[] => true,
								['о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 22402);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30486);
							match s {
								[] => true,
								['у', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 22403);
					match s {
						[] => true,
						['р', 'о', 'г', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13925);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['о', 'н', 'и', 'д', 'е', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', 'в', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'я', 'р', 'т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['щ', 'м', 'я', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', 'а', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['л', 'к', 'с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 22404);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 30487);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 36201);
									match s {
										[] => true,
										['ч', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 30488);
							match s {
								[] => true,
								['л', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', 'у', 'к', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 22405);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 30489);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 36202);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', 'е', 'м', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13926);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 30490);
							match s {
								[] => true,
								['о', 'п', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 30491);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 36203);
									match s {
										[] => true,
										['е', 'б', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 22406);
					match s {
						[] => true,
						['я', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'л', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22407);
					match s {
						[] => true,
						['а', 'г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 36204);
									match s {
										[] => true,
										['м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36205);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 39715);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', 'р', 'к', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шничать.
pub(crate) fn n7750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6543);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13927);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 22408);
					match s {
						[] => true,
						['а', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36206);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39716);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22409);
					match s {
						[] => true,
						['п', 'о', 'т', 'р', 'е', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 22410);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 30492);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13928);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22411);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 36207);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 41612);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n86783(s, out),
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
									super::push(out, 36208);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 39717);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22412);
					match s {
						[] => true,
						['с', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 22413);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13929);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 22414);
					match s {
						[] => true,
						['ь', 'л', 'а', 'м', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13930);
			match s {
				[] => true,
				['б', 'е', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'й', 'о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'с', 'о', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', 'у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 13931);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22415);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 30493);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36209);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39718);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22416);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 30494);
							match s {
								[] => true,
								['о', 'в', 'д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 36210);
									match s {
										[] => true,
										['и', 'р', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 36211);
									match s {
										[] => true,
										['и', 'л', 'е', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 41613);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t26::n86784(s, out),
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
									super::push(out, 36212);
									match s {
										[] => true,
										['а', 'м', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', s @ ..] => super::t26::n83258(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['м', s @ ..] => super::t26::n65726(s, out),
								['н', s @ ..] => super::t26::n65727(s, out),
								['т', s @ ..] => super::t26::n65728(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => super::t26::n34095(s, out),
				['к', s @ ..] => super::t26::n34096(s, out),
				['л', s @ ..] => super::t26::n34097(s, out),
				['м', s @ ..] => super::t26::n34098(s, out),
				['р', s @ ..] => super::t26::n34099(s, out),
				['х', s @ ..] => super::t26::n34100(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t26::n18637(s, out),
		['ю', s @ ..] => super::t26::n18638(s, out),
		_ => false,
	}
}

/// Words ending -щничать.
pub(crate) fn n7751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6544);
	match s {
		[] => true,
		['и', 'х', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 22426);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 30501);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'у', 'л', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьничать.
pub(crate) fn n7752(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6545);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 22427);
					match s {
						[] => true,
						['б', 'а', 'х', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', 'х', 'о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 30502);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 36223);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 39724);
											match s {
												[] => true,
												['б', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 30503);
							match s {
								[] => true,
								['н', s @ ..] => {
									match s {
										[] => true,
										['а', 'к', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'д', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 30504);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 36224);
									match s {
										[] => true,
										['е', 'г', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 36225);
									match s {
										[] => true,
										['и', 'ц', 'н', 'и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ц', s @ ..] => super::t26::n65745(s, out),
								_ => false,
							}
						}
						['к', s @ ..] => super::t26::n51189(s, out),
						['н', s @ ..] => super::t26::n51190(s, out),
						['р', s @ ..] => super::t26::n51191(s, out),
						['с', '\u{1}', s @ ..] => super::t26::n51192(s, out),
						['т', s @ ..] => super::t26::n51193(s, out),
						['у', s @ ..] => super::t26::n51194(s, out),
						['х', s @ ..] => super::t26::n51195(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t26::n34107(s, out),
				['и', s @ ..] => super::t26::n34108(s, out),
				['о', s @ ..] => super::t26::n34109(s, out),
				['у', s @ ..] => super::t26::n34110(s, out),
				['ы', 'т', 'у', 'б', 'о', 'с', '\u{1}', s @ ..] => super::t26::n34111(s, out),
				_ => false,
			}
		}
		['с', 'о', 'в', 'а', '\u{1}', s @ ..] => super::t26::n18642(s, out),
		_ => false,
	}
}

/// Words ending -муравить.
pub(crate) fn n20064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14692);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 22803);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 22804);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 30638);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 22805);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 30639);
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 22806);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 30640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 22807);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 30641);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word самовоспроизводить.
pub(crate) fn n86800(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word заколобродить.
pub(crate) fn n83299(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word наколобродить.
pub(crate) fn n83300(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поколобродить.
pub(crate) fn n83301(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -насумасбродить.
pub(crate) fn n83302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41627);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word обуглеродить.
pub(crate) fn n83303(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -изродить.
pub(crate) fn n20426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14924);
	match s {
		[] => true,
		['о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -озродить.
pub(crate) fn n20427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14925);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иродить.
pub(crate) fn n9192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7566);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 14926);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 22998);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ородить.
pub(crate) fn n9193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7567);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14927);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 22999);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 30741);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 14928);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 23000);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 23001);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 30742);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 30743);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 36291);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39761);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39762);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 30744);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 23002);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 23003);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 23004);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 23005);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 23006);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 23007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 23008);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 23009);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 23010);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 30745);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 14929);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23011);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 30746);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 14930);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 23012);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 30747);
							match s {
								[] => true,
								['к', 'с', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 14931);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тродить.
pub(crate) fn n9194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7568);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 14932);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уродить.
pub(crate) fn n9195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7569);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыродить.
pub(crate) fn n9196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7570);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 14933);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word заблагорассудить.
pub(crate) fn n83305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41628);
	s.is_empty()
}

/// The word пообморозить.
pub(crate) fn n83317(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поскандалить.
pub(crate) fn n83318(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word просигналить.
pub(crate) fn n83319(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word осовременить.
pub(crate) fn n83321(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word наодеколонить.
pub(crate) fn n83324(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыронить.
pub(crate) fn n10260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8506);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15900);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word полицемерить.
pub(crate) fn n83325(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word температурить.
pub(crate) fn n83328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41640);
	s.is_empty()
}

/// The word затемпературить.
pub(crate) fn n83329(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word потемпературить.
pub(crate) fn n83330(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word облитературить.
pub(crate) fn n83331(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word олитературить.
pub(crate) fn n83332(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -отурить.
pub(crate) fn n10890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9036);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 16430);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 16431);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 23979);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word птурить.
pub(crate) fn n22749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16432);
	s.is_empty()
}

/// Words ending -аптурить.
pub(crate) fn n22750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16433);
	match s {
		[] => true,
		['з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ытурить.
pub(crate) fn n10892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9038);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16434);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтурить.
pub(crate) fn n10893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9039);
	match s {
		[] => true,
		['л', 'у', 'к', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -отплодоносить.
pub(crate) fn n83333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41641);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -относить.
pub(crate) fn n22931(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 24050);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 24051);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 31170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыносить.
pub(crate) fn n11030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9144);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 16536);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24052);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 24053);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 31171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -просить.
pub(crate) fn n11034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9148);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16545);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16546);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16547);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 24068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 24069);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 24070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 16548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 24071);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 24072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 24073);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 24074);
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 24075);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31180);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 16549);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 16550);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24076);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word заблаговестить.
pub(crate) fn n83334(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -азблаговестить.
pub(crate) fn n83335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41642);
	match s {
		[] => true,
		['р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word отблаговестить.
pub(crate) fn n83336(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -скрестить.
pub(crate) fn n38214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24262);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыкрестить.
pub(crate) fn n38215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24263);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31277);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тестить.
pub(crate) fn n11231(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9331);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16826);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16827);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 24264);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -честить.
pub(crate) fn n11232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9332);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16828);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 16829);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 24265);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36431);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36432);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word набухвостить.
pub(crate) fn n83337(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лостить.
pub(crate) fn n11244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9344);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16850);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 24281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31286);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 31287);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 31288);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 36446);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мостить.
pub(crate) fn n11245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9345);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16851);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16852);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 24282);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 31289);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['и', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 16853);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 16854);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24283);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -постить.
pub(crate) fn n11246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9346);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16855);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 16856);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 24284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 16857);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31290);
							s.is_empty()
						}
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 31291);
							match s {
								[] => true,
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ростить.
pub(crate) fn n11247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9347);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 16858);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 16859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 16860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24285);
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сстрочить.
pub(crate) fn n38589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24420);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 31333);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 36454);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 31334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отстрочить.
pub(crate) fn n38590(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыстрочить.
pub(crate) fn n38591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24421);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31335);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урочить.
pub(crate) fn n11457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9532);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17069);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 17070);
			match s {
				[] => true,
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 17071);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 24422);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 24423);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word замироточить.
pub(crate) fn n83340(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -шикнуть.
pub(crate) fn n12141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10022);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 17501);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыикнуть.
pub(crate) fn n12142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10023);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17502);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word попримолкнуть.
pub(crate) fn n83341(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ритолкнуть.
pub(crate) fn n53886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31449);
	match s {
		[] => true,
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word протолкнуть.
pub(crate) fn n39293(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -столкнуть.
pub(crate) fn n39294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24733);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31450);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 31451);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ттолкнуть.
pub(crate) fn n39295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24734);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 31452);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытолкнуть.
pub(crate) fn n39296(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24735);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 31453);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word горкнуть.
pub(crate) fn n39346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24762);
	s.is_empty()
}

/// Words ending -егоркнуть.
pub(crate) fn n39347(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24763);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31465);
			match s {
				[] => true,
				['е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огоркнуть.
pub(crate) fn n39348(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24764);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 31466);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -моркнуть.
pub(crate) fn n24551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17544);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 24765);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31467);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 31468);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 36487);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -торкнуть.
pub(crate) fn n24552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17545);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 24766);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 31469);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -форкнуть.
pub(crate) fn n24553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17546);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шоркнуть.
pub(crate) fn n24554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17547);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уркнуть.
pub(crate) fn n12166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10047);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 17548);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 17549);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыркнуть.
pub(crate) fn n12167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10048);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 17550);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 24767);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 31470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 24768);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 17551);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 17552);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 17553);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 24769);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 31471);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 17554);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юркнуть.
pub(crate) fn n12168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10049);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 17555);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17556);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 24770);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 17557);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24771);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 17558);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нырнуть.
pub(crate) fn n12297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10166);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', 'о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 17682);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ш', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 24855);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31508);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 17683);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 24856);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -быстнуть.
pub(crate) fn n24893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17742);
	match s {
		[] => true,
		['о', 'л', 'х', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31541);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 31542);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 36509);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лыстнуть.
pub(crate) fn n24894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17743);
	match s {
		[] => true,
		['х', s @ ..] => {
			super::push(out, 24913);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ястнуть.
pub(crate) fn n12355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10213);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 17744);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 24914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 17745);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 24915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word отсохнуть.
pub(crate) fn n24990(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -усохнуть.
pub(crate) fn n24991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17809);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ысохнуть.
pub(crate) fn n24992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17810);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 24957);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word допродаваться.
pub(crate) fn n83348(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word отдаваться.
pub(crate) fn n40361(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word удаваться.
pub(crate) fn n40362(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word выдаваться.
pub(crate) fn n40363(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -лаваться.
pub(crate) fn n25812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18123);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 25159);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31663);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 31664);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наваться.
pub(crate) fn n25813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18124);
	match s {
		[] => true,
		['з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 31665);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 31666);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 36547);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 31667);
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 36548);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', 'р', '\u{1}', s @ ..] => {
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
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 31668);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -таваться.
pub(crate) fn n25814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18125);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25160);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 31669);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 36549);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 25161);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 31670);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 36550);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 31671);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хаваться.
pub(crate) fn n25815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18126);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25162);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 31672);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еваться.
pub(crate) fn n13193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10596);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18127);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18128);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25163);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['л', 'в', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25164);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31673);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36551);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 39799);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 31674);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 36552);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 31675);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 31676);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 31677);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18129);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25166);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 31678);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18130);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25167);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25168);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 25169);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25170);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 31680);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39800);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39801);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => super::t26::n54579(s, out),
						['р', s @ ..] => super::t26::n54580(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => super::t26::n40391(s, out),
				['з', s @ ..] => super::t26::n40392(s, out),
				['м', s @ ..] => super::t26::n40393(s, out),
				['н', s @ ..] => super::t26::n40394(s, out),
				['о', s @ ..] => super::t26::n40395(s, out),
				['р', s @ ..] => super::t26::n40396(s, out),
				['с', s @ ..] => super::t26::n40397(s, out),
				['т', s @ ..] => super::t26::n40398(s, out),
				['у', s @ ..] => super::t26::n40399(s, out),
				['ь', s @ ..] => super::t26::n40400(s, out),
				_ => false,
			}
		}
		['з', s @ ..] => super::t26::n25820(s, out),
		['л', s @ ..] => super::t26::n25821(s, out),
		['м', s @ ..] => super::t26::n25822(s, out),
		['н', s @ ..] => super::t26::n25823(s, out),
		['о', s @ ..] => super::t26::n25824(s, out),
		['п', s @ ..] => super::t26::n25825(s, out),
		['р', s @ ..] => super::t26::n25826(s, out),
		['с', s @ ..] => super::t26::n25827(s, out),
		['т', s @ ..] => super::t26::n25828(s, out),
		['ц', s @ ..] => super::t26::n25829(s, out),
		['ч', s @ ..] => super::t26::n25830(s, out),
		['ш', s @ ..] => super::t26::n25831(s, out),
		['щ', s @ ..] => super::t26::n25832(s, out),
		_ => false,
	}
}

/// Words ending -зваться.
pub(crate) fn n13194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10597);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18144);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18145);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 25220);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 31739);
							s.is_empty()
						}
						['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18146);
			match s {
				[] => true,
				['о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18147);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18148);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25221);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31740);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25223);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25224);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25225);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25226);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 31741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36591);
									s.is_empty()
								}
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
			super::push(out, 18149);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иваться.
pub(crate) fn n13195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10598);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18150);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', 'д', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', 'а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => super::t26::n40483(s, out),
				['р', s @ ..] => super::t26::n40484(s, out),
				['т', s @ ..] => super::t26::n40485(s, out),
				['х', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t26::n25840(s, out),
		['в', s @ ..] => super::t26::n25841(s, out),
		['г', s @ ..] => super::t26::n25842(s, out),
		['е', s @ ..] => super::t26::n25843(s, out),
		['ж', s @ ..] => super::t26::n25844(s, out),
		['з', s @ ..] => super::t26::n25845(s, out),
		['к', s @ ..] => super::t26::n25846(s, out),
		['л', s @ ..] => super::t26::n25847(s, out),
		['н', s @ ..] => super::t26::n25848(s, out),
		['о', s @ ..] => super::t26::n25849(s, out),
		['п', s @ ..] => super::t26::n25850(s, out),
		['р', s @ ..] => super::t26::n25851(s, out),
		['с', 'е', 'л', 'з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', 'ч', s @ ..] => super::t26::n25853(s, out),
		['х', s @ ..] => super::t26::n25854(s, out),
		['ч', s @ ..] => super::t26::n25855(s, out),
		['ш', s @ ..] => super::t26::n25856(s, out),
		['щ', s @ ..] => super::t26::n25857(s, out),
		_ => false,
	}
}

/// Words ending -оваться.
pub(crate) fn n13196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10599);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18165);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 25289);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31853);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39867);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 39868);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 39869);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['и', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25290);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 31854);
							match s {
								[] => true,
								['р', 'т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39870);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['у', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25291);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 31855);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39871);
											s.is_empty()
										}
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25292);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 31856);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 39872);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 39873);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25293);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 31857);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36661);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 36662);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36663);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t26::n25859(s, out),
		['г', s @ ..] => super::t26::n25860(s, out),
		['д', s @ ..] => super::t26::n25861(s, out),
		['з', s @ ..] => super::t26::n25862(s, out),
		['к', s @ ..] => super::t26::n25863(s, out),
		['л', s @ ..] => super::t26::n25864(s, out),
		['м', s @ ..] => {
			super::push(out, 18172);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25338);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 31920);
							match s {
								[] => true,
								['ф', 'н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 36759);
									match s {
										[] => true,
										['ш', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ф', s @ ..] => super::t26::n68522(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 31921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', 'и', 'р', s @ ..] => super::t26::n40714(s, out),
				['ь', s @ ..] => {
					super::push(out, 25339);
					match s {
						[] => true,
						['л', 'е', 'ш', '\u{1}', s @ ..] => super::t26::n55427(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t26::n25866(s, out),
		['п', s @ ..] => super::t26::n25867(s, out),
		['р', s @ ..] => super::t26::n25868(s, out),
		['с', s @ ..] => super::t26::n25869(s, out),
		['т', s @ ..] => super::t26::n25870(s, out),
		['ф', s @ ..] => super::t26::n25871(s, out),
		['х', s @ ..] => super::t26::n25872(s, out),
		['ц', s @ ..] => super::t26::n25873(s, out),
		_ => false,
	}
}

/// Words ending -рваться.
pub(crate) fn n13197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10600);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18181);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18182);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					super::push(out, 25394);
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					super::push(out, 25395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18183);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 32018);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', '\u{1}', s @ ..] => {
							super::push(out, 32019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18184);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25396);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36983);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36984);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40294);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42049);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					super::push(out, 25397);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 25398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32020);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							super::push(out, 32021);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 32022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25399);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 32023);
							s.is_empty()
						}
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36985);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36986);
									match s {
										[] => true,
										['м', 'а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 32024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', '\u{1}', s @ ..] => {
					super::push(out, 25400);
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					super::push(out, 25401);
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 25402);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25403);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 36987);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 36988);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40295);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42050);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18185);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 18186);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уваться.
pub(crate) fn n13198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10601);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18187);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ываться.
pub(crate) fn n13199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10602);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18188);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25405);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32025);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 36989);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['е', s @ ..] => {
									super::push(out, 36990);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 40296);
											match s {
												[] => true,
												['е', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 36991);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40297);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25406);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 32026);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 32027);
							match s {
								[] => true,
								['х', s @ ..] => {
									match s {
										[] => true,
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', 'р', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ы', 'в', '\u{1}', s @ ..] => {
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
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32028);
							match s {
								[] => true,
								['к', 'с', s @ ..] => {
									match s {
										[] => true,
										['а', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['ы', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'с', 'ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ъ', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 32029);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40298);
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25407);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32030);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 25408);
					match s {
						[] => true,
						['а', 'д', s @ ..] => super::t26::n55592(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t26::n40803(s, out),
				['с', s @ ..] => {
					super::push(out, 25410);
					s.is_empty()
				}
				['т', s @ ..] => super::t26::n40805(s, out),
				['я', 'л', 'х', 'с', 'а', 'р', '\u{1}', s @ ..] => super::t26::n40806(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t27::n25883(s, out),
		['д', s @ ..] => super::t27::n25884(s, out),
		['з', s @ ..] => super::t27::n25885(s, out),
		['л', s @ ..] => super::t27::n25886(s, out),
		['м', s @ ..] => super::t27::n25887(s, out),
		['н', s @ ..] => super::t27::n25888(s, out),
		['п', s @ ..] => super::t27::n25889(s, out),
		['р', s @ ..] => super::t27::n25890(s, out),
		['с', s @ ..] => super::t27::n25891(s, out),
		['т', s @ ..] => super::t27::n25892(s, out),
		_ => false,
	}
}

/// Words ending -регаться.
pub(crate) fn n25897(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18203);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25472);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25473);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', 'е', 'н', 'е', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 25474);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тегаться.
pub(crate) fn n25898(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18204);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 25475);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32093);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -згаться.
pub(crate) fn n13202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10605);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 18205);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25476);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 18206);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32094);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32095);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37013);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32096);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37014);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37015);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37016);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -игаться.
pub(crate) fn n13203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10606);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18207);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32097);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32098);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32099);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32100);
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 37017);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 32101);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'м', 'а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18208);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 18209);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25477);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32102);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18210);
			match s {
				[] => true,
				['т', 'с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18211);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лгаться.
pub(crate) fn n13204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10607);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18212);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -огаться.
pub(crate) fn n13205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10608);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 18213);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['н', 'з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'о', 'в', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18214);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25478);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32104);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32105);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37018);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32106);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37019);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37020);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 40309);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 40310);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37021);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 37022);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40311);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргаться.
pub(crate) fn n13206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10609);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18215);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25480);
					match s {
						[] => true,
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
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
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'и', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32107);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32108);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 32109);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32110);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37023);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32111);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32112);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 32113);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37024);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18216);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25482);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32114);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25483);
					match s {
						[] => true,
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40312);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42064);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n87887(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -угаться.
pub(crate) fn n13207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10610);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 18217);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25484);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25485);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 25486);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18218);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25487);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25488);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 25489);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25490);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25491);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37025);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 37026);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40313);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25492);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32115);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18219);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25493);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25494);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25495);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыгаться.
pub(crate) fn n13208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10611);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18220);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25496);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32116);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18221);
			match s {
				[] => true,
				['ш', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18222);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25497);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25498);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 32117);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25499);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32118);
							s.is_empty()
						}
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25500);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32119);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -югаться.
pub(crate) fn n13209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10612);
	match s {
		[] => true,
		['т', 'а', 'м', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25501);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 25502);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32120);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 25503);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32121);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ягаться.
pub(crate) fn n13210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10613);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18223);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25504);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25505);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25506);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18224);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
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
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 32122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ведаться.
pub(crate) fn n25932(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18235);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25512);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25513);
			match s {
				[] => true,
				['н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', 'а', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25514);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 37030);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 40314);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'и', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еедаться.
pub(crate) fn n25933(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18236);
	match s {
		[] => true,
		['р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -недаться.
pub(crate) fn n25935(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18238);
	match s {
		[] => true,
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оедаться.
pub(crate) fn n25936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18239);
	match s {
		[] => true,
		['д', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -редаться.
pub(crate) fn n25937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18240);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 25515);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 25516);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 32128);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37031);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 25517);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 32129);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -седаться.
pub(crate) fn n25938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18241);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 25518);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 32130);
					match s {
						[] => true,
						['о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', 'а', 'н', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 25519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 25520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уедаться.
pub(crate) fn n25939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18242);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ъедаться.
pub(crate) fn n25940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18243);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыедаться.
pub(crate) fn n25941(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18244);
	match s {
		[] => true,
		['в', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ждаться.
pub(crate) fn n13217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10620);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18245);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18246);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 25521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32131);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32132);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37032);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25522);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32133);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 25523);
					match s {
						[] => true,
						['к', 'о', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37033);
									match s {
										[] => true,
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 42065);
													match s {
														[] => true,
														['п', s @ ..] => super::t27::n87888(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', 'о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25524);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 25525);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['з', 'о', 'в', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25526);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18247);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25527);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
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
						['в', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32135);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37034);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', 'у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18248);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 25528);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37035);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 37036);
									match s {
										[] => true,
										['с', 'с', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25529);
					match s {
						[] => true,
						['г', 'и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25530);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t27::n41037(s, out),
				['р', s @ ..] => super::t27::n41038(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t27::n25946(s, out),
		['у', s @ ..] => super::t27::n25947(s, out),
		_ => false,
	}
}

/// Words ending -здаться.
pub(crate) fn n13218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10621);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18251);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25538);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 32142);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37042);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37043);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 37044);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 40315);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37045);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40316);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25539);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18252);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25540);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25541);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32144);
							match s {
								[] => true,
								['е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18253);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25543);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32146);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 32147);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 37046);
									match s {
										[] => true,
										['е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32148);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37047);
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18254);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 25544);
					match s {
						[] => true,
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -идаться.
pub(crate) fn n13219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10622);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18255);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25545);
					s.is_empty()
				}
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32149);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37048);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25546);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25547);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25548);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 18256);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 18257);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25549);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18258);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25550);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25551);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25552);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25553);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37049);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37050);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18259);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ндаться.
pub(crate) fn n13220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10623);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18260);
			match s {
				[] => true,
				['л', 'а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37051);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 32151);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одаться.
pub(crate) fn n13221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10624);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18261);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25555);
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25556);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 18262);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25557);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25558);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18263);
			match s {
				[] => true,
				['г', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', s @ ..] => {
									super::push(out, 37052);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40317);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', 'и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18264);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25559);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18265);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32152);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32153);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32154);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32155);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40318);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40319);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сдаться.
pub(crate) fn n13222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10625);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18266);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18267);
			match s {
				[] => true,
				['з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18268);
			match s {
				[] => true,
				['р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тдаться.
pub(crate) fn n13223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10626);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18269);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -удаться.
pub(crate) fn n13224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10627);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18270);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 18271);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыдаться.
pub(crate) fn n13225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10628);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18272);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18273);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юдаться.
pub(crate) fn n13226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10629);
	match s {
		[] => true,
		['л', 'б', s @ ..] => {
			match s {
				[] => true,
				['а', 'н', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32156);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядаться.
pub(crate) fn n13227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10630);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18274);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -свежаться.
pub(crate) fn n41120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25578);
	match s {
		[] => true,
		['о', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лежаться.
pub(crate) fn n25985(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18283);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 25579);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 25580);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 37055);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 40320);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 25581);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32158);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 25582);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 32159);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 37056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 25583);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 25584);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 25585);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 25586);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 32160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -межаться.
pub(crate) fn n25986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18284);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25587);
			s.is_empty()
		}
		['с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нежаться.
pub(crate) fn n25987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18285);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 25588);
			match s {
				[] => true,
				['о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -режаться.
pub(crate) fn n25988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18286);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 25589);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 37057);
							s.is_empty()
						}
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', 'а', 'р', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жжаться.
pub(crate) fn n13234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10637);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18287);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32161);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32162);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32163);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18288);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32164);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32165);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37058);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зжаться.
pub(crate) fn n13235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10638);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18289);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25590);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18290);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ъ', 'б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 18291);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25591);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18292);
			match s {
				[] => true,
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['м', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 18293);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25592);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ижаться.
pub(crate) fn n13236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10639);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18294);
			match s {
				[] => true,
				['о', 'о', s @ ..] => {
					super::push(out, 25593);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32166);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18295);
			match s {
				[] => true,
				['з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18296);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 18297);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 25594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32167);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37059);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18298);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25595);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', 'и', 'н', 'у', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лжаться.
pub(crate) fn n13237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10640);
	match s {
		[] => true,
		['о', 'д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25596);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32168);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25597);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ожаться.
pub(crate) fn n13238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10641);
	match s {
		[] => true,
		['б', 'о', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['в', 'о', 'р', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 18299);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18300);
			match s {
				[] => true,
				['о', 'м', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18301);
			match s {
				[] => true,
				['м', s @ ..] => {
					match s {
						[] => true,
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18302);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18303);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25600);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25601);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 32169);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25602);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32170);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32171);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 37060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25603);
					match s {
						[] => true,
						['д', 'у', '\u{1}', s @ ..] => {
							super::push(out, 32172);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 25604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25605);
					match s {
						[] => true,
						['с', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18304);
			match s {
				[] => true,
				['ч', 'и', 'н', s @ ..] => {
					match s {
						[] => true,
						['з', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', s @ ..] => {
									match s {
										[] => true,
										['а', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'а', 'з', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ржаться.
pub(crate) fn n13239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10642);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18305);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32174);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32175);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['и', 'р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 32176);
							match s {
								[] => true,
								['и', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', 'д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'р', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37061);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40321);
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
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 32177);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37062);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25607);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32178);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18306);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25608);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сжаться.
pub(crate) fn n13240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10643);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тжаться.
pub(crate) fn n13241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10644);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18307);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужаться.
pub(crate) fn n13242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10645);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18308);
			s.is_empty()
		}
		['м', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 18309);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25609);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25610);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25611);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25612);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18310);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['д', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['м', 'а', 'с', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18311);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыжаться.
pub(crate) fn n13243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10646);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18312);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18313);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжаться.
pub(crate) fn n13244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10647);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 18314);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25614);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32179);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
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
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37063);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 40322);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'а', 'р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', 'е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['м', 'а', 'с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'с', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word подсказаться.
pub(crate) fn n85143(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -езаться.
pub(crate) fn n13247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10650);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18319);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25628);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18320);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25629);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25630);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 32185);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 32186);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					super::push(out, 25631);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 25632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25633);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25634);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25635);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 32188);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 32189);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 32190);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25636);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32191);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37067);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25637);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 32192);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 32193);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 32194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					super::push(out, 25638);
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					super::push(out, 25639);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 25640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 25641);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37068);
									s.is_empty()
								}
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
			super::push(out, 18321);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -изаться.
pub(crate) fn n13248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10651);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 18322);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 25642);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 32196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18323);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25643);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25644);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 25645);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 25646);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32197);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37069);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25647);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25648);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32198);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18324);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25649);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25650);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 25651);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32199);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 32200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 25652);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25654);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ползаться.
pub(crate) fn n41255(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 32201);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 37070);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 32202);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 37071);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 32203);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нзаться.
pub(crate) fn n13250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10653);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 18326);
			match s {
				[] => true,
				['р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рзаться.
pub(crate) fn n13251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10654);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18327);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25655);
					match s {
						[] => true,
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 25656);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32204);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32205);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32206);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32207);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ъ', s @ ..] => {
					super::push(out, 25658);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 32208);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32209);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18328);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25659);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ызаться.
pub(crate) fn n13252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10655);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18329);
			match s {
				[] => true,
				['о', 'л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 32210);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 37072);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18330);
			match s {
				[] => true,
				['г', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьзаться.
pub(crate) fn n13253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10656);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 18331);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25660);
					match s {
						[] => true,
						['к', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юзаться.
pub(crate) fn n13254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10657);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -язаться.
pub(crate) fn n13255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10658);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 18333);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 18334);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25661);
					s.is_empty()
				}
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 25662);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 32211);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18335);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25663);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 18336);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 25664);
					match s {
						[] => true,
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 37073);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 40324);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word разбалакаться.
pub(crate) fn n85145(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word назавтракаться.
pub(crate) fn n69813(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -такаться.
pub(crate) fn n26054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18344);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 25674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бкаться.
pub(crate) fn n13257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10660);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18345);
			match s {
				[] => true,
				['р', 'а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32224);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32225);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 32226);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37083);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вкаться.
pub(crate) fn n13258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10661);
	match s {
		[] => true,
		['а', 'г', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 25675);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 32227);
							match s {
								[] => true,
								['р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'р', 'о', 'д', 'з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25676);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18346);
			match s {
				[] => true,
				['т', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -екаться.
pub(crate) fn n13259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10662);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 18347);
			match s {
				[] => true,
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 32229);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 40326);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 42066);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t27::n87889(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'с', '\u{1}', s @ ..] => {
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
				['а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 18348);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25677);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32230);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18349);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ы', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18350);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', 'е', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25678);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 32231);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37084);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 32232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'и', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['п', s @ ..] => {
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25679);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32233);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 37085);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 40327);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 40328);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 42067);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 18351);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', 'о', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', 'р', s @ ..] => {
					match s {
						[] => true,
						['е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', '\u{1}', s @ ..] => super::t27::n56319(s, out),
						_ => false,
					}
				}
				['т', 'о', '\u{1}', s @ ..] => super::t27::n41327(s, out),
				['у', '\u{1}', s @ ..] => super::t27::n41328(s, out),
				['ы', 'в', '\u{1}', s @ ..] => super::t27::n41329(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t27::n26066(s, out),
		_ => false,
	}
}

/// Words ending -икаться.
pub(crate) fn n13260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10663);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18353);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18354);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32234);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 37086);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18355);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 25681);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32235);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32236);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 37087);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 32237);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 32238);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32239);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 37088);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32240);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32241);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 32242);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25682);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 32243);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37089);
									match s {
										[] => true,
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18356);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 18357);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25684);
					match s {
						[] => true,
						['ч', 'с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 18358);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 18359);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 25685);
					match s {
						[] => true,
						['х', 'о', 'д', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 18360);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 18361);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 25686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лкаться.
pub(crate) fn n13261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10664);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 18362);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 25687);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32244);
							s.is_empty()
						}
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18363);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 25688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32245);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32246);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32247);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 37090);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40329);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 32248);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мкаться.
pub(crate) fn n13262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10665);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18364);
			match s {
				[] => true,
				['л', 'а', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32249);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 32250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18365);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32251);
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32252);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'е', 'ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32253);
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 18366);
			match s {
				[] => true,
				['т', 'о', 'д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['ш', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нкаться.
pub(crate) fn n13263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10666);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 18367);
			match s {
				[] => true,
				['р', 'д', 'а', 'н', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окаться.
pub(crate) fn n13264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10667);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 18368);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 25689);
					match s {
						[] => true,
						['а', 'х', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 18369);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25690);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25691);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25692);
					match s {
						[] => true,
						['а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 18370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25693);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
				['\u{1}', s @ ..] => {
					super::push(out, 25694);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25695);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 32254);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 25696);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32255);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 32256);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18371);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 25697);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 18372);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25698);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25699);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пкаться.
pub(crate) fn n13265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10668);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18373);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25700);
					match s {
						[] => true,
						['а', 'ц', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18374);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 25701);
					match s {
						[] => true,
						['у', 'к', 'с', 'и', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ркаться.
pub(crate) fn n13266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10669);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18375);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25702);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 25703);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 32257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 25704);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 25705);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32258);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32259);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 32260);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 32261);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 37092);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25706);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32262);
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 37093);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 37094);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18376);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25707);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 32263);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 40330);
											s.is_empty()
										}
										['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['с', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 32264);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 37095);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 25708);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 25709);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32266);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32267);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 37096);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18377);
			match s {
				[] => true,
				['ч', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32268);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32269);
							match s {
								[] => true,
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18378);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 25710);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 32270);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37097);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37098);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37099);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 37100);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 37101);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40331);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32271);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32272);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 32273);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => super::t27::n69871(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t27::n56394(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t27::n26094(s, out),
		['ы', s @ ..] => super::t27::n26095(s, out),
		_ => false,
	}
}

/// Words ending -скаться.
pub(crate) fn n13267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10670);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 18381);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32279);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 25716);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32280);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 32281);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', 'о', 'п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 32282);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 18382);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25717);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 32283);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37108);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 37109);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 37110);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 37111);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 40333);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25718);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 32284);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 37112);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 37113);
									match s {
										[] => true,
										['а', 'р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 18383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25719);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 25720);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25721);
					match s {
						[] => true,
						['р', s @ ..] => super::t27::n56422(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t27::n41386(s, out),
				['н', s @ ..] => super::t27::n41387(s, out),
				['о', s @ ..] => super::t27::n41388(s, out),
				['т', s @ ..] => super::t27::n41389(s, out),
				['ы', s @ ..] => super::t27::n41390(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t27::n26099(s, out),
		['у', s @ ..] => super::t27::n26100(s, out),
		['ы', s @ ..] => super::t27::n26101(s, out),
		_ => false,
	}
}

/// Words ending -ткаться.
pub(crate) fn n13268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 18387);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 18388);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 25735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 25736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 18389);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 25738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 25740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 32303);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 32304);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 18390);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25741);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

