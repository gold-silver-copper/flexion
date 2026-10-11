use flexion::Path;

/// Words ending -зивший.
pub(crate) fn n1999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1933);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5247);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11408);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18685);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18686);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18687);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 25061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11409);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18688);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11410);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18689);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25062);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11411);
					match s {
						[] => true,
						['а', 'з', 'з', s @ ..] => {
							super::push(out, 18690);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25063);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 18691);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29315);
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 29316);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 31634);
													match s {
														[] => true,
														['б', s @ ..] => super::t9::n34397(s, out),
														_ => false,
													}
												}
												['и', s @ ..] => {
													super::push(out, 31635);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31636);
													s.is_empty()
												}
												['г', s @ ..] => {
													super::push(out, 31637);
													match s {
														[] => true,
														['а', s @ ..] => super::t9::n34398(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 31638);
													match s {
														[] => true,
														['з', s @ ..] => super::t9::n34399(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 31639);
													s.is_empty()
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 29317);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 18692);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18693);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18694);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 18695);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18696);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11412);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5248);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11413);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11414);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11415);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18698);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5249);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 11416);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5250);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 11417);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5251);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5252);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11419);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18699);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25065);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 25066);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29318);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29319);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 29320);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18700);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 11420);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11421);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18702);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11422);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11423);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18703);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18704);
							match s {
								[] => true,
								['м', 'ы', s @ ..] => {
									super::push(out, 25067);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29321);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5253);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18705);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25068);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 29322);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 25069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 18706);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 18707);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n5479(s, out),
		['ь', s @ ..] => {
			super::push(out, 5254);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11430);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ливший.
pub(crate) fn n2000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1934);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5255);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11431);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18720);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18721);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25073);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11432);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18722);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 25074);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29324);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 25075);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 18723);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25076);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29325);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18724);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25077);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 11433);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11434);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 18726);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25078);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18727);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25079);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 11435);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11436);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18729);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 18730);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18731);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18732);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18733);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18734);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18735);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25080);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29326);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 25081);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18736);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11437);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18737);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 18738);
							match s {
								[] => true,
								['а', 'р', 'к', 'ы', s @ ..] => {
									super::push(out, 25084);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29327);
											s.is_empty()
										}
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
					super::push(out, 11438);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18739);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 18740);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 25085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11439);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18741);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18742);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18743);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11440);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 18744);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25087);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29328);
											s.is_empty()
										}
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
					super::push(out, 11441);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18745);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 11442);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18746);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18747);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25089);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 11443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5256);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11444);
					match s {
						[] => true,
						['к', 'с', 'ы', s @ ..] => {
							super::push(out, 18748);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25090);
									s.is_empty()
								}
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
			super::push(out, 5257);
			s.is_empty()
		}
		['г', 'у', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11445);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18749);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 11446);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11447);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18750);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25091);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29329);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5258);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11448);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 18751);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11449);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 18752);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18753);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t9::n5486(s, out),
		['з', s @ ..] => {
			super::push(out, 5260);
			s.is_empty()
		}
		['и', s @ ..] => super::t9::n5488(s, out),
		['й', s @ ..] => {
			super::push(out, 5262);
			s.is_empty()
		}
		['к', s @ ..] => super::t9::n5490(s, out),
		['м', s @ ..] => super::t9::n5491(s, out),
		['о', s @ ..] => super::t9::n5492(s, out),
		['п', s @ ..] => super::t9::n5493(s, out),
		['р', s @ ..] => super::t9::n5494(s, out),
		['с', s @ ..] => super::t9::n5495(s, out),
		['т', s @ ..] => super::t9::n5496(s, out),
		['у', s @ ..] => super::t9::n5497(s, out),
		['ф', s @ ..] => super::t9::n5498(s, out),
		['х', s @ ..] => super::t9::n5499(s, out),
		['ш', s @ ..] => super::t9::n5500(s, out),
		['ы', s @ ..] => super::t9::n5501(s, out),
		['ю', s @ ..] => super::t9::n5502(s, out),
		['я', s @ ..] => super::t9::n5503(s, out),
		_ => false,
	}
}

/// Words ending -мивший.
pub(crate) fn n2001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1935);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5276);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 18828);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 25140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 11506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5277);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5278);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 11507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 5279);
			match s {
				[] => true,
				['е', 'л', 'к', 'ы', s @ ..] => {
					super::push(out, 11508);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18830);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5280);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5281);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11509);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18831);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18832);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11510);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18833);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 25143);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 25144);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11511);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18834);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18835);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11512);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18836);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11513);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 18837);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18838);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 25146);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5282);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 18839);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25147);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29342);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ф', s @ ..] => {
							super::push(out, 18840);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25148);
									s.is_empty()
								}
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
			super::push(out, 5283);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5284);
			match s {
				[] => true,
				['з', s @ ..] => {
					match s {
						[] => true,
						['а', 'р', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 25149);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29343);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 25150);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25151);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 29344);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18841);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 25152);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 18842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11515);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11516);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 18843);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5285);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 5286);
			match s {
				[] => true,
				['р', 'у', 'с', 'ы', s @ ..] => {
					super::push(out, 11517);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5287);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5288);
			match s {
				[] => true,
				['р', 'п', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18845);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18846);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18847);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нивший.
pub(crate) fn n2002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5289);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11519);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11520);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 11521);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18849);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18850);
							match s {
								[] => true,
								['ц', 'ы', s @ ..] => {
									super::push(out, 25155);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29345);
											s.is_empty()
										}
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
					super::push(out, 11522);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11523);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 11524);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18851);
							match s {
								[] => true,
								['ч', 'ы', s @ ..] => {
									super::push(out, 25156);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29346);
											s.is_empty()
										}
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
					super::push(out, 11525);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11526);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18853);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25157);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25158);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 25159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 18854);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 18855);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 18856);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 18857);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18858);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 18859);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 25163);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18860);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 18861);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18862);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25164);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18863);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11527);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11528);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18864);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 18865);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 25167);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 18866);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25168);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25169);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29347);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 18867);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25170);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29348);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 18868);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25171);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 18869);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 18870);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 25173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 18871);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 25174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18872);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 18873);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 18874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5290);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 11530);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5291);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 5292);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 11531);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5293);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11532);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11533);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11534);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 11535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t9::n5522(s, out),
		['ж', s @ ..] => {
			super::push(out, 5295);
			match s {
				[] => true,
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 18888);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18889);
							match s {
								[] => true,
								['о', 'п', s @ ..] => super::t9::n26451(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t9::n5524(s, out),
		['и', s @ ..] => super::t9::n5525(s, out),
		['л', s @ ..] => super::t9::n5526(s, out),
		['м', s @ ..] => super::t9::n5527(s, out),
		['о', s @ ..] => super::t9::n5528(s, out),
		['п', s @ ..] => super::t9::n5529(s, out),
		['р', s @ ..] => super::t9::n5530(s, out),
		['с', s @ ..] => super::t9::n5531(s, out),
		['т', s @ ..] => super::t9::n5532(s, out),
		['у', s @ ..] => super::t9::n5533(s, out),
		['ч', s @ ..] => super::t9::n5534(s, out),
		['ш', s @ ..] => super::t9::n5535(s, out),
		['ы', s @ ..] => super::t9::n5536(s, out),
		['ю', s @ ..] => super::t9::n5537(s, out),
		['я', s @ ..] => super::t9::n5538(s, out),
		_ => false,
	}
}

/// Words ending -оивший.
pub(crate) fn n2003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1937);
	match s {
		[] => true,
		['б', 'с', 'а', s @ ..] => {
			super::push(out, 5311);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5312);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11583);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 18942);
							s.is_empty()
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25215);
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									super::push(out, 25216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18943);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11584);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 11585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5313);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 11586);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18945);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5314);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5315);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 11588);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5316);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 11589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5317);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 11590);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18946);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5318);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 11591);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 11592);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 18947);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 18948);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25218);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11593);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18949);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18950);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18951);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25220);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 25221);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25222);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29365);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5319);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 11595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пивший.
pub(crate) fn n2004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1938);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5320);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 11596);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11597);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5321);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11599);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18952);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11600);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 11601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5322);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11602);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11603);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 18953);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25224);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 29366);
											s.is_empty()
										}
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
					super::push(out, 11604);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18954);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25225);
									s.is_empty()
								}
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
			super::push(out, 5323);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5324);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 11605);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11606);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18956);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11607);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18957);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11608);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18958);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 18959);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 18960);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25229);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29367);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5325);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11609);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 11610);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 18961);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5326);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11611);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 18962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ривший.
pub(crate) fn n2005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1939);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5327);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11612);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11613);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18963);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18964);
							s.is_empty()
						}
						['е', 'ш', s @ ..] => {
							super::push(out, 18965);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18966);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 25231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18967);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11614);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18968);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 18969);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 18970);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 18971);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25234);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 18972);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 11615);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18974);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11616);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18975);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11617);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11618);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11619);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18977);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 18978);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 18979);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25236);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 25237);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25238);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29368);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 18980);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 18981);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25239);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29369);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 18982);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25240);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11620);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11621);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 11622);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 18983);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 18984);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 25241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 11623);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 11624);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 18985);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25242);
									s.is_empty()
								}
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
			super::push(out, 5328);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11625);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18986);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 18987);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 18988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11626);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 11627);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 18989);
							match s {
								[] => true,
								['е', 'с', 'ы', s @ ..] => {
									super::push(out, 25243);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29370);
											s.is_empty()
										}
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
					super::push(out, 11628);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 18990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 18991);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 18992);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25244);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 18993);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 18994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11630);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 11631);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 11632);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 18995);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25245);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29371);
											s.is_empty()
										}
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
					super::push(out, 11633);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 18996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5329);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5330);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11635);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25246);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25247);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t9::n30828(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => super::t9::n12221(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t9::n5559(s, out),
		['е', s @ ..] => super::t9::n5560(s, out),
		['з', s @ ..] => super::t9::n5561(s, out),
		['и', s @ ..] => super::t9::n5562(s, out),
		['к', s @ ..] => super::t9::n5563(s, out),
		['о', s @ ..] => super::t9::n5564(s, out),
		['т', s @ ..] => super::t9::n5565(s, out),
		['у', s @ ..] => super::t9::n5566(s, out),
		['ф', s @ ..] => super::t9::n5567(s, out),
		['х', s @ ..] => super::t9::n5568(s, out),
		['щ', s @ ..] => super::t9::n5569(s, out),
		['ы', s @ ..] => super::t9::n5570(s, out),
		['я', s @ ..] => super::t9::n5571(s, out),
		_ => false,
	}
}

/// Words ending -сивший.
pub(crate) fn n2006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1940);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5344);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19079);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19080);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 19081);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19082);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25298);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 19083);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25299);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11689);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19084);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 11690);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11691);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25300);
									s.is_empty()
								}
								['о', 'н', s @ ..] => {
									super::push(out, 25301);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29388);
											s.is_empty()
										}
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
					super::push(out, 11692);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11693);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19085);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25302);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29389);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5345);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11694);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 11695);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19086);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19087);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19088);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 11696);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 19089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11697);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19090);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19091);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 25305);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 25306);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 29390);
											s.is_empty()
										}
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
					super::push(out, 11698);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19092);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11699);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5346);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5347);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5348);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11701);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 19094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11702);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19095);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11703);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11704);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19096);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11705);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19098);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25309);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25310);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29391);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19099);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 25311);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 25312);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 29392);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 25313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 19100);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25314);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25315);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25316);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29393);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 29394);
											s.is_empty()
										}
										['ы', s @ ..] => {
											super::push(out, 29395);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31645);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 25317);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29396);
											s.is_empty()
										}
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
					super::push(out, 11706);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5349);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 11707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5350);
			match s {
				[] => true,
				['о', 'в', s @ ..] => {
					super::push(out, 11708);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n5579(s, out),
		['ы', s @ ..] => super::t9::n5580(s, out),
		_ => false,
	}
}

/// Words ending -тивший.
pub(crate) fn n2007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1941);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5353);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11714);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 11715);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 19105);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25327);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29403);
											s.is_empty()
										}
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
					super::push(out, 11716);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19106);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19107);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25328);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 11717);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19108);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19109);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11718);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19110);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25330);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25331);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29404);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 19111);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25332);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11719);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 19112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11720);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11721);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19114);
							match s {
								[] => true,
								['н', 'о', 'к', 'ы', s @ ..] => {
									super::push(out, 25333);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29405);
											s.is_empty()
										}
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
					super::push(out, 11722);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19115);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 19116);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 19117);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25334);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29406);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 31646);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 29407);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19118);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 11723);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 19119);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5354);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5355);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11724);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19120);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19121);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25336);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25337);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19122);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25338);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ц', 'с', 'е', s @ ..] => {
							super::push(out, 19123);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 25339);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11725);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19124);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25340);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11726);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19125);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 19126);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 19127);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19128);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11727);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 11728);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25343);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 25344);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 25345);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29409);
											s.is_empty()
										}
										_ => false,
									}
								}
								['д', s @ ..] => {
									super::push(out, 25346);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 25347);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29410);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19130);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 25348);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t9::n5584(s, out),
		['л', s @ ..] => super::t9::n5585(s, out),
		['н', s @ ..] => super::t9::n5586(s, out),
		['о', s @ ..] => super::t9::n5587(s, out),
		['п', s @ ..] => super::t9::n5588(s, out),
		['р', s @ ..] => super::t9::n5589(s, out),
		['с', s @ ..] => super::t9::n5590(s, out),
		['у', s @ ..] => super::t9::n5591(s, out),
		['ч', s @ ..] => super::t9::n5592(s, out),
		['ы', s @ ..] => super::t9::n5593(s, out),
		['ю', s @ ..] => super::t9::n5594(s, out),
		['я', s @ ..] => super::t9::n5595(s, out),
		_ => false,
	}
}

/// Words ending -фивший.
pub(crate) fn n2008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1942);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5368);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19179);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5369);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 5370);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чивший.
pub(crate) fn n2009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1943);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5371);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11757);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 11758);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 11759);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11760);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19181);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25397);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 19182);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 19183);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19184);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 19185);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 25399);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 11761);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11762);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19186);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 19187);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 19188);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19189);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25401);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 19190);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25402);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11763);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5372);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11764);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5373);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11765);
					match s {
						[] => true,
						['л', 'ы', s @ ..] => {
							super::push(out, 19191);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 11766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5374);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11767);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 11768);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19192);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25404);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29438);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 25405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 19193);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 19194);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19195);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 25406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19196);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19197);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25407);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11769);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 11770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5375);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11771);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19198);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 19199);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25409);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25410);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 25411);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29439);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19200);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19201);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 19202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11772);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11773);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5376);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5377);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11776);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19204);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11777);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19205);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19206);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 25413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 11778);
					match s {
						[] => true,
						['л', 'к', 'ы', s @ ..] => {
							super::push(out, 19207);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25414);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25415);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25416);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29440);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => super::t9::n5606(s, out),
		['р', s @ ..] => {
			super::push(out, 5379);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11788);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 19231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11789);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19232);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11790);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19233);
							s.is_empty()
						}
						['к', s @ ..] => super::t9::n20214(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n5608(s, out),
		['ы', s @ ..] => super::t9::n5609(s, out),
		['ь', s @ ..] => super::t9::n5610(s, out),
		['ю', s @ ..] => super::t9::n5611(s, out),
		['я', s @ ..] => super::t9::n5612(s, out),
		_ => false,
	}
}

/// Words ending -шивший.
pub(crate) fn n2010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1944);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5385);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 11819);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19261);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 11820);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 11821);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19262);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 25455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19263);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25456);
									s.is_empty()
								}
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
			super::push(out, 5386);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11822);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 11823);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 19264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5387);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11824);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19265);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11825);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 11826);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 11827);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19267);
							s.is_empty()
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25458);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11828);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19268);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19269);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25460);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11829);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19270);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 19271);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 19272);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 25461);
									s.is_empty()
								}
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
			super::push(out, 5388);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5389);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11830);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 11831);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11832);
					s.is_empty()
				}
				['п', 'а', s @ ..] => {
					super::push(out, 11833);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19275);
							match s {
								[] => true,
								['ъ', s @ ..] => {
									super::push(out, 25462);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 19276);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25463);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25464);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29459);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19277);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25465);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 25466);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29460);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 25467);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19278);
							match s {
								[] => true,
								['о', 'п', 'ы', s @ ..] => {
									super::push(out, 25468);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29461);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5390);
			match s {
				[] => true,
				['е', 'в', 'ы', s @ ..] => {
					super::push(out, 11834);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19279);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5391);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5392);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 11835);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19280);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25469);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29462);
											s.is_empty()
										}
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
					super::push(out, 11836);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 11837);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 11838);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19282);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25470);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11839);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19283);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 19284);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 19285);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 19286);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 11840);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19288);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 19289);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5393);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 11842);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5394);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 5395);
			match s {
				[] => true,
				['н', 'е', 'м', 'у', 'и', s @ ..] => {
					super::push(out, 11844);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5396);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 11845);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щивший.
pub(crate) fn n2011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1945);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5397);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11846);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19290);
							match s {
								[] => true,
								['т', 'ы', s @ ..] => {
									super::push(out, 25472);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29463);
											s.is_empty()
										}
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
					super::push(out, 11847);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19291);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25473);
									s.is_empty()
								}
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
			super::push(out, 5398);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5399);
			match s {
				[] => true,
				['в', 'ы', s @ ..] => {
					super::push(out, 11849);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'ы', s @ ..] => {
					super::push(out, 11850);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5400);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 11851);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 19294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19295);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 19296);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25475);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 19297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5401);
			match s {
				[] => true,
				['л', 'ы', s @ ..] => {
					super::push(out, 11852);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5402);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 11853);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нувший.
pub(crate) fn n2017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1950);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5410);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 11857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11858);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11859);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19301);
							match s {
								[] => true,
								['х', 'ы', s @ ..] => {
									super::push(out, 25477);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29464);
											s.is_empty()
										}
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
					super::push(out, 11860);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19302);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11861);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11862);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 11863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5411);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11864);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19303);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 19304);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 25478);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11865);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 5412);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11866);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 19305);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25479);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25480);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29465);
											s.is_empty()
										}
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
					super::push(out, 11867);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19306);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25481);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 25482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19307);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19308);
							match s {
								[] => true,
								['с', 'ы', s @ ..] => {
									super::push(out, 25483);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29466);
											s.is_empty()
										}
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
					super::push(out, 11868);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11869);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19310);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 19311);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19312);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25485);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19313);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25486);
									s.is_empty()
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
						['и', s @ ..] => {
							super::push(out, 19314);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19315);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 11870);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19316);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19317);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25488);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 25489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 11871);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19318);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19319);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 25490);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25491);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 11872);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19320);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 25492);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29467);
											s.is_empty()
										}
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
					super::push(out, 11873);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 19321);
							match s {
								[] => true,
								['ш', s @ ..] => {
									super::push(out, 25493);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 29468);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31656);
													s.is_empty()
												}
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
							super::push(out, 19322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25494);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 25495);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29469);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 25496);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 29470);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31657);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 25497);
									match s {
										[] => true,
										['\u{1}', s @ ..] => super::t9::n30927(s, out),
										_ => false,
									}
								}
								['т', s @ ..] => super::t9::n26768(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => super::t9::n12469(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t9::n5642(s, out),
		['з', s @ ..] => super::t9::n5643(s, out),
		['и', s @ ..] => super::t9::n5644(s, out),
		['к', s @ ..] => super::t9::n5645(s, out),
		['м', s @ ..] => super::t9::n5646(s, out),
		['о', s @ ..] => super::t9::n5647(s, out),
		['п', s @ ..] => super::t9::n5648(s, out),
		['р', s @ ..] => super::t9::n5649(s, out),
		['с', s @ ..] => super::t9::n5650(s, out),
		['т', s @ ..] => super::t9::n5651(s, out),
		['у', s @ ..] => super::t9::n5652(s, out),
		['х', s @ ..] => super::t9::n5653(s, out),
		['ц', s @ ..] => super::t9::n5654(s, out),
		['ч', s @ ..] => super::t9::n5655(s, out),
		['ы', s @ ..] => super::t9::n5656(s, out),
		['ь', s @ ..] => super::t9::n5657(s, out),
		['ю', s @ ..] => super::t9::n5658(s, out),
		['я', s @ ..] => super::t9::n5659(s, out),
		_ => false,
	}
}

/// Words ending -тнейший.
pub(crate) fn n5729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5497);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12022);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 19503);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 12023);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12024);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 19504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12025);
			match s {
				[] => true,
				['е', 'в', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25644);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 25645);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 29539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19505);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 25646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 12026);
			match s {
				[] => true,
				['п', 'о', '\u{1}', s @ ..] => {
					super::push(out, 19506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 12027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чнейший.
pub(crate) fn n5731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5499);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 12028);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12029);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 19507);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 25647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 19508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 12030);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 19509);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 25648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шнейший.
pub(crate) fn n5732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5500);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12031);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щнейший.
pub(crate) fn n5733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5501);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12032);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьнейший.
pub(crate) fn n5734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5502);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19510);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 25649);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 25650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 19511);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 25651);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 19512);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 25652);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 25653);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 25654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вающий.
pub(crate) fn n2175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2101);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5631);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12115);
					match s {
						[] => true,
						['п', 'о', s @ ..] => {
							super::push(out, 19540);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25659);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 12116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5632);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5633);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12118);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19541);
							match s {
								[] => true,
								['о', 'п', 'с', 'у', '\u{1}', s @ ..] => {
									super::push(out, 25660);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 12119);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 12120);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19543);
							match s {
								[] => true,
								['а', 'р', 'е', s @ ..] => {
									super::push(out, 25661);
									match s {
										[] => true,
										['щ', s @ ..] => {
											super::push(out, 29541);
											s.is_empty()
										}
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
					super::push(out, 12121);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12122);
					match s {
						[] => true,
						['л', 'к', 'с', s @ ..] => {
							super::push(out, 19544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 12123);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19545);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25662);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29542);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 25663);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 25664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29543);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 25665);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 25666);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 19546);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25667);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => super::t9::n20546(s, out),
						['з', s @ ..] => super::t9::n20547(s, out),
						['и', s @ ..] => {
							super::push(out, 19548);
							s.is_empty()
						}
						['о', s @ ..] => super::t9::n20549(s, out),
						['р', s @ ..] => super::t9::n20550(s, out),
						['т', s @ ..] => super::t9::n20551(s, out),
						['у', s @ ..] => super::t9::n20552(s, out),
						['ы', s @ ..] => super::t9::n20553(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t9::n12729(s, out),
				['л', s @ ..] => super::t9::n12730(s, out),
				['н', s @ ..] => super::t9::n12731(s, out),
				['п', s @ ..] => super::t9::n12732(s, out),
				['р', s @ ..] => super::t9::n12733(s, out),
				['х', s @ ..] => super::t9::n12734(s, out),
				['ч', s @ ..] => super::t9::n12735(s, out),
				['ш', s @ ..] => super::t9::n12736(s, out),
				['щ', s @ ..] => super::t9::n12737(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t9::n5873(s, out),
		['у', s @ ..] => super::t9::n5874(s, out),
		['ы', s @ ..] => super::t9::n5875(s, out),
		_ => false,
	}
}

/// Words ending -досберегающий.
pub(crate) fn n33243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31685);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 32778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -алосберегающий.
pub(crate) fn n33244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31686);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 32779);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дающий.
pub(crate) fn n2177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2103);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5646);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5647);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12158);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12159);
					match s {
						[] => true,
						['б', 'о', 'е', s @ ..] => {
							super::push(out, 19660);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 25766);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19661);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19662);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 19663);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19664);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 25767);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29580);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 25768);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25769);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 29581);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12161);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5648);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5649);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12164);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 12165);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12166);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12167);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12168);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19667);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 19668);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25770);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 29582);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 31687);
													match s {
														[] => true,
														['м', 'е', 'и', 'р', 'п', '\u{1}', s @ ..] => super::t9::n34404(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', 'м', 'е', 'и', 'р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 29583);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 19669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12169);
					s.is_empty()
				}
				['ъ', s @ ..] => {
					super::push(out, 12170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5651);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12171);
					s.is_empty()
				}
				['е', 'б', s @ ..] => {
					super::push(out, 12172);
					match s {
						[] => true,
						['о', 'п', 'е', s @ ..] => {
							super::push(out, 19670);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25771);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12173);
					s.is_empty()
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 19671);
							match s {
								[] => true,
								['з', 'о', 'в', '\u{1}', s @ ..] => {
									super::push(out, 25772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 19672);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									super::push(out, 25773);
									s.is_empty()
								}
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
			super::push(out, 5652);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5653);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 12175);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12176);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12177);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5654);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12178);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12179);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12180);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12181);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19675);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5655);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5656);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5657);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12183);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5658);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 5659);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12185);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12186);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19676);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -жающий.
pub(crate) fn n2178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2104);
	match s {
		[] => true,
		['а', 'р', 'о', s @ ..] => {
			super::push(out, 5660);
			match s {
				[] => true,
				['п', '\u{1}', s @ ..] => {
					super::push(out, 12187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5661);
			match s {
				[] => true,
				['а', 'н', 'с', 'о', s @ ..] => {
					super::push(out, 12188);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5662);
			match s {
				[] => true,
				['р', 'е', 'п', 'о', '\u{1}', s @ ..] => {
					super::push(out, 12189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5663);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5664);
			s.is_empty()
		}
		['о', 'р', s @ ..] => {
			super::push(out, 5665);
			match s {
				[] => true,
				['г', 'у', '\u{1}', s @ ..] => {
					super::push(out, 12190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зающий.
pub(crate) fn n2179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2105);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5667);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12191);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5668);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12192);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 12193);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5669);
			match s {
				[] => true,
				['о', 'п', '\u{1}', s @ ..] => {
					super::push(out, 12194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', 'е', 'м', s @ ..] => {
			super::push(out, 5670);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 5671);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12195);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5672);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кающий.
pub(crate) fn n2180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2106);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5673);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12196);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 12197);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12198);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19679);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12200);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12201);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19681);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12202);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5674);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12203);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5675);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12204);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12205);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 12206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'а', s @ ..] => {
					super::push(out, 12207);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19684);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 25775);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12208);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12209);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19685);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 25776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19686);
							match s {
								[] => true,
								['р', 'п', 'о', s @ ..] => {
									super::push(out, 25777);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29584);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5676);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12210);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19688);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 19689);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12211);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19691);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19692);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25778);
									s.is_empty()
								}
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29585);
											s.is_empty()
										}
										['е', 'с', 'в', '\u{1}', s @ ..] => {
											super::push(out, 29586);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29587);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 31688);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12212);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12213);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12214);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 12215);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 5677);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5678);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12217);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 19695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 5679);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5680);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12220);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12221);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19696);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 19697);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12222);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 12223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12224);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12225);
					match s {
						[] => true,
						['в', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25779);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 29588);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 25780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'м', s @ ..] => {
					super::push(out, 12226);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 19698);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', 'в', s @ ..] => {
					super::push(out, 12227);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 19699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5682);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 12228);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19700);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12229);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 12230);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5683);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12231);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 12232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5684);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5685);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12235);
					s.is_empty()
				}
				['в', '\u{1}', s @ ..] => {
					super::push(out, 12236);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12237);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12238);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12239);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 19703);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t9::n12849(s, out),
				['р', s @ ..] => {
					super::push(out, 12241);
					s.is_empty()
				}
				['т', s @ ..] => super::t9::n12851(s, out),
				_ => false,
			}
		}
		['ь', 'л', s @ ..] => super::t9::n5926(s, out),
		['э', s @ ..] => super::t9::n5927(s, out),
		['ю', s @ ..] => super::t9::n5928(s, out),
		['я', s @ ..] => super::t9::n5929(s, out),
		_ => false,
	}
}

/// Words ending -лающий.
pub(crate) fn n2181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2107);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5690);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12250);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 12251);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мающий.
pub(crate) fn n2182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2108);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5692);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 12252);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 12253);
					match s {
						[] => true,
						['а', 'з', s @ ..] => {
							super::push(out, 19710);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25782);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 29589);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19711);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 25783);
									s.is_empty()
								}
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
			super::push(out, 5693);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12254);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нающий.
pub(crate) fn n2183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2109);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 5694);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12255);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12256);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 19712);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12257);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12258);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 12259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5695);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 12260);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 19713);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12261);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пающий.
pub(crate) fn n2184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2110);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5696);
			s.is_empty()
		}
		['е', 'л', 'ш', s @ ..] => {
			super::push(out, 5697);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5698);
			match s {
				[] => true,
				['к', 'с', s @ ..] => {
					super::push(out, 12262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12263);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5699);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12264);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12265);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19715);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25786);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 19716);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 19717);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5700);
			match s {
				[] => true,
				['е', 'ч', 'о', s @ ..] => {
					super::push(out, 12266);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5701);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12267);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19719);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 25787);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12268);
					match s {
						[] => true,
						['с', 'ы', s @ ..] => {
							super::push(out, 19720);
							match s {
								[] => true,
								['в', '\u{1}', s @ ..] => {
									super::push(out, 25788);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 12269);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19721);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5702);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 5703);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12270);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5704);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рающий.
pub(crate) fn n2185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2111);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 5705);
			match s {
				[] => true,
				['и', 'о', s @ ..] => {
					super::push(out, 12271);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5706);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12272);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12273);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19723);
							match s {
								[] => true,
								['а', 'р', 'е', s @ ..] => {
									super::push(out, 25789);
									match s {
										[] => true,
										['ш', s @ ..] => {
											super::push(out, 29590);
											s.is_empty()
										}
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
					super::push(out, 12274);
					match s {
						[] => true,
						['о', 'п', 'е', s @ ..] => {
							super::push(out, 19724);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25790);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', 'у', s @ ..] => {
					super::push(out, 12275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12276);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12277);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5707);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сающий.
pub(crate) fn n2186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2112);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 12278);
					match s {
						[] => true,
						['у', '\u{1}', s @ ..] => {
							super::push(out, 19726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12279);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19727);
							match s {
								[] => true,
								['з', 'о', s @ ..] => {
									super::push(out, 25791);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29591);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5708);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12280);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12281);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19728);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5709);
			match s {
				[] => true,
				['р', 'т', 'о', 'п', s @ ..] => {
					super::push(out, 12282);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19729);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тающий.
pub(crate) fn n2187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5710);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12283);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 19730);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 19731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12284);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12285);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 12286);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19732);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 25792);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 29592);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 29593);
											s.is_empty()
										}
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
					super::push(out, 12287);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5711);
			match s {
				[] => true,
				['л', 'е', s @ ..] => {
					super::push(out, 12288);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							super::push(out, 19733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5712);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5713);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5714);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12289);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19734);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29594);
											s.is_empty()
										}
										['е', 'н', '\u{1}', s @ ..] => {
											super::push(out, 29595);
											s.is_empty()
										}
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
					super::push(out, 12290);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12291);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12292);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5715);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12293);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12294);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19735);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 25793);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 19736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25794);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 19737);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25795);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19738);
							match s {
								[] => true,
								['а', 'н', '\u{1}', s @ ..] => {
									super::push(out, 25796);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 25797);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29596);
											s.is_empty()
										}
										['о', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31689);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 31690);
													match s {
														[] => true,
														['с', s @ ..] => super::t9::n34405(s, out),
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
									super::push(out, 25798);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 19739);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12295);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12296);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19740);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12297);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19741);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12298);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19742);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19743);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 25800);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12299);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19744);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29597);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 29598);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31691);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12300);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 12301);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12302);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19746);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12303);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5716);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12304);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12305);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5717);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хающий.
pub(crate) fn n2188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2114);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5718);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12307);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12308);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5719);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5720);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5721);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5722);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12310);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 12311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19747);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12312);
					match s {
						[] => true,
						['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
							super::push(out, 19748);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12313);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12314);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19749);
							match s {
								[] => true,
								['з', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29599);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29600);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19750);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12315);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19751);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5723);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					super::push(out, 12316);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 5724);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цающий.
pub(crate) fn n2189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2115);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5725);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5726);
			match s {
				[] => true,
				['е', 'м', '\u{1}', s @ ..] => {
					super::push(out, 12320);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чающий.
pub(crate) fn n2190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2116);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5727);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 5728);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5729);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5730);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12321);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12322);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 12323);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19753);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25801);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29601);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25802);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 25803);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 19754);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 25804);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 25805);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29602);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5731);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5732);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12324);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 19755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12325);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5733);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12326);
					match s {
						[] => true,
						['с', '\u{1}', s @ ..] => {
							super::push(out, 19756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 12327);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12328);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12329);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19757);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									super::push(out, 25806);
									s.is_empty()
								}
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
			super::push(out, 5734);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 5735);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шающий.
pub(crate) fn n2191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2117);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5736);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12330);
					s.is_empty()
				}
				['р', 'т', s @ ..] => {
					super::push(out, 12331);
					match s {
						[] => true,
						['с', 'у', '\u{1}', s @ ..] => {
							super::push(out, 19758);
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
				['в', s @ ..] => {
					super::push(out, 12332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12333);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19760);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 25807);
									s.is_empty()
								}
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
			super::push(out, 5737);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5738);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5739);
			match s {
				[] => true,
				['к', '\u{1}', s @ ..] => {
					super::push(out, 12334);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12335);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 19761);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12336);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 19763);
							match s {
								[] => true,
								['а', 'р', 'е', s @ ..] => {
									super::push(out, 25808);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29603);
											s.is_empty()
										}
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
					super::push(out, 12337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 5740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щающий.
pub(crate) fn n2192(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2118);
	match s {
		[] => true,
		['а', 'р', 'в', 'о', s @ ..] => {
			super::push(out, 5741);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12338);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19764);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5742);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12339);
					match s {
						[] => true,
						['о', 'о', s @ ..] => {
							super::push(out, 19765);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 25809);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12340);
					s.is_empty()
				}
				['м', 'а', s @ ..] => {
					super::push(out, 12341);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
							super::push(out, 19766);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 25810);
									s.is_empty()
								}
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
			super::push(out, 5743);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5744);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ствующий.
pub(crate) fn n12963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12352);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 19774);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 19775);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 19776);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25812);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 29604);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 31692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 25813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 19777);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 25814);
					match s {
						[] => true,
						['п', 'с', 'о', 'г', '\u{1}', s @ ..] => {
							super::push(out, 29605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 19778);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25815);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 29606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 25816);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 29607);
							match s {
								[] => true,
								['о', 'т', '\u{1}', s @ ..] => {
									super::push(out, 31693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 25817);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 29608);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 31694);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 25818);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29609);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 31695);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 29610);
							match s {
								[] => true,
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 31696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 29611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 25819);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 29612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 19779);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 19780);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25820);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 29613);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25821);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 31698);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 31699);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 32782);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 31700);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 32783);
											s.is_empty()
										}
										['г', s @ ..] => {
											super::push(out, 32784);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 33303);
													s.is_empty()
												}
												_ => false,
											}
										}
										['к', 'т', s @ ..] => {
											super::push(out, 32785);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 33304);
													s.is_empty()
												}
												_ => false,
											}
										}
										['м', s @ ..] => {
											super::push(out, 32786);
											s.is_empty()
										}
										['н', 'ь', 'л', 'а', s @ ..] => {
											super::push(out, 32787);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 33305);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 25822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 19781);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 19782);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25823);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25824);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 29614);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31701);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 31702);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 32788);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 29615);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 29616);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 31703);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 31704);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 32789);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 29617);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 31705);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 29618);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 29619);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 25825);
					match s {
						[] => true,
						['о', 'в', '\u{1}', s @ ..] => {
							super::push(out, 29620);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 19783);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 25826);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 25827);
					match s {
						[] => true,
						['о', 'б', '\u{1}', s @ ..] => {
							super::push(out, 29621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 25828);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 25829);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 29622);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 29623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 19784);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 19785);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 19786);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 25830);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 29624);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 31707);
									match s {
										[] => true,
										['а', s @ ..] => super::t9::n34414(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => super::t9::n31086(s, out),
						['о', s @ ..] => super::t9::n31087(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ледующий.
pub(crate) fn n12966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12355);
	match s {
		[] => true,
		['с', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 25831);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 25832);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 29627);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 31709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 29628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 25833);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -редующий.
pub(crate) fn n12967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12356);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 19787);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 25834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 19788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -аждующий.
pub(crate) fn n12968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12357);
	s.is_empty()
}

/// Words ending -идующий.
pub(crate) fn n6020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5778);
	s.is_empty()
}

/// Words ending -лдующий.
pub(crate) fn n6021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5779);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 12358);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ндующий.
pub(crate) fn n6022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5780);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 12359);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 19789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 12360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -одующий.
pub(crate) fn n6023(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 12361);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 19790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 12362);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 12363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удующий.
pub(crate) fn n6024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5781);
	s.is_empty()
}

/// Words ending -ядующий.
pub(crate) fn n6025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5782);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 12364);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жующий.
pub(crate) fn n2233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2157);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5783);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12365);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зующий.
pub(crate) fn n2234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2158);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5784);
			match s {
				[] => true,
				['р', 'б', 'о', 'о', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19791);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 25835);
									match s {
										[] => true,
										['е', 'л', 'п', '\u{1}', s @ ..] => {
											super::push(out, 29629);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 19792);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25836);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29630);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 19793);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25837);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 29631);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5785);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12366);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12367);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 19794);
							match s {
								[] => true,
								['л', 'о', 'п', 'е', 'д', 'е', s @ ..] => {
									super::push(out, 25838);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29632);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5786);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кующий.
pub(crate) fn n2235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2159);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5788);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 5789);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 5790);
			match s {
				[] => true,
				['л', '\u{1}', s @ ..] => {
					super::push(out, 12369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5791);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лующий.
pub(crate) fn n2236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2160);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5792);
			match s {
				[] => true,
				['б', '\u{1}', s @ ..] => {
					super::push(out, 12370);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 12371);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5793);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 12372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5794);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12373);
					match s {
						[] => true,
						['о', 'з', 'и', '\u{1}', s @ ..] => {
							super::push(out, 19795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12374);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мующий.
pub(crate) fn n2237(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2161);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 5795);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нующий.
pub(crate) fn n2238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2162);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5796);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5797);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5798);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5799);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5800);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5801);
			match s {
				[] => true,
				['о', 'в', '\u{1}', s @ ..] => {
					super::push(out, 12378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5802);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пующий.
pub(crate) fn n2239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2163);
	s.is_empty()
}

/// Words ending -рующий.
pub(crate) fn n2240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2164);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5803);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12379);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 12380);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5804);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12381);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19796);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5805);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12382);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19797);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 25839);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 12383);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 12384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5806);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12385);
					match s {
						[] => true,
						['р', 'о', s @ ..] => {
							super::push(out, 19799);
							match s {
								[] => true,
								['с', 'б', 'а', '\u{1}', s @ ..] => {
									super::push(out, 25840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 12386);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19800);
							match s {
								[] => true,
								['р', 'г', '\u{1}', s @ ..] => {
									super::push(out, 25841);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 19801);
							s.is_empty()
						}
						['р', 'е', 'с', '\u{1}', s @ ..] => {
							super::push(out, 19802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 12387);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12388);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19803);
							s.is_empty()
						}
						['н', 'а', 'м', s @ ..] => {
							super::push(out, 19804);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25842);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 29633);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19805);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19806);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25843);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 29634);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 25844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', '\u{1}', s @ ..] => {
					super::push(out, 12389);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12390);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19807);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 25845);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 19808);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 25846);
									s.is_empty()
								}
								['н', 'о', s @ ..] => {
									super::push(out, 25847);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 29635);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31710);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', 'о', s @ ..] => {
									super::push(out, 25848);
									match s {
										[] => true,
										['н', 'п', 'и', 'г', '\u{1}', s @ ..] => {
											super::push(out, 29636);
											s.is_empty()
										}
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
					super::push(out, 12391);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19809);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 25849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 19810);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19811);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19812);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 19813);
							match s {
								[] => true,
								['а', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29637);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 29638);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => super::t9::n13004(s, out),
				['м', s @ ..] => super::t9::n13005(s, out),
				['н', s @ ..] => super::t9::n13006(s, out),
				['п', s @ ..] => super::t9::n13007(s, out),
				['р', s @ ..] => super::t9::n13008(s, out),
				['с', s @ ..] => super::t9::n13009(s, out),
				['т', s @ ..] => super::t9::n13010(s, out),
				['ц', s @ ..] => super::t9::n13011(s, out),
				['ш', s @ ..] => super::t9::n13012(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5807);
			match s {
				[] => true,
				['в', s @ ..] => super::t9::n13013(s, out),
				_ => false,
			}
		}
		['т', s @ ..] => super::t9::n6051(s, out),
		['у', s @ ..] => super::t9::n6052(s, out),
		['ф', s @ ..] => super::t9::n6053(s, out),
		_ => false,
	}
}

/// Words ending -сующий.
pub(crate) fn n2241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2165);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5811);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тующий.
pub(crate) fn n2242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2166);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5812);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5813);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 5814);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5815);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5816);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5817);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12406);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12407);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 19838);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25870);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 19839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12408);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 19840);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 5818);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12409);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19841);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фующий.
pub(crate) fn n2243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2167);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 5819);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хующий.
pub(crate) fn n2244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2168);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5820);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цующий.
pub(crate) fn n2245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2169);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 5821);
			match s {
				[] => true,
				['а', 'т', '\u{1}', s @ ..] => {
					super::push(out, 12410);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чующий.
pub(crate) fn n2246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2170);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 5822);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5823);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12411);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шующий.
pub(crate) fn n2247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2171);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 5824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -непроводящий.
pub(crate) fn n33280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31721);
	s.is_empty()
}

/// Words ending -преходящий.
pub(crate) fn n27163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 25881);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 29659);
			s.is_empty()
		}
		['е', 'н', '\u{1}', s @ ..] => {
			super::push(out, 29660);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 29661);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сходящий.
pub(crate) fn n13080(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12465);
	match s {
		[] => true,
		['о', 'в', '\u{1}', s @ ..] => {
			super::push(out, 19869);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -еуходящий.
pub(crate) fn n13081(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12466);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 19870);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рдящий.
pub(crate) fn n2282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2206);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удящий.
pub(crate) fn n2283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2207);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5886);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 5887);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5888);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5889);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 5890);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5891);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 5892);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ядящий.
pub(crate) fn n2284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2208);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5893);
			match s {
				[] => true,
				['г', 'ы', s @ ..] => {
					super::push(out, 12469);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19871);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5894);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -анкоговорящий.
pub(crate) fn n33281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31722);
	s.is_empty()
}

/// Words ending -ехсотрублевый.
pub(crate) fn n33283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31724);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 32793);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ухсотрублевый.
pub(crate) fn n33284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31725);
	s.is_empty()
}

/// The word трехуровневый.
pub(crate) fn n33285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31726);
	s.is_empty()
}

/// Words ending -асливый.
pub(crate) fn n6554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6283);
	match s {
		[] => true,
		['п', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 19972);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 25916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 19973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усливый.
pub(crate) fn n6556(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6285);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 12719);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тливый.
pub(crate) fn n2592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2497);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6286);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6287);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12721);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19974);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19975);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12722);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12723);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19976);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12724);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 19978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12725);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6288);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6289);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12727);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6290);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12728);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19979);
							match s {
								[] => true,
								['з', 'е', s @ ..] => {
									super::push(out, 25918);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29680);
											s.is_empty()
										}
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
					super::push(out, 12729);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12730);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19980);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 25919);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 25920);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12731);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19981);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19982);
							match s {
								[] => true,
								['в', 'з', s @ ..] => {
									super::push(out, 25922);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 12732);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 19983);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19984);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25923);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29681);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 25924);
									s.is_empty()
								}
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
			super::push(out, 6291);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6292);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12733);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19985);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 25925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 19986);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25926);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29682);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													super::push(out, 31727);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 25927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12734);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19987);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 19988);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12735);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12736);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19990);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 19991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6293);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 12737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6294);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6295);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19992);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 25930);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 29683);
											s.is_empty()
										}
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
					super::push(out, 12740);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19993);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чливый.
pub(crate) fn n2594(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2499);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6296);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12741);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12742);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6297);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6298);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12745);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6299);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6300);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'к', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12747);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 19995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 12748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6301);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6302);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шливый.
pub(crate) fn n2595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2500);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 12750);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 19996);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12751);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6303);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 12752);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25931);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 25932);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29684);
											s.is_empty()
										}
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
					super::push(out, 12753);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 19997);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									super::push(out, 25933);
									s.is_empty()
								}
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
			super::push(out, 6304);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12754);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 12755);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12756);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 19998);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 19999);
							match s {
								[] => true,
								['л', '\u{1}', s @ ..] => {
									super::push(out, 25934);
									s.is_empty()
								}
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
			super::push(out, 6305);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 6306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -удовый.
pub(crate) fn n2660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2564);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 6399);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12837);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 12838);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12839);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20064);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12840);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 12841);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 25974);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 25975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ядовый.
pub(crate) fn n2663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2567);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 6401);
			match s {
				[] => true,
				['а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 12842);
					s.is_empty()
				}
				['б', 'о', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20066);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 20067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 12843);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иковый.
pub(crate) fn n2689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2593);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6443);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6444);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12891);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 12892);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12893);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 6445);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 6446);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6447);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12895);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 12896);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20090);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 20091);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25986);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12897);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 20092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12898);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12899);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20094);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25987);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 20095);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20096);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25988);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29697);
											s.is_empty()
										}
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
					super::push(out, 12900);
					match s {
						[] => true,
						['а', 'к', '\u{1}', s @ ..] => {
							super::push(out, 20097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12901);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6448);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12902);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 12903);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20098);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 25989);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20099);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 25990);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20100);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 12904);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20102);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 25991);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20103);
							match s {
								[] => true,
								['л', 'о', s @ ..] => {
									super::push(out, 25992);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29698);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 29699);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 31728);
													s.is_empty()
												}
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
							super::push(out, 20104);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25993);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 25994);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29700);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 12905);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 20106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 12906);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20107);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 25995);
									s.is_empty()
								}
								['ш', s @ ..] => {
									super::push(out, 25996);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20108);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25997);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 29701);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 25998);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12907);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20109);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 25999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12908);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12909);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12910);
					match s {
						[] => true,
						['а', s @ ..] => super::t9::n21132(s, out),
						['у', s @ ..] => {
							super::push(out, 20111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t9::n13545(s, out),
				['т', s @ ..] => super::t9::n13546(s, out),
				['ч', s @ ..] => super::t9::n13547(s, out),
				['ш', s @ ..] => super::t9::n13548(s, out),
				['ь', s @ ..] => super::t9::n13549(s, out),
				_ => false,
			}
		}
		['п', s @ ..] => super::t9::n6727(s, out),
		['р', s @ ..] => super::t9::n6728(s, out),
		['с', s @ ..] => {
			super::push(out, 6451);
			s.is_empty()
		}
		['т', s @ ..] => super::t9::n6730(s, out),
		['ч', s @ ..] => super::t9::n6731(s, out),
		_ => false,
	}
}

/// Words ending -йковый.
pub(crate) fn n2690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2594);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6454);
			s.is_empty()
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 12933);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20142);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12934);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20143);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26017);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 20144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6455);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кковый.
pub(crate) fn n2691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2595);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6456);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20145);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20146);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лковый.
pub(crate) fn n2692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2596);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6457);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12936);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 12937);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 20147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6458);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12938);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 12939);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20148);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12940);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 12941);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 12942);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 12943);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 20150);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 26018);
									s.is_empty()
								}
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
			super::push(out, 6459);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12944);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12945);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6460);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12946);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20152);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 12947);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20153);
							s.is_empty()
						}
						['с', 'е', 'б', '\u{1}', s @ ..] => {
							super::push(out, 20154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 12949);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мковый.
pub(crate) fn n2693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2597);
	match s {
		[] => true,
		['а', 'з', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12950);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 12951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нковый.
pub(crate) fn n2694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2598);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6462);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12952);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20155);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 12953);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20156);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 26019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12954);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20157);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12955);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20158);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26020);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 12956);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20160);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20161);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6463);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6464);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12958);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20163);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', '\u{1}', s @ ..] => {
							super::push(out, 20164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 12959);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12960);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20165);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 20166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 12961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20167);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 12962);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 12963);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6465);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12964);
					match s {
						[] => true,
						['о', 'к', '\u{1}', s @ ..] => {
							super::push(out, 20169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12965);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20170);
							match s {
								[] => true,
								['в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29715);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 29716);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6466);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12966);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20171);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6467);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 12967);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 6468);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оковый.
pub(crate) fn n2695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2599);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6469);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 20173);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 20174);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26023);
									s.is_empty()
								}
								['щ', s @ ..] => {
									super::push(out, 26024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 20175);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26025);
									s.is_empty()
								}
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
			super::push(out, 6470);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 12969);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 12970);
					match s {
						[] => true,
						['с', 'о', s @ ..] => {
							super::push(out, 20176);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26026);
									s.is_empty()
								}
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
			super::push(out, 6471);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 6472);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пковый.
pub(crate) fn n2696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2600);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12971);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 12972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6473);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6474);
			match s {
				[] => true,
				['щ', s @ ..] => {
					super::push(out, 12973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6475);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рковый.
pub(crate) fn n2697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2601);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6476);
			match s {
				[] => true,
				['в', 'к', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 12975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6477);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 12976);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 20177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 12977);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6478);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6479);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6480);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 6481);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 6482);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12979);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сковый.
pub(crate) fn n2698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2602);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6483);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6484);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12981);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20181);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20182);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12982);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20183);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26028);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20184);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 20185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6485);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 12983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 12984);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12985);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20187);
							match s {
								[] => true,
								['-', 'о', 'н', 'н', s @ ..] => {
									super::push(out, 26029);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29717);
											match s {
												[] => true,
												['и', 'ц', 'а', 'м', 'р', 'о', 'ф', 'н', 'и', '\u{1}', s @ ..] => {
													super::push(out, 31738);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 12986);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 12987);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20188);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6486);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6487);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12989);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12990);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 20191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 6488);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тковый.
pub(crate) fn n2699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2603);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6489);
			match s {
				[] => true,
				['в', 'ц', s @ ..] => {
					match s {
						[] => true,
						['с', 'е', 'б', '\u{1}', s @ ..] => {
							super::push(out, 20192);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 20193);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 26030);
									s.is_empty()
								}
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
			super::push(out, 6490);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 12992);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 12993);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 12994);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20194);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6491);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6492);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12995);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12996);
					match s {
						[] => true,
						['в', 'з', 'и', '\u{1}', s @ ..] => {
							super::push(out, 20195);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20196);
							match s {
								[] => true,
								['е', 'л', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 26031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12997);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 26032);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 29718);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 26033);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29719);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6493);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12998);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уковый.
pub(crate) fn n2700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2604);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6494);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 12999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 6495);
			match s {
				[] => true,
				['м', 'а', 'б', s @ ..] => {
					super::push(out, 13000);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6496);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13001);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6497);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6498);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6499);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13003);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 26034);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29720);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 26035);
									s.is_empty()
								}
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
			super::push(out, 6500);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13004);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13005);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чковый.
pub(crate) fn n2701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6501);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13006);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13007);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6502);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6503);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6504);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13008);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13009);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 20200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13010);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20201);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 20202);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13011);
					match s {
						[] => true,
						['е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 20203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6505);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6506);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13012);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20204);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13013);
					match s {
						[] => true,
						['т', 'с', '\u{1}', s @ ..] => {
							super::push(out, 20205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6507);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 6508);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13014);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шковый.
pub(crate) fn n2702(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2606);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6509);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6510);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6511);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6512);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13015);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6513);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13016);
					match s {
						[] => true,
						['в', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
							super::push(out, 20206);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6514);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13017);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13018);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 20207);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 13019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 6515);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13020);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20208);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыковый.
pub(crate) fn n2703(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2607);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 6516);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 13021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13022);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6517);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13024);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьковый.
pub(crate) fn n2704(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2608);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6518);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13025);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20209);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20210);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6519);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яковый.
pub(crate) fn n2707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2611);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 6522);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6523);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 6524);
			match s {
				[] => true,
				['ш', 'ы', 'м', '\u{1}', s @ ..] => {
					super::push(out, 13028);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аноловый.
pub(crate) fn n13688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13049);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 20228);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иноловый.
pub(crate) fn n13689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13050);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 20229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -половый.
pub(crate) fn n6828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6545);
	s.is_empty()
}

/// Words ending -соловый.
pub(crate) fn n6829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6546);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13051);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -толовый.
pub(crate) fn n6830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6547);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13052);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 13053);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20230);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13054);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эоловый.
pub(crate) fn n6831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6548);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13055);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рловый.
pub(crate) fn n2717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2621);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6549);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -словый.
pub(crate) fn n2718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2622);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 6550);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13056);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 6551);
			match s {
				[] => true,
				['м', 'о', 'р', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20232);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26047);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 20233);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тловый.
pub(crate) fn n2719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2623);
	s.is_empty()
}

/// Words ending -уловый.
pub(crate) fn n2720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2624);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6552);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6553);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13058);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6554);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 13059);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эловый.
pub(crate) fn n2721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2625);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 6555);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13060);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яловый.
pub(crate) fn n2722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2626);
	s.is_empty()
}

/// Words ending -еновый.
pub(crate) fn n2736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2640);
	match s {
		[] => true,
		['г', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13096);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13097);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6580);
			match s {
				[] => true,
				['б', 'и', 'л', 'о', 'м', '\u{1}', s @ ..] => {
					super::push(out, 13098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'д', s @ ..] => {
			super::push(out, 6581);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13099);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6582);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13101);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['е', 'м', s @ ..] => {
									super::push(out, 26060);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29725);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 31739);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['э', s @ ..] => {
									super::push(out, 26061);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 13102);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13103);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6583);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13104);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 13105);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', 'ф', s @ ..] => {
			super::push(out, 6584);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13106);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 6585);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13107);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20265);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ц', s @ ..] => {
			super::push(out, 6586);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иновый.
pub(crate) fn n2737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2641);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6587);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 6588);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 6589);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13108);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 6590);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13110);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20266);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6591);
			s.is_empty()
		}
		['з', 'е', s @ ..] => {
			super::push(out, 6592);
			match s {
				[] => true,
				['р', '\u{1}', s @ ..] => {
					super::push(out, 13111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6593);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13112);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6594);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20268);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 20269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26062);
									s.is_empty()
								}
								['-', 'о', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 29726);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 29727);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 26063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13113);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13114);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13115);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20270);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 26064);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13116);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20271);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', 'м', '\u{1}', s @ ..] => {
							super::push(out, 20272);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 20273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6595);
			match s {
				[] => true,
				['а', 'т', 'с', s @ ..] => {
					super::push(out, 13118);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20274);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 26065);
									s.is_empty()
								}
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
			super::push(out, 6596);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13119);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 13120);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13121);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6597);
			match s {
				[] => true,
				['р', 'е', 'г', '\u{1}', s @ ..] => {
					super::push(out, 13122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6598);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6599);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13123);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20277);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26066);
									s.is_empty()
								}
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
			super::push(out, 6600);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6601);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13124);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 20278);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26067);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 29728);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 26068);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 20279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13125);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13126);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13127);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13128);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20280);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 26069);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'и', 'н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26070);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 26071);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29729);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 26072);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 20281);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', 'е', s @ ..] => {
			super::push(out, 6602);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13129);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6603);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13130);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -олновый.
pub(crate) fn n6895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6604);
	match s {
		[] => true,
		['в', 'о', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20284);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 20285);
					match s {
						[] => true,
						['т', 'о', 'р', 'о', 'к', '\u{1}', s @ ..] => {
							super::push(out, 26073);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 20286);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26074);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20287);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оновый.
pub(crate) fn n2741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2645);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6606);
			match s {
				[] => true,
				['р', 'а', 'к', 'и', s @ ..] => {
					super::push(out, 13132);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20288);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26075);
									match s {
										[] => true,
										['е', 'л', 'и', 'т', 'е', 'ц', 'а', '\u{1}', s @ ..] => {
											super::push(out, 29730);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6607);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6608);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 6609);
			match s {
				[] => true,
				['и', 'б', '\u{1}', s @ ..] => {
					super::push(out, 13133);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13135);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6610);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 13137);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 13138);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13139);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6611);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6612);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13140);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20291);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 26076);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20292);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26077);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 13141);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', 'и', 'н', '\u{1}', s @ ..] => {
					super::push(out, 13142);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6613);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13143);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13144);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', 'и', 'р', 'к', '\u{1}', s @ ..] => {
					super::push(out, 13145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 6614);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13146);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рновый.
pub(crate) fn n2742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2646);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 6615);
			match s {
				[] => true,
				['д', 'о', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20295);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 20296);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6616);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 6617);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 13148);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сновый.
pub(crate) fn n2743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2647);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6618);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13149);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хновый.
pub(crate) fn n2744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2648);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 6619);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13150);
					match s {
						[] => true,
						['в', 'с', '\u{1}', s @ ..] => {
							super::push(out, 20298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьновый.
pub(crate) fn n2745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2649);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6620);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -андровый.
pub(crate) fn n13837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13189);
	s.is_empty()
}

/// Words ending -индровый.
pub(crate) fn n13838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13190);
	match s {
		[] => true,
		['л', 'и', 'ц', s @ ..] => {
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 26082);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 26083);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 31740);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 31741);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ундровый.
pub(crate) fn n13839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13191);
	s.is_empty()
}

/// Words ending -еровый.
pub(crate) fn n2759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2663);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6656);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 6657);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 6658);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13192);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6659);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6660);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 6661);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13194);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 6662);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13195);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13196);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ировый.
pub(crate) fn n2760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2664);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 6663);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 6664);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13197);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20319);
							s.is_empty()
						}
						['т', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26084);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -йровый.
pub(crate) fn n2761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2665);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6665);
			match s {
				[] => true,
				['с', '\u{1}', s @ ..] => {
					super::push(out, 13199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кровый.
pub(crate) fn n2762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2666);
	s.is_empty()
}

/// Words ending -нровый.
pub(crate) fn n2763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2667);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6666);
			match s {
				[] => true,
				['ж', 'о', s @ ..] => {
					super::push(out, 13200);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оровый.
pub(crate) fn n2764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2668);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 6667);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6668);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 6669);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13202);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 13203);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20321);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20322);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13204);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6670);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6671);
			match s {
				[] => true,
				['н', 'е', 'л', 'о', 'к', '\u{1}', s @ ..] => {
					super::push(out, 13205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6672);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13206);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 20324);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 26087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 20325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 6673);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6674);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6675);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 6676);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тровый.
pub(crate) fn n2765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2669);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6677);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13207);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 13208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6678);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13209);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 20327);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 26088);
									s.is_empty()
								}
								['м', s @ ..] => {
									super::push(out, 26089);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29732);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 26090);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29733);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 31742);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 26091);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29734);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 31743);
													match s {
														[] => true,
														['с', 'х', 'е', 'р', 'т', '\u{1}', s @ ..] => super::t9::n34420(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 29735);
											s.is_empty()
										}
										['я', s @ ..] => {
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 31744);
													s.is_empty()
												}
												['п', s @ ..] => {
													super::push(out, 31745);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 31746);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ц', s @ ..] => {
									super::push(out, 26092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 20328);
							match s {
								[] => true,
								['л', 'и', 'к', s @ ..] => {
									match s {
										[] => true,
										['т', 'о', 'с', 'х', 'е', 'р', s @ ..] => {
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 31747);
													s.is_empty()
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 31748);
													s.is_empty()
												}
												_ => false,
											}
										}
										['х', 'е', 'р', s @ ..] => {
											match s {
												[] => true,
												['т', '\u{1}', s @ ..] => {
													super::push(out, 31749);
													s.is_empty()
												}
												['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 31750);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', 'о', 'с', 'х', 'е', 'р', 'ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
							super::push(out, 20329);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 20330);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26093);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['т', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t9::n34421(s, out),
														['и', s @ ..] => super::t9::n34422(s, out),
														_ => false,
													}
												}
												['ы', s @ ..] => {
													super::push(out, 31751);
													match s {
														[] => true,
														['т', 'е', 'ч', '\u{1}', s @ ..] => super::t9::n34423(s, out),
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
									super::push(out, 26094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 13210);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 13211);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20331);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6679);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20332);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 26095);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 20333);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 26096);
									match s {
										[] => true,
										['я', s @ ..] => {
											super::push(out, 29736);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 20334);
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									match s {
										[] => true,
										['т', '\u{1}', s @ ..] => {
											super::push(out, 29737);
											s.is_empty()
										}
										['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
											super::push(out, 29738);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6680);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6681);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13213);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13214);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20335);
							s.is_empty()
						}
						['к', s @ ..] => super::t9::n21365(s, out),
						['м', s @ ..] => {
							super::push(out, 20337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n6974(s, out),
		['ь', s @ ..] => super::t9::n6975(s, out),
		_ => false,
	}
}

/// Words ending -уровый.
pub(crate) fn n2766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2670);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6684);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 13217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 6685);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 6686);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 6687);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 6688);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13218);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хровый.
pub(crate) fn n2767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2671);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6689);
			match s {
				[] => true,
				['м', '\u{1}', s @ ..] => {
					super::push(out, 13219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6690);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юровый.
pub(crate) fn n2768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2672);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6691);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яровый.
pub(crate) fn n2769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2673);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 6692);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -топсовый.
pub(crate) fn n13903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13254);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 20357);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рсовый.
pub(crate) fn n2779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2683);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6726);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 6727);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20358);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 26100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20359);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6728);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13255);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ссовый.
pub(crate) fn n2780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2684);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13257);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13258);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 13259);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 13260);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20362);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6729);
			s.is_empty()
		}
		['и', 'л', s @ ..] => {
			super::push(out, 6730);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13261);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -усовый.
pub(crate) fn n2781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2685);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6731);
			match s {
				[] => true,
				['р', 'т', 'с', '\u{1}', s @ ..] => {
					super::push(out, 13262);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6732);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13263);
					s.is_empty()
				}
				['у', 'г', '\u{1}', s @ ..] => {
					super::push(out, 13264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 6733);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6734);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 6735);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13266);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 6736);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьсовый.
pub(crate) fn n2782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2686);
	match s {
		[] => true,
		['л', 'е', 'р', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13267);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20363);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20364);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 26101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ясовый.
pub(crate) fn n2784(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2688);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 6737);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13268);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вролитовый.
pub(crate) fn n27397(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 29741);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 29742);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -солитовый.
pub(crate) fn n21409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20379);
	s.is_empty()
}

/// Words ending -толитовый.
pub(crate) fn n21410(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 26106);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 26107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улитовый.
pub(crate) fn n13937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13285);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 20380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -митовый.
pub(crate) fn n7051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6757);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13286);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 20381);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 26108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нитовый.
pub(crate) fn n7052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6758);
	match s {
		[] => true,
		['а', 'м', s @ ..] => {
			super::push(out, 13287);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 20382);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 26109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 20383);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 13288);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 20384);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 26110);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ритовый.
pub(crate) fn n7053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6759);
	match s {
		[] => true,
		['е', 'н', s @ ..] => {
			super::push(out, 13290);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 20385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 20386);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 26111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 20387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 13291);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 13292);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ситовый.
pub(crate) fn n7054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6760);
	s.is_empty()
}

/// Words ending -титовый.
pub(crate) fn n7055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6761);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 13293);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 20389);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фитовый.
pub(crate) fn n7056(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6762);
	s.is_empty()
}

/// Words ending -хитовый.
pub(crate) fn n7057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6763);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13294);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 13295);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 20390);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 26112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 20391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 13296);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 20392);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 13297);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 20393);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьцитовый.
pub(crate) fn n7058(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6764);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 13298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шитовый.
pub(crate) fn n7059(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6765);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 13299);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йтовый.
pub(crate) fn n2789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2693);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13300);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20394);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13302);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 20395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13303);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20396);
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
					super::push(out, 13304);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13305);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ктовый.
pub(crate) fn n2790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2694);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6766);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6767);
			match s {
				[] => true,
				['л', 'е', 'р', '\u{1}', s @ ..] => {
					super::push(out, 13306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6768);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13307);
					match s {
						[] => true,
						['о', 'р', 'п', 'е', s @ ..] => {
							super::push(out, 20398);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'ф', '\u{1}', s @ ..] => {
					super::push(out, 13308);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтовый.
pub(crate) fn n2791(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2695);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6769);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13309);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нтовый.
pub(crate) fn n2792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2696);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6771);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13310);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 13311);
					match s {
						[] => true,
						['а', 'д', s @ ..] => {
							super::push(out, 20399);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 26114);
									s.is_empty()
								}
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
			super::push(out, 6772);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 6773);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6774);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13312);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13313);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20401);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 20402);
							match s {
								[] => true,
								['е', 'б', '\u{1}', s @ ..] => {
									super::push(out, 26115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 13314);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 20403);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6775);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13315);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 20404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20405);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20406);
							match s {
								[] => true,
								['н', 'д', 'о', '\u{1}', s @ ..] => {
									super::push(out, 26116);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20407);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 26117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 20408);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 26118);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отовый.
pub(crate) fn n2793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2697);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 6776);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13316);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 20409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 6777);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13317);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 20410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 13318);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6778);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 6779);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 6780);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 6781);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 6782);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6783);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 6784);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13321);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -птовый.
pub(crate) fn n2794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2698);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 6785);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13322);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6786);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ртовый.
pub(crate) fn n2795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2699);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6787);
			match s {
				[] => true,
				['в', 'ш', s @ ..] => {
					super::push(out, 13323);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13324);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 13325);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20413);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6788);
			match s {
				[] => true,
				['ч', '\u{1}', s @ ..] => {
					super::push(out, 13326);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6789);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 6790);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13327);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13328);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20414);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 20415);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 20416);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26119);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 26120);
									match s {
										[] => true,
										['э', s @ ..] => {
											super::push(out, 29743);
											s.is_empty()
										}
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
					super::push(out, 13329);
					s.is_empty()
				}
				['ф', s @ ..] => {
					super::push(out, 13330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 6791);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13331);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стовый.
pub(crate) fn n2796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2700);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6792);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13332);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13333);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26121);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26122);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 29744);
											match s {
												[] => true,
												['ю', s @ ..] => {
													super::push(out, 31752);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 29745);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6793);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13334);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20418);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 13335);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 13336);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 20419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13337);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20420);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 20421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13338);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6794);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13339);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 13340);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13341);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 20423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 13342);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 20424);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 13343);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 13344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 6795);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 6796);
			match s {
				[] => true,
				['е', 'т', 'н', s @ ..] => {
					super::push(out, 13346);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 20425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6797);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6798);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 6799);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13348);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 20426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -утовый.
pub(crate) fn n2797(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2701);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 6800);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13349);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 6801);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 13350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 6802);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 6803);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13351);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20428);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фтовый.
pub(crate) fn n2798(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2702);
	s.is_empty()
}

/// Words ending -хтовый.
pub(crate) fn n2799(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2703);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6804);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13352);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чтовый.
pub(crate) fn n2800(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2704);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6805);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 13353);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 6806);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьтовый.
pub(crate) fn n2801(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2705);
	match s {
		[] => true,
		['л', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13355);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20429);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 20430);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26124);
									match s {
										[] => true,
										['к', '\u{1}', s @ ..] => {
											super::push(out, 29746);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 20431);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 20432);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 20433);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 20434);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 20435);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 26125);
									match s {
										[] => true,
										['а', '\u{1}', s @ ..] => {
											super::push(out, 29747);
											s.is_empty()
										}
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
					super::push(out, 13356);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 13357);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 13358);
					match s {
						[] => true,
						['к', '\u{1}', s @ ..] => {
							super::push(out, 20436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ваемый.
pub(crate) fn n2988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2882);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 6980);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13489);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20497);
							match s {
								[] => true,
								['о', 'с', 'с', s @ ..] => {
									super::push(out, 26138);
									match s {
										[] => true,
										['о', 'в', 'е', s @ ..] => {
											super::push(out, 29752);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 31753);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 13490);
					match s {
						[] => true,
						['з', 'о', 'п', s @ ..] => {
							super::push(out, 20498);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26139);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29753);
											s.is_empty()
										}
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
					super::push(out, 13491);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20499);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 6981);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 6982);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13493);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 13494);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 20500);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 20501);
							match s {
								[] => true,
								['р', 'п', 'е', s @ ..] => {
									super::push(out, 26140);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29754);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 20502);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 26141);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29755);
											s.is_empty()
										}
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
					super::push(out, 13495);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 13496);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 13497);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 13498);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 20503);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26142);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29756);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 20504);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 26143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 20505);
							match s {
								[] => true,
								['л', s @ ..] => super::t9::n27437(s, out),
								['р', s @ ..] => super::t9::n27438(s, out),
								_ => false,
							}
						}
						['з', s @ ..] => super::t9::n21541(s, out),
						['и', s @ ..] => super::t9::n21542(s, out),
						['о', s @ ..] => super::t9::n21543(s, out),
						['р', s @ ..] => super::t9::n21544(s, out),
						['у', s @ ..] => super::t9::n21545(s, out),
						['ы', s @ ..] => super::t9::n21546(s, out),
						_ => false,
					}
				}
				['к', s @ ..] => super::t9::n14159(s, out),
				['л', s @ ..] => super::t9::n14160(s, out),
				['н', s @ ..] => super::t9::n14161(s, out),
				['п', s @ ..] => super::t9::n14162(s, out),
				['р', s @ ..] => super::t9::n14163(s, out),
				['х', s @ ..] => super::t9::n14164(s, out),
				['ч', s @ ..] => super::t9::n14165(s, out),
				['ш', s @ ..] => super::t9::n14166(s, out),
				['щ', s @ ..] => super::t9::n14167(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t9::n7304(s, out),
		['ы', s @ ..] => super::t9::n7305(s, out),
		_ => false,
	}
}

/// Words ending -ажаемый.
pub(crate) fn n7318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6997);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 13531);
			match s {
				[] => true,
				['у', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26232);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 26233);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 29787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 26234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 13532);
			match s {
				[] => true,
				['б', 'о', 'з', s @ ..] => {
					super::push(out, 20595);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ужаемый.
pub(crate) fn n7322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7001);
	match s {
		[] => true,
		['р', 'г', 'а', s @ ..] => {
			super::push(out, 13534);
			match s {
				[] => true,
				['з', 'о', s @ ..] => {
					super::push(out, 20597);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 26235);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яжаемый.
pub(crate) fn n7323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7002);
	s.is_empty()
}

/// Words ending -заемый.
pub(crate) fn n2992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2886);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7003);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -каемый.
pub(crate) fn n2993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2887);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7004);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 13535);
					match s {
						[] => true,
						['в', 'з', s @ ..] => {
							super::push(out, 20598);
							match s {
								[] => true,
								['и', 'о', s @ ..] => {
									super::push(out, 26236);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 29788);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7005);
			match s {
				[] => true,
				['л', 'к', '\u{1}', s @ ..] => {
					super::push(out, 13536);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 13537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 7006);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 7007);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 13539);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 20599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 7008);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 13540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', 'м', 'ч', s @ ..] => {
			super::push(out, 7009);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13541);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7010);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13542);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 20600);
							s.is_empty()
						}
						['ч', '\u{1}', s @ ..] => {
							super::push(out, 20601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 13543);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 20602);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 20603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13544);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13545);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 7011);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 13546);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 20604);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 13547);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 13548);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 20605);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 26237);
									s.is_empty()
								}
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
			super::push(out, 7012);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 13549);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 13550);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 13551);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 7013);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13552);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 13553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 13554);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 13555);
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 13556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 7014);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', 'р', s @ ..] => {
			super::push(out, 7015);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 13558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лаемый.
pub(crate) fn n2994(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2888);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 7016);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13559);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 13560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7017);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7018);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -маемый.
pub(crate) fn n2995(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2889);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7019);
			match s {
				[] => true,
				['н', 'с', s @ ..] => {
					super::push(out, 13561);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20607);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 26238);
									s.is_empty()
								}
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
			super::push(out, 7020);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 13562);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -наемый.
pub(crate) fn n2996(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2890);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7021);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -паемый.
pub(crate) fn n2997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2891);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7022);
			s.is_empty()
		}
		['е', 'л', 'ш', s @ ..] => {
			super::push(out, 7023);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7024);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13563);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 13564);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 13565);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 20609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 7025);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 13566);
					s.is_empty()
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 20610);
							s.is_empty()
						}
						['ч', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 26239);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 26240);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 26241);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 29789);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 7026);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 13567);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 13568);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 7027);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 7028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -раемый.
pub(crate) fn n2998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2892);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 7029);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7030);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 13569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -саемый.
pub(crate) fn n2999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2893);
	s.is_empty()
}

/// Words ending -таемый.
pub(crate) fn n3000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2894);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7031);
			match s {
				[] => true,
				['в', 'с', s @ ..] => {
					super::push(out, 13570);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 13571);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 20612);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 7032);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 7033);
			s.is_empty()
		}
		['с', 'а', 'ш', s @ ..] => {
			super::push(out, 7034);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 13572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 7035);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 13573);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хаемый.
pub(crate) fn n3001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2895);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 7036);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 13574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 7037);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 7038);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 7039);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 7040);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 20613);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 20614);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 20615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 13575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 7041);
			s.is_empty()
		}
		_ => false,
	}
}

