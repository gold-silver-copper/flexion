use flexion::Path;

/// Words ending -ночник.
pub(crate) fn n59146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58534);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 86469);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 86470);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108204);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122649);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 131410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 108205);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 108206);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122650);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131411);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 136535);
											match s {
												[] => true,
												['о', 'н', 'м', '-', s @ ..] => {
													super::push(out, 139547);
													match s {
														[] => true,
														['т', s @ ..] => super::t53::n142768(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86471);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 108207);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122651);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131412);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136536);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 139548);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 108208);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122652);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131413);
									s.is_empty()
								}
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
			super::push(out, 86472);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 108209);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 108210);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122654);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131414);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 136537);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 86473);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108211);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 122655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86474);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108212);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 122656);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131415);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136538);
											s.is_empty()
										}
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
					super::push(out, 108213);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122657);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86475);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108214);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 122658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 86476);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108215);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122659);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131416);
									s.is_empty()
								}
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
			super::push(out, 86477);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -почник.
pub(crate) fn n59147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58535);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86478);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 108217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86479);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108218);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122660);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131417);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136539);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 139549);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', 'к', 'а', 'з', '\u{1}', s @ ..] => {
					super::push(out, 108219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86480);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 108220);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 108221);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122661);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131418);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 136540);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 139550);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86481);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 86482);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108223);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рочник.
pub(crate) fn n59148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58536);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86483);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108224);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122663);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131419);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 108225);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 122664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 108226);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 86484);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108227);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86485);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 108228);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122666);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 122667);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86486);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108229);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122668);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 108230);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 108231);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 86487);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 108232);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122671);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131420);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136541);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 122672);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 131421);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136542);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 139551);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сочник.
pub(crate) fn n59149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58537);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 86488);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 86489);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 108233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86490);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 86491);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 86492);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 108236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 108237);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 122674);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -точник.
pub(crate) fn n59150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58538);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86493);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108238);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 108239);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 108240);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122677);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131423);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 108241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 108242);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122679);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 122680);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 86494);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108243);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86495);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108244);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 122682);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131424);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136543);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139552);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 108245);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122683);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 131425);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136544);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139553);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 131426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 108246);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 108247);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 108248);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122684);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 131427);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 122685);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 131428);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['щ', s @ ..] => {
					super::push(out, 108249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 86496);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 86497);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108250);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122686);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 86498);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108251);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122687);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131429);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136545);
											s.is_empty()
										}
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
					super::push(out, 108252);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122688);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 122689);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 108253);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 108254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 86499);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108255);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122690);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131431);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136546);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 139554);
													s.is_empty()
												}
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
							super::push(out, 122691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86500);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108256);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122692);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131432);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 122693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108257);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122694);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131433);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136547);
											s.is_empty()
										}
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
					super::push(out, 108258);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 122695);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131434);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 136548);
											match s {
												[] => true,
												['з', s @ ..] => super::t53::n140995(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t53::n123973(s, out),
						_ => false,
					}
				}
				['о', s @ ..] => super::t53::n109410(s, out),
				['р', s @ ..] => super::t53::n109411(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t53::n87425(s, out),
		['ы', s @ ..] => super::t53::n87426(s, out),
		['я', s @ ..] => super::t53::n87427(s, out),
		_ => false,
	}
}

/// Words ending -шочник.
pub(crate) fn n59152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58540);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 86505);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 108266);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -одгалстучник.
pub(crate) fn n141000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139560);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141326);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цатитысячник.
pub(crate) fn n141001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139561);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 141327);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142924);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 143232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending --стотысячник.
pub(crate) fn n141002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139562);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 141328);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142345);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 142925);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143233);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 143385);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -коммунальник.
pub(crate) fn n141004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139564);
	s.is_empty()
}

/// Words ending -ье-купальник.
pub(crate) fn n141005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139565);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 141329);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142346);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142926);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ральник.
pub(crate) fn n87584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86659);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 108378);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 122760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 108379);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 122761);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 131464);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136565);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 139566);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 108380);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тальник.
pub(crate) fn n87586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86661);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 108381);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 108382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хальник.
pub(crate) fn n87587(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 108383);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 122762);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 108384);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 122763);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чальник.
pub(crate) fn n87588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86662);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 108385);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 122764);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шальник.
pub(crate) fn n87589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86663);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 108386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -щальник.
pub(crate) fn n87590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86664);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 108387);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 122765);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ельник.
pub(crate) fn n59250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58634);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 86665);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108388);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122766);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 131465);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 122767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 86666);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108390);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122768);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 131466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108391);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 122769);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 131467);
									s.is_empty()
								}
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
			super::push(out, 86667);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 108392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122770);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 86668);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108393);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122771);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108394);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122772);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 122773);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131468);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136566);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 139567);
													match s {
														[] => true,
														['б', s @ ..] => super::t53::n142776(s, out),
														_ => false,
													}
												}
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
								['\u{1}', s @ ..] => {
									super::push(out, 131469);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 131470);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136567);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 139568);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 108395);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 122774);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 131471);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108396);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122775);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 122776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 108397);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108398);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122778);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131472);
									s.is_empty()
								}
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
			super::push(out, 86669);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 108399);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 86670);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 108400);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 122779);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131473);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136568);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 86671);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108401);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122780);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 122781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108402);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108403);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 122783);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 86672);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108404);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 86673);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108405);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122785);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108406);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122786);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 108407);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122787);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 122788);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108408);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 122789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108409);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122790);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 108410);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122791);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131474);
									s.is_empty()
								}
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
			super::push(out, 86674);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122792);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 122793);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 131475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108411);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122794);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 86675);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108412);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122795);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108413);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122796);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131476);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136569);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139569);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => super::t53::n87602(s, out),
		['ш', s @ ..] => super::t53::n87603(s, out),
		['ъ', s @ ..] => super::t53::n87604(s, out),
		_ => false,
	}
}

/// Words ending -ильник.
pub(crate) fn n59251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58635);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 86679);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 86680);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 86681);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108420);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131480);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 131481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 108421);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 122801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131482);
									s.is_empty()
								}
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
			super::push(out, 86682);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108422);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122802);
							match s {
								[] => true,
								['о', 'х', '-', s @ ..] => {
									super::push(out, 131483);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 136573);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 139573);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n142779(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 86683);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 86684);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 108423);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122803);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 86685);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 86686);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108424);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122804);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 122805);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 122806);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131484);
									s.is_empty()
								}
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
			super::push(out, 86687);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 86688);
			s.is_empty()
		}
		['ш', s @ ..] => {
			super::push(out, 86689);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108425);
					s.is_empty()
				}
				_ => false,
			}
		}
		['щ', s @ ..] => {
			super::push(out, 86690);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108426);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122807);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 131485);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 136574);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 139574);
													match s {
														[] => true,
														['л', s @ ..] => super::t53::n142780(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лльник.
pub(crate) fn n59252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58636);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86691);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122808);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 122809);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131486);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 131487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольник.
pub(crate) fn n59253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58637);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 86692);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108427);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 122810);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108428);
					s.is_empty()
				}
				['т', 'с', 'д', s @ ..] => {
					super::push(out, 108429);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122811);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131488);
									s.is_empty()
								}
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
			super::push(out, 86693);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 108430);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122812);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 131489);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136575);
											s.is_empty()
										}
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
					super::push(out, 108431);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122813);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 131490);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136576);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 122814);
							s.is_empty()
						}
						['х', 'е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 131491);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136577);
											s.is_empty()
										}
										['и', 'т', 'а', 'ц', 'д', 'а', 'в', 'д', '\u{1}', s @ ..] => {
											super::push(out, 136578);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 86694);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108432);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 86695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108433);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 108434);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 122815);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108435);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 122816);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 122817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 108436);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122818);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108437);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122819);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 108438);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 122820);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131495);
									s.is_empty()
								}
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
			super::push(out, 86696);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108439);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 122821);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131496);
									s.is_empty()
								}
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
			super::push(out, 86697);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108440);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122822);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131497);
									s.is_empty()
								}
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
			super::push(out, 86698);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 86699);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 108441);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 122823);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 131498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 122824);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131499);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136579);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 122825);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 131500);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 131501);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136580);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139575);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 131502);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульник.
pub(crate) fn n59254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58638);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 86700);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 108442);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 122826);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 86701);
			s.is_empty()
		}
		['ж', s @ ..] => {
			super::push(out, 86702);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108443);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 86703);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 86704);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 108444);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 122827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131503);
									s.is_empty()
								}
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
			super::push(out, 86705);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 122828);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 122829);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131504);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136581);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыльник.
pub(crate) fn n59255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58639);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 86706);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 86707);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108447);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 122830);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131505);
									s.is_empty()
								}
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
			super::push(out, 86708);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 86709);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 122831);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131506);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 122832);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108448);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 122833);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131508);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136582);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юльник.
pub(crate) fn n59256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58640);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 86710);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 108449);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 122834);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -яльник.
pub(crate) fn n59257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58641);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 86711);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108450);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 86712);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -зик-теоретик.
pub(crate) fn n141022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139582);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141340);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 142354);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сарь-трубоукладчик.
pub(crate) fn n144908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143455);
	s.is_empty()
}

/// Words ending -еолог-разведчик.
pub(crate) fn n142792(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141346);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 142360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -молет-разведчик.
pub(crate) fn n142793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141347);
	match s {
		[] => true,
		['а', 'с', '\u{1}', s @ ..] => {
			super::push(out, 142361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рь-водопроводчик.
pub(crate) fn n141034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139594);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141348);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142362);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142931);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 143237);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вод-подрядчик.
pub(crate) fn n141035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139595);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141349);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 142363);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -е-субподрядчик.
pub(crate) fn n141036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139596);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141350);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142364);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 142932);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143238);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word приемоответчик.
pub(crate) fn n141045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139605);
	s.is_empty()
}

/// Words ending -чик-налетчик.
pub(crate) fn n141046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139606);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 141359);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142372);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -грозоотметчик.
pub(crate) fn n141047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139607);
	s.is_empty()
}

/// Words ending -негабаритчик.
pub(crate) fn n141048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139608);
	s.is_empty()
}

/// Words ending -ос-обработчик.
pub(crate) fn n141049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139609);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141360);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142373);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142938);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 143243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -р-разработчик.
pub(crate) fn n141050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139610);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141361);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142374);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142939);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 143244);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 143391);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 143456);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -я-разработчик.
pub(crate) fn n141051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139611);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141362);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 142375);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 142376);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мпания-поставщик.
pub(crate) fn n144699(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143246);
	s.is_empty()
}

/// Words ending -ляр-краснодеревщик.
pub(crate) fn n141060(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139620);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141369);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142383);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тылезабивщик.
pub(crate) fn n141061(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139621);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141370);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142384);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word инициативщик.
pub(crate) fn n141062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139622);
	s.is_empty()
}

/// Words ending -расштыбовщик.
pub(crate) fn n141063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139623);
	s.is_empty()
}

/// Words ending --упаковщик.
pub(crate) fn n132976(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131617);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 136650);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 139624);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 141371);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142385);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 142945);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 143247);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n144845(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 139625);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 141372);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 142386);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 142946);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оупаковщик.
pub(crate) fn n132977(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131618);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 136651);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139626);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 141373);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 142387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раковщик.
pub(crate) fn n109949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108789);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 123052);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ловщик.
pub(crate) fn n59809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59181);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 87138);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108790);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 87139);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 108791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 108792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123054);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -новщик.
pub(crate) fn n59811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59183);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87140);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108793);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 123055);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 108794);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 123056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131620);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 131621);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136652);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 139627);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n142820(s, out),
														['р', s @ ..] => super::t53::n142821(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 139628);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n142822(s, out),
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
									super::push(out, 131622);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136653);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 131623);
									s.is_empty()
								}
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
			super::push(out, 87141);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 108795);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 87142);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 87143);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108796);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 123058);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131624);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139629);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 139630);
													match s {
														[] => true,
														['д', s @ ..] => super::t53::n142823(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 87144);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108797);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 123059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108798);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 123060);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 87145);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108799);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -повщик.
pub(crate) fn n59812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59184);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87146);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108800);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 123061);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', 'а', 'л', s @ ..] => {
			super::push(out, 87147);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108801);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ровщик.
pub(crate) fn n59813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59185);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 87148);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108802);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 123062);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131625);
									s.is_empty()
								}
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
			super::push(out, 87149);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 108803);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 123063);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 123064);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131627);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 136654);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'к', 'е', s @ ..] => {
							super::push(out, 123065);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131628);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136655);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 139631);
													s.is_empty()
												}
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
							super::push(out, 123066);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131629);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136656);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139632);
													match s {
														[] => true,
														['о', 'б', '-', s @ ..] => super::t53::n142824(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 108804);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 108805);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 123067);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131630);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 123068);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131631);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136657);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 123069);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 131632);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136658);
											s.is_empty()
										}
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
					super::push(out, 108806);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 123070);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131633);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136659);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139633);
													s.is_empty()
												}
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
							super::push(out, 123071);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 131634);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136660);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139634);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n142825(s, out),
														_ => false,
													}
												}
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
							super::push(out, 123072);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 108807);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 108808);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 108809);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 123073);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131635);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 131636);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136661);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', 'е', s @ ..] => {
							super::push(out, 123074);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t53::n109970(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 87150);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108811);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t53::n88090(s, out),
		['ф', 'и', s @ ..] => {
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 108813);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 108814);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -совщик.
pub(crate) fn n59814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59186);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87152);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 108815);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 108816);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 87153);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123081);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123082);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 123083);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131640);
									s.is_empty()
								}
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
			super::push(out, 87154);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108818);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 123084);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 123085);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131641);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136665);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 139638);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n142828(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 87155);
			s.is_empty()
		}
		['с', 'е', 'р', 'п', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108819);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 108820);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 87156);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 108821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 87157);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108822);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товщик.
pub(crate) fn n59815(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59187);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 87158);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 108823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 87159);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108824);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108825);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 123087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 123088);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 123089);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 131643);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108826);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 123090);
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
				['г', s @ ..] => {
					super::push(out, 108827);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 123091);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 131644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 108828);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 87160);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108829);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 123093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108830);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 123094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 87161);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108831);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 123095);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131645);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108832);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 123096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 108833);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 123097);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 131646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 108834);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 123098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 87162);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 108835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 87163);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108836);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 123099);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131647);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фовщик.
pub(crate) fn n59816(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59188);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 87164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ховщик.
pub(crate) fn n59817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59189);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87165);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108837);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 123100);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 131648);
									s.is_empty()
								}
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
			super::push(out, 87166);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108838);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 108839);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 108840);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 87167);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108841);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 87168);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108842);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цовщик.
pub(crate) fn n59818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59190);
	s.is_empty()
}

/// Words ending -ер-электронщик.
pub(crate) fn n141083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139643);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141387);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142400);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 142954);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -есс-подборщик.
pub(crate) fn n141086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139646);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141390);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сарь-сборщик.
pub(crate) fn n141087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139647);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141391);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142402);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 142955);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нефтесборщик.
pub(crate) fn n141088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139648);
	s.is_empty()
}

/// Words ending -оловосборщик.
pub(crate) fn n141089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139649);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141392);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -торщик.
pub(crate) fn n59881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59253);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87242);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 108908);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 87243);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 108909);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 123146);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 131676);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136684);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 139650);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 87244);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108910);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 123147);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'м', s @ ..] => {
			super::push(out, 87245);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108911);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 87246);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 108912);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 87247);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 108913);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 123148);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оклеивальщик.
pub(crate) fn n141091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139651);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141393);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -склеивальщик.
pub(crate) fn n141092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139652);
	s.is_empty()
}

/// The word приживальщик.
pub(crate) fn n141093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139653);
	s.is_empty()
}

/// Words ending -опрашивальщик.
pub(crate) fn n141094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139654);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141394);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -кладывальщик.
pub(crate) fn n141095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139655);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141395);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тывальщик.
pub(crate) fn n124439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123157);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 131683);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 136689);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 139656);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 141396);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 142403);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 142956);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дальщик.
pub(crate) fn n88201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87255);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 108923);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 123158);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 123159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131684);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -зальщик.
pub(crate) fn n88203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87257);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 108924);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 123160);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', 'р', '\u{1}', s @ ..] => {
			super::push(out, 108925);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 108926);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 123161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кальщик.
pub(crate) fn n88205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87259);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 108927);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 123162);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 123163);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131686);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 123164);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 123165);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 131687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 123166);
					s.is_empty()
				}
				['т', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131688);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 131689);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 136690);
									s.is_empty()
								}
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
			super::push(out, 108928);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 108929);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 123167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', 'и', 'т', s @ ..] => {
			super::push(out, 108930);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 123168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', 'т', s @ ..] => {
			super::push(out, 108931);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 123169);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131690);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 108932);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 123170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мальщик.
pub(crate) fn n88206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87260);
	s.is_empty()
}

/// Words ending -нальщик.
pub(crate) fn n88207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87261);
	s.is_empty()
}

/// Words ending -пальщик.
pub(crate) fn n88208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87262);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 108933);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 123171);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 131692);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 136691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 123172);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 131693);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 136692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 123173);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 131694);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 136693);
									s.is_empty()
								}
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
			super::push(out, 108934);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ральщик.
pub(crate) fn n88209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87263);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 108935);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 108936);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 123174);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 131695);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 136694);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 139657);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141397);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n143856(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сальщик.
pub(crate) fn n88210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87264);
	s.is_empty()
}

/// Words ending -тальщик.
pub(crate) fn n88211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87265);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 108937);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 123175);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 131696);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 136695);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139658);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 108938);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 123176);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 131697);
							match s {
								[] => true,
								['у', 'р', 'т', 'с', 'н', 'и', '-', s @ ..] => {
									super::push(out, 136696);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 139659);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 141398);
													match s {
														[] => true,
														['и', s @ ..] => super::t53::n143857(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', 'м', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
			super::push(out, 108939);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 108940);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 108941);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 123177);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 131698);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 136697);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уальщик.
pub(crate) fn n88212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87266);
	s.is_empty()
}

/// Words ending -хальщик.
pub(crate) fn n88213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87267);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 108942);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 123178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 108943);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 123179);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131700);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шальщик.
pub(crate) fn n88214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87268);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 108944);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 123180);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 131701);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ельщик.
pub(crate) fn n59889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59261);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 87269);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108945);
					match s {
						[] => true,
						['р', 'о', s @ ..] => {
							super::push(out, 123181);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131702);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136698);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 123182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 108946);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 123183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 108947);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 123184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 87270);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108948);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 123185);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 123186);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131703);
									s.is_empty()
								}
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
			super::push(out, 87271);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108949);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 108950);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 123187);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 131704);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136699);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139660);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 108951);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 108952);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 123188);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131705);
									s.is_empty()
								}
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
			super::push(out, 87272);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108953);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 123189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 87273);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108954);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 87274);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 87275);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 108955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 87276);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 108956);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 123190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 108957);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 123191);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131706);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136700);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 87277);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108958);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 108959);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 123192);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 131707);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 136701);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 139661);
													match s {
														[] => true,
														['щ', s @ ..] => super::t53::n142845(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 87278);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108960);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 123193);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 123194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108961);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 123195);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ильщик.
pub(crate) fn n59890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59262);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 87279);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108962);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 123196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131708);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', 'а', 'л', s @ ..] => {
			super::push(out, 87280);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108963);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 108964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123197);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 87281);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108965);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 123198);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 131709);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136702);
											match s {
												[] => true,
												['ы', s @ ..] => {
													super::push(out, 139662);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n142846(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', 'р', 'п', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
					super::push(out, 108966);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 87282);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 108967);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 123199);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 131710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136703);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 87283);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108968);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 123200);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 131711);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136704);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 139663);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n142847(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 87284);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 87285);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 87286);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 108969);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123201);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131712);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 108970);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 123202);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 131713);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136705);
											s.is_empty()
										}
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
					super::push(out, 108971);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 123203);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 108972);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 87287);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 87288);
			match s {
				[] => true,
				['с', 'и', s @ ..] => {
					super::push(out, 108973);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 123204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131715);
									s.is_empty()
								}
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
			super::push(out, 87289);
			s.is_empty()
		}
		['щ', s @ ..] => {
			super::push(out, 87290);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108974);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 123205);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лльщик.
pub(crate) fn n59891(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59263);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 87291);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 108975);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 123206);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 131716);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136706);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 139664);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n142848(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольщик.
pub(crate) fn n59892(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59264);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 87292);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 108976);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 87293);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 108977);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 123208);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 131717);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 136707);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139665);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульщик.
pub(crate) fn n59893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59265);
	match s {
		[] => true,
		['г', 'ы', s @ ..] => {
			super::push(out, 87294);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123209);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 87295);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 108979);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 123210);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 131718);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыльщик.
pub(crate) fn n59894(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 87296);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 108980);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 123211);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 131719);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136708);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 87297);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яльщик.
pub(crate) fn n59895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59266);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 87298);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 108981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123212);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 87299);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 108982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123213);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 108983);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 123214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 87300);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 87301);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чка-подросток.
pub(crate) fn n142855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141409);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 142411);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 142960);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143255);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нсцендентализм.
pub(crate) fn n142866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141420);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142416);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142962);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word назаретянин.
pub(crate) fn n141162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139722);
	s.is_empty()
}

/// Words ending -сердцеедство.
pub(crate) fn n141188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139748);
	s.is_empty()
}

/// Words ending -тормножество.
pub(crate) fn n141189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139749);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141443);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142426);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 142965);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 143257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чество.
pub(crate) fn n63980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63319);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 90090);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 110556);
					match s {
						[] => true,
						['т', 'о', 'к', 'л', 'е', 'ш', '\u{1}', s @ ..] => {
							super::push(out, 123971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 110557);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 110558);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 123972);
							s.is_empty()
						}
						['ь', s @ ..] => {
							super::push(out, 123973);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 132068);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136870);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 139750);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 90091);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 110559);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 123974);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 90092);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110560);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 110561);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 123975);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 90093);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 110562);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 110563);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 110564);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 110565);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 123976);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 132069);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 110566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t53::n91054(s, out),
		['м', s @ ..] => {
			super::push(out, 90095);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 110574);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 124002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 90096);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 110575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 90097);
			match s {
				[] => true,
				['н', 'и', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 124003);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 124004);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 132111);
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
						['о', s @ ..] => {
							super::push(out, 124005);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132112);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136918);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 124006);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 132113);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 110576);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 124007);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 90098);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 90099);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110577);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 124008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 90100);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110578);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 124009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 110579);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 124010);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 132114);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136919);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 139796);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t53::n142920(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 90101);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 110580);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 110581);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 124012);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 90102);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 110582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', 'д', 'з', 'и', 'п', '\u{1}', s @ ..] => {
			super::push(out, 90103);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 90104);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 110583);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 110584);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 110585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аздолбайство.
pub(crate) fn n141238(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139797);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141475);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -дношатайство.
pub(crate) fn n141239(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139798);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 141476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мбургианство.
pub(crate) fn n141240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139799);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141477);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овегетарианство.
pub(crate) fn n141241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139800);
	s.is_empty()
}

/// Words ending -тенгрианство.
pub(crate) fn n141242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139801);
	s.is_empty()
}

/// Words ending -дельфианство.
pub(crate) fn n141243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139802);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141478);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142445);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тманство.
pub(crate) fn n111794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110622);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 124039);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 132130);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 136932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -цманство.
pub(crate) fn n111795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110623);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 124040);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 132131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 136933);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -транство.
pub(crate) fn n111796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110624);
	s.is_empty()
}

/// Words ending -танство.
pub(crate) fn n91094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90133);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 110625);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 110626);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 110627);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 124041);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 132132);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 136934);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 139803);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 141479);
													match s {
														[] => true,
														['ь', s @ ..] => super::t53::n143898(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -енство.
pub(crate) fn n63997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63336);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 90134);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110628);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 124042);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 132133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 124043);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132134);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 136935);
											s.is_empty()
										}
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
					super::push(out, 110629);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 110630);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 124044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 110631);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124045);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 132135);
									s.is_empty()
								}
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
			super::push(out, 90135);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110632);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 124046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 124047);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 132136);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 124048);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 132137);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 110633);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 124049);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 132138);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136936);
											s.is_empty()
										}
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
					super::push(out, 110634);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 124050);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 90136);
			s.is_empty()
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 110635);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 110636);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 124051);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 132139);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136937);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 110637);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 110638);
					match s {
						[] => true,
						['ю', s @ ..] => {
							super::push(out, 124052);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 132140);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 136938);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 90137);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110639);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 124053);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 132141);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 136939);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 139804);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 110640);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 124054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 110641);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124055);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 132142);
									s.is_empty()
								}
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
			super::push(out, 90138);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 110642);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 124056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 132143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 110643);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 110644);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 124057);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инство.
pub(crate) fn n63998(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63337);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 90139);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 110645);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 124058);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 110646);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 90140);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 90141);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 110647);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 90142);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 90143);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 110648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 90144);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110649);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 124059);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 132144);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136940);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139805);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 90145);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 90146);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 110650);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 110651);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 124060);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 132145);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 136941);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 90147);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 110652);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 124061);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 132146);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136942);
											s.is_empty()
										}
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
					super::push(out, 110653);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 124062);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132147);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136943);
											s.is_empty()
										}
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
					super::push(out, 110654);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 124063);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132148);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 136944);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 90148);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 90149);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110656);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 110657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 90150);
			match s {
				[] => true,
				['й', s @ ..] => {
					super::push(out, 110658);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124064);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132149);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136945);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 139806);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n142926(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 110659);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124065);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 132150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 110660);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 124066);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 132151);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 136946);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 124067);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132152);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136947);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -онство.
pub(crate) fn n63999(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63338);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 90151);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110661);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 124068);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 132153);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 110662);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 124069);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 132154);
									s.is_empty()
								}
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
			super::push(out, 90152);
			match s {
				[] => true,
				['к', 'о', 'п', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124070);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132155);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136948);
											match s {
												[] => true,
												['ц', s @ ..] => {
													super::push(out, 139807);
													s.is_empty()
												}
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
							super::push(out, 124071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 90153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -унство.
pub(crate) fn n64000(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63339);
	s.is_empty()
}

/// Words ending -янство.
pub(crate) fn n64001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63340);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 90154);
			s.is_empty()
		}
		['ь', s @ ..] => {
			super::push(out, 90155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ноградарство.
pub(crate) fn n141249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139808);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -д-государство.
pub(crate) fn n141250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139809);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141482);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142447);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142975);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 143260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ерство.
pub(crate) fn n64011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63350);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 90175);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 110683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 90176);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110684);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 124086);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 132165);
									s.is_empty()
								}
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
			super::push(out, 90177);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 110685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 90178);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110686);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 124087);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 132166);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 110687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 90179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110688);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 124088);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 132167);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 136953);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139810);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 110689);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 124089);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 110690);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', 'о', s @ ..] => {
					super::push(out, 110691);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124091);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 132168);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 110692);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 124092);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 132169);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 124093);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 132170);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 136954);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 90180);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 110693);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 90181);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 110694);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124094);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 132171);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 136955);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 139811);
													s.is_empty()
												}
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
							super::push(out, 124095);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132172);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 136956);
											s.is_empty()
										}
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
					super::push(out, 110695);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124096);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 132173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 110696);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 124097);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 132174);
									s.is_empty()
								}
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
			super::push(out, 90182);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110697);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124098);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 132175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 110698);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 124099);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 110699);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 124100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 110700);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 124101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 90183);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110701);
					s.is_empty()
				}
				['и', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 124102);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 132177);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 136957);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 124103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['й', s @ ..] => {
					super::push(out, 110702);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124104);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 110703);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 110704);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124105);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 132178);
									s.is_empty()
								}
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
			super::push(out, 90184);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 110705);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 124106);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 132179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 110706);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 124107);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 132180);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 136958);
											s.is_empty()
										}
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
					super::push(out, 110707);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 124108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 110708);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124109);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 132181);
									match s {
										[] => true,
										['с', s @ ..] => super::t53::n138375(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => super::t53::n91149(s, out),
		['н', s @ ..] => super::t53::n91150(s, out),
		['п', s @ ..] => super::t53::n91151(s, out),
		['р', s @ ..] => super::t53::n91152(s, out),
		['с', s @ ..] => super::t53::n91153(s, out),
		['т', s @ ..] => super::t53::n91154(s, out),
		['ф', s @ ..] => super::t53::n91155(s, out),
		['х', s @ ..] => super::t53::n91156(s, out),
		['ц', s @ ..] => super::t53::n91157(s, out),
		['ч', s @ ..] => super::t53::n91158(s, out),
		['ш', s @ ..] => super::t53::n91159(s, out),
		['ь', s @ ..] => super::t53::n91160(s, out),
		_ => false,
	}
}

/// Words ending -орство.
pub(crate) fn n64013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63352);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 90196);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 90197);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110741);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 110742);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 124143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 90198);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110743);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124144);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 124145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 110744);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 124146);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124147);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 132206);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 136972);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 124148);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 132207);
									s.is_empty()
								}
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
			super::push(out, 90199);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110745);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 124149);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 110746);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 124150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 110747);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 124151);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 90200);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 110748);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124152);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 132208);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 110749);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 124153);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132209);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136973);
											s.is_empty()
										}
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
					super::push(out, 110750);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124154);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 132210);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 110751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 124155);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 90201);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110752);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 124156);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 90202);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 110753);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 110754);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 124157);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 90203);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110755);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 124158);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 90204);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 110756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 124159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 90205);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 110757);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 124160);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 132211);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136974);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 90206);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 110758);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 110759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 90207);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 110760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 90208);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 110761);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 124161);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 132212);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 136975);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ронырство.
pub(crate) fn n125447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124162);
	s.is_empty()
}

/// Words ending -тырство.
pub(crate) fn n91179(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 110764);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 124163);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 110765);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 124164);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 132214);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эрство.
pub(crate) fn n64016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63354);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 90212);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 90213);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ярство.
pub(crate) fn n64017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63355);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 90214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -езграмотство.
pub(crate) fn n141259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139818);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 141484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -донкихотство.
pub(crate) fn n141260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139819);
	s.is_empty()
}

/// Words ending -донкишотство.
pub(crate) fn n141261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139820);
	s.is_empty()
}

/// Words ending -консультство.
pub(crate) fn n141262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139821);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141485);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -род-графство.
pub(crate) fn n141263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139822);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141486);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьдмаршальство.
pub(crate) fn n141264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139823);
	s.is_empty()
}

/// Words ending -кодельство.
pub(crate) fn n133589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132227);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 136986);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139824);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 141487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мельство.
pub(crate) fn n111962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110785);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 124180);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 132228);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 136987);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 139825);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 141488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тельство.
pub(crate) fn n111963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110786);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 124181);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 132229);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 136988);
							match s {
								[] => true,
								['б', 'о', '-', s @ ..] => {
									super::push(out, 139826);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 141489);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 142450);
													match s {
														[] => true,
														['в', s @ ..] => super::t53::n144430(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 124182);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 132230);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 136989);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 136990);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 139827);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 132231);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 136991);
							s.is_empty()
						}
						['ж', s @ ..] => {
							super::push(out, 136992);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 139828);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 141490);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 136993);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 139829);
									s.is_empty()
								}
								['х', s @ ..] => {
									super::push(out, 139830);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 141491);
											match s {
												[] => true,
												['о', 'в', 'е', 'р', 'п', 'о', s @ ..] => {
													super::push(out, 142451);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n144431(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 132232);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 132233);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 136994);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 139831);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 136995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 124183);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 132234);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 136996);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 139832);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 141492);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 142452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ильство.
pub(crate) fn n91200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90232);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 110787);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 124184);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 132235);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 136997);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ольство.
pub(crate) fn n91201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90233);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 110788);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 124185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 110789);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 124186);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 132236);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 110790);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 124187);
					match s {
						[] => true,
						['щ', s @ ..] => {
							super::push(out, 132237);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 136998);
									s.is_empty()
								}
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
				['а', s @ ..] => {
					super::push(out, 124188);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 132238);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 136999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 124189);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 132239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 110791);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 124190);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132240);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137000);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139833);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ульство.
pub(crate) fn n91202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90234);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 110792);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 124191);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 110793);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 124192);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 110794);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 124193);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 132243);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137001);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139834);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word человечишко.
pub(crate) fn n141277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139836);
	s.is_empty()
}

/// Words ending -рана-экспортер.
pub(crate) fn n143921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142469);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142989);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 143267);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 143400);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ективизатор.
pub(crate) fn n133881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132519);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 137127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ятивизатор.
pub(crate) fn n126020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124732);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 132520);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 137128);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -овизатор.
pub(crate) fn n113057(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111872);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 124733);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 132521);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -гизатор.
pub(crate) fn n93017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92039);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 111873);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 124734);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лизатор.
pub(crate) fn n93018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92040);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 124735);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132523);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 137129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 124736);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 132524);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137130);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 132525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 124737);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 124738);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132526);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 137131);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 124739);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132527);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 137132);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139895);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 137133);
									s.is_empty()
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 132528);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137134);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139896);
											match s {
												[] => true,
												['-', s @ ..] => {
													super::push(out, 141525);
													match s {
														[] => true,
														['ь', s @ ..] => super::t53::n143926(s, out),
														_ => false,
													}
												}
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
							super::push(out, 132529);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137135);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 139897);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141526);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143927(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
						['е', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 137136);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 139898);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 141527);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 137137);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 139899);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 132530);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 137138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 124740);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132531);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 137139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 132532);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137140);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139900);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 141528);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 124741);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 111874);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 124742);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 132533);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137141);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 139901);
											s.is_empty()
										}
										_ => false,
									}
								}
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
					super::push(out, 124743);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132534);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137142);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', 'и', s @ ..] => {
					super::push(out, 124744);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 132535);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 124745);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 132536);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 132537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мизатор.
pub(crate) fn n93019(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 111875);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 124746);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132538);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137144);
									s.is_empty()
								}
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
				['т', s @ ..] => {
					super::push(out, 124747);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 124748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 132539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 111876);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -низатор.
pub(crate) fn n93020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92041);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 111877);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 124749);
					match s {
						[] => true,
						['р', 'о', '-', s @ ..] => {
							super::push(out, 132540);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137145);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 139902);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141529);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143928(s, out),
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
									super::push(out, 137146);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 139903);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 141530);
													match s {
														[] => true,
														['з', s @ ..] => super::t53::n143929(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 124750);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 132541);
							match s {
								[] => true,
								['у', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139904);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 139905);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 141531);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 124751);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132542);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 137147);
									s.is_empty()
								}
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
				['г', s @ ..] => {
					super::push(out, 124752);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132543);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 137148);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 124753);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 124754);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 132545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 111878);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 124755);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 132546);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 132547);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 137149);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 124756);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 132548);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137150);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 139906);
											s.is_empty()
										}
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
					super::push(out, 124757);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 132549);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 137151);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 124758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 111879);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 124759);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ризатор.
pub(crate) fn n93021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92042);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 111880);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 124760);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 132550);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 137152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 124761);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 132551);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137153);
									s.is_empty()
								}
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
				['в', s @ ..] => {
					super::push(out, 124762);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 132552);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 137154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 124763);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 132553);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137155);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 139907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', 'л', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 124764);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 132554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 124765);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 132555);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137156);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 139908);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тизатор.
pub(crate) fn n93022(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 124766);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 132556);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137157);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 132557);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137158);
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
						['г', s @ ..] => {
							super::push(out, 132558);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 132559);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137159);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 137160);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139909);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 141532);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n143930(s, out),
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
									super::push(out, 137161);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139910);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 132560);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 132561);
							s.is_empty()
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137162);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139911);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 137163);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 139912);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141533);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', 'к', 'о', 'м', 'е', 'д', '\u{1}', s @ ..] => {
					super::push(out, 124767);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 111881);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 111882);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 124768);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 132562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 124769);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 132563);
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
					super::push(out, 124770);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 132564);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 137164);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139913);
											s.is_empty()
										}
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
					super::push(out, 124771);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 132565);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137165);
									s.is_empty()
								}
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
			super::push(out, 111883);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 124772);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 132566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хизатор.
pub(crate) fn n93023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92043);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 111884);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 124773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 111885);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 124774);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озатор.
pub(crate) fn n66546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65867);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 92044);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 111886);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124775);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132567);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 137166);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 139914);
													match s {
														[] => true,
														['у', s @ ..] => super::t53::n142980(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 111887);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -узатор.
pub(crate) fn n66547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65868);
	s.is_empty()
}

/// Words ending -ие-индикатор.
pub(crate) fn n141356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139915);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141535);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142480);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 142997);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 143271);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 143403);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ль-индикатор.
pub(crate) fn n141357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139916);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141536);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 142481);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 142998);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 143272);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 143404);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 143463);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 141537);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пликатор.
pub(crate) fn n113095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111902);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 124784);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 124785);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 132572);
					match s {
						[] => true,
						['ь', 'л', 'у', 'м', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 139917);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 141538);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 142482);
													match s {
														[] => true,
														['н', s @ ..] => super::t53::n144452(s, out),
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
									super::push(out, 139918);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 141539);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 124786);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 124787);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -никатор.
pub(crate) fn n93041(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 111903);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 111904);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 124788);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 132573);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137169);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 139919);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -брикатор.
pub(crate) fn n93042(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 111905);
			match s {
				[] => true,
				['ф', s @ ..] => {
					super::push(out, 124789);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 132574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 111906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фикатор.
pub(crate) fn n93045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92062);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 111907);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 124790);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 124791);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 132575);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137170);
									s.is_empty()
								}
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
			super::push(out, 111908);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 137171);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 139920);
											s.is_empty()
										}
										_ => false,
									}
								}
								['м', s @ ..] => {
									super::push(out, 137172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', s @ ..] => {
							super::push(out, 132576);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 124792);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 124793);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 124794);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 124795);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 132577);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 137173);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 124796);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 132578);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 137174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 132579);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['а', 'т', s @ ..] => {
							super::push(out, 132580);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 132581);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					match s {
						[] => true,
						['р', 'е', 'в', '\u{1}', s @ ..] => {
							super::push(out, 132582);
							s.is_empty()
						}
						['с', 'а', 'л', 'к', '\u{1}', s @ ..] => {
							super::push(out, 132583);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 132584);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137176);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', 'л', s @ ..] => {
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 137177);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 139921);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 137178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 124797);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 132585);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137179);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 132586);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 137180);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139922);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 137181);
									match s {
										[] => true,
										['ф', s @ ..] => {
											super::push(out, 139923);
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
								['а', s @ ..] => {
									super::push(out, 137182);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 139924);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 137183);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 139925);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 141540);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 124798);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132587);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 137184);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 139926);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 111909);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 124799);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 132588);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 137185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -окатор.
pub(crate) fn n66559(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65878);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 92063);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 111910);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124800);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 132589);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137186);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 139927);
													match s {
														[] => true,
														['н', s @ ..] => super::t53::n142987(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 92064);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 111911);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ц-декламатор.
pub(crate) fn n141369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139928);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 141542);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 142484);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 143001);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рь-реформатор.
pub(crate) fn n141370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139929);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141543);
			match s {
				[] => true,
				['ц', s @ ..] => {
					super::push(out, 142485);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 143002);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ья-информатор.
pub(crate) fn n141371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139930);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 141544);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 142486);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 143003);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 143275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трансформатор.
pub(crate) fn n141372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139931);
	s.is_empty()
}

/// Words ending -кууматор.
pub(crate) fn n113118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111925);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 124811);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ат-экзаменатор.
pub(crate) fn n141373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139932);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 141545);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142487);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143004);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тр-координатор.
pub(crate) fn n141374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139933);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141546);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142488);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 143005);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -терминатор.
pub(crate) fn n133969(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 137195);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 137196);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 139934);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 141547);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 142489);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 143006);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 143277);
													match s {
														[] => true,
														['ц', s @ ..] => super::t53::n144859(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пинатор.
pub(crate) fn n93070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92085);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 111937);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 124824);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тинатор.
pub(crate) fn n93072(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92087);
	s.is_empty()
}

/// Words ending -уинатор.
pub(crate) fn n93073(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92088);
	s.is_empty()
}

/// Words ending -хинатор.
pub(crate) fn n93074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92089);
	s.is_empty()
}

/// Words ending -вакцинатор.
pub(crate) fn n93075(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 111938);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 111939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -онатор.
pub(crate) fn n66572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65891);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 92090);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 111940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 92091);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 111941);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 92092);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 111942);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 124825);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 132602);
									match s {
										[] => true,
										['ь', s @ ..] => {
											super::push(out, 137197);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 139935);
													match s {
														[] => true,
														['ю', s @ ..] => super::t53::n142994(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рнатор.
pub(crate) fn n66573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65892);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 92093);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 111943);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 124826);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 111944);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 124827);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 132603);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137198);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139936);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рефрижератор.
pub(crate) fn n138617(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 139937);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 141549);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 142491);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 143008);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 143278);
											s.is_empty()
										}
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
					super::push(out, 141550);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 142492);
							match s {
								[] => true,
								['ц', s @ ..] => {
									super::push(out, 143009);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 143279);
											s.is_empty()
										}
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
					super::push(out, 141551);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 142493);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 143010);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 143280);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 143407);
													match s {
														[] => true,
														['м', s @ ..] => super::t53::n144918(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 139938);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 141552);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 142494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мератор.
pub(crate) fn n93091(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92104);
	match s {
		[] => true,
		['у', s @ ..] => {
			super::push(out, 111955);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нератор.
pub(crate) fn n93092(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92105);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 111956);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 124837);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 132608);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 137199);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 139939);
											match s {
												[] => true,
												['е', 'т', s @ ..] => {
													super::push(out, 141553);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n143947(s, out),
														_ => false,
													}
												}
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
							super::push(out, 132609);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137200);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 139940);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 141554);
													match s {
														[] => true,
														['м', s @ ..] => super::t53::n143948(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ператор.
pub(crate) fn n93093(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92106);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 111957);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 124838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 111958);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 124839);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 132610);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 137201);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 139941);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 141555);
													match s {
														[] => true,
														['т', s @ ..] => super::t53::n143949(s, out),
														_ => false,
													}
												}
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
							super::push(out, 132611);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 137202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 132612);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 137203);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 139942);
											match s {
												[] => true,
												['х', s @ ..] => {
													super::push(out, 141556);
													match s {
														[] => true,
														['е', s @ ..] => super::t53::n143950(s, out),
														_ => false,
													}
												}
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
							super::push(out, 132613);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 137204);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 139943);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 141557);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143951(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 124840);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 132614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 111959);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 124841);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132615);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 137205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тератор.
pub(crate) fn n93094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92107);
	match s {
		[] => true,
		['и', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 124842);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 124843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 111960);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 124844);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 132616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иратор.
pub(crate) fn n66582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65900);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 92108);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 111961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 92109);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124845);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 124846);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 124847);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 132618);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 132619);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 137206);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оратор.
pub(crate) fn n66583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65901);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 92110);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 111962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 111963);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 111964);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 124848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 92111);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 111965);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124849);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 132620);
									s.is_empty()
								}
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
			super::push(out, 92112);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 111966);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 124850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 92113);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 111967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 92114);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 111968);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 124851);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 92115);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 111969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 92116);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 111970);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124852);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 132621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рратор.
pub(crate) fn n66584(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 92117);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 111971);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 92118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тратор.
pub(crate) fn n66585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65902);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 92119);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 111972);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 124853);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 132622);
									s.is_empty()
								}
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
			super::push(out, 92120);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 111973);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 124854);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 132623);
									s.is_empty()
								}
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
			super::push(out, 92121);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 111974);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 124855);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 132624);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137207);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 139944);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 111975);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 124856);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 124857);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 132625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ю', s @ ..] => {
					super::push(out, 111976);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 124858);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 132626);
									match s {
										[] => true,
										['и', '-', s @ ..] => {
											super::push(out, 137208);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139945);
													match s {
														[] => true,
														['и', s @ ..] => super::t53::n143004(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 92122);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 111977);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 124859);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 132627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уратор.
pub(crate) fn n66586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65903);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 92123);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 111978);
					match s {
						[] => true,
						['ф', s @ ..] => {
							super::push(out, 124860);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 92124);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 111979);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124861);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 132628);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 124862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 92125);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 111980);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 111981);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фратор.
pub(crate) fn n66587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65904);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 92126);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 111982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эратор.
pub(crate) fn n66588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65905);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 92127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юратор.
pub(crate) fn n66589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65906);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 92128);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 111983);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -документатор.
pub(crate) fn n141387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139946);
	s.is_empty()
}

/// Words ending -биль-эвакуатор.
pub(crate) fn n141388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139947);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141559);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142501);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -адиус-вектор.
pub(crate) fn n141393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139952);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141562);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -антиколлектор.
pub(crate) fn n141394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139953);
	s.is_empty()
}

/// Words ending -от-инспектор.
pub(crate) fn n141395(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139954);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141563);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142502);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 143016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word рыбинспектор.
pub(crate) fn n141396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139955);
	s.is_empty()
}

/// Words ending -иректор.
pub(crate) fn n93195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92203);
	match s {
		[] => true,
		['д', s @ ..] => {
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 124910);
					match s {
						[] => true,
						['н', 'а', s @ ..] => {
							super::push(out, 132657);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137223);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 139956);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141564);
													match s {
														[] => true,
														['а', s @ ..] => super::t53::n143955(s, out),
														_ => false,
													}
												}
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
							super::push(out, 132658);
							match s {
								[] => true,
								['р', 'а', '\u{1}', s @ ..] => {
									super::push(out, 137224);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 137225);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 139957);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 141565);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 124911);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 132659);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 137226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 124912);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 132660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рректор.
pub(crate) fn n93197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92205);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 112043);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 132661);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 132662);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 137227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эректор.
pub(crate) fn n93198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92206);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 112044);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 112045);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 124914);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 132663);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 137228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сектор.
pub(crate) fn n66645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65954);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 92207);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 92208);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 112046);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124915);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 132664);
									s.is_empty()
								}
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
			super::push(out, 92209);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 112047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 92210);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 112048);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 124916);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 132665);
									s.is_empty()
								}
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
			super::push(out, 92211);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 112049);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 124917);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132666);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137229);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 92212);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 112050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 92213);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 112051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 92214);
			match s {
				[] => true,
				['и', 'д', '\u{1}', s @ ..] => {
					super::push(out, 112052);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тектор.
pub(crate) fn n66646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65955);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 92215);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 112053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 92216);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 112054);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 124918);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 132667);
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137230);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139958);
													match s {
														[] => true,
														['и', s @ ..] => super::t53::n143012(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 92217);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 112055);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 132668);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137231);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 139959);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143013(s, out),
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
									super::push(out, 132669);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фектор.
pub(crate) fn n66647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65956);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 92218);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 112056);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 124919);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 132670);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137232);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 92219);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 112057);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -с-инструктор.
pub(crate) fn n141401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139960);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141568);
			match s {
				[] => true,
				['е', 'с', '\u{1}', s @ ..] => {
					super::push(out, 142506);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-инструктор.
pub(crate) fn n141402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139961);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 141569);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 142507);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 143020);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 143284);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -люз-регулятор.
pub(crate) fn n143018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141572);
	match s {
		[] => true,
		['ш', s @ ..] => {
			super::push(out, 142510);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 143023);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word финрегулятор.
pub(crate) fn n143019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141573);
	s.is_empty()
}

/// Words ending -ор-демодулятор.
pub(crate) fn n143020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141574);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 142511);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 143024);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 143285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтер-калькулятор.
pub(crate) fn n143021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141575);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 142512);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 143025);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -а-манипулятор.
pub(crate) fn n141409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139968);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141576);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 142513);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 143026);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -т-манипулятор.
pub(crate) fn n141410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139969);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141577);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 142514);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 143027);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 143286);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 143409);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word эбуллиометр.
pub(crate) fn n141413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139972);
	s.is_empty()
}

/// Words ending -тальмоманометр.
pub(crate) fn n141414(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139973);
	match s {
		[] => true,
		['ф', s @ ..] => {
			super::push(out, 141578);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рометр.
pub(crate) fn n66761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66068);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 92329);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 112148);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 92330);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 92331);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 92332);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 112149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 92333);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 112150);
					match s {
						[] => true,
						['е', 'п', 'с', '-', s @ ..] => {
							super::push(out, 124984);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 132703);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137257);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 139974);
													match s {
														[] => true,
														['с', s @ ..] => super::t53::n143025(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сометр.
pub(crate) fn n66762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66069);
	s.is_empty()
}

/// Words ending -тометр.
pub(crate) fn n66763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66070);
	match s {
		[] => true,
		['а', 'р', s @ ..] => {
			super::push(out, 92334);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 112151);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 124985);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 132704);
									s.is_empty()
								}
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
			super::push(out, 92335);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 92336);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 124986);
							s.is_empty()
						}
						['р', 'ф', 'е', 'р', '\u{1}', s @ ..] => {
							super::push(out, 124987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 112152);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 124988);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 132705);
									s.is_empty()
								}
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
			super::push(out, 92337);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					super::push(out, 112153);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 112154);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124989);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 132706);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ф', 'о', 'р', 'э', s @ ..] => {
					super::push(out, 112155);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 124990);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 132707);
									s.is_empty()
								}
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
			super::push(out, 92338);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -хометр.
pub(crate) fn n66764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66071);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 92339);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ар-консультант.
pub(crate) fn n141471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140030);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 141609);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142528);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ер-консультант.
pub(crate) fn n141472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140031);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141610);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142529);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ор-консультант.
pub(crate) fn n141473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140032);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141611);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142530);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 143032);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 143288);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 143410);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 143467);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ес-консультант.
pub(crate) fn n141474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140033);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141612);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ат-консультант.
pub(crate) fn n141475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140034);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141613);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142531);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 143033);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 143289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ет-консультант.
pub(crate) fn n141476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140035);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141614);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142532);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ец-консультант.
pub(crate) fn n141477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140036);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 141615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 142533);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 143034);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 143290);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 143411);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 143468);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ач-консультант.
pub(crate) fn n141478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140037);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141616);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 142534);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ица-адъютант.
pub(crate) fn n141479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140038);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 141617);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 142535);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -инвайронмент.
pub(crate) fn n141489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140048);
	s.is_empty()
}

/// Words ending -ектроинструмент.
pub(crate) fn n141490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140049);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 141624);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 142537);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 143035);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ажист-стилист.
pub(crate) fn n141508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140067);
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 141639);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 142547);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бочий-металлист.
pub(crate) fn n141509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140068);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 141640);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142548);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -во-монополист.
pub(crate) fn n141510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140069);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 141641);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 142549);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 143039);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -аллограф.
pub(crate) fn n114567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113359);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 125620);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 125621);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 125622);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 133028);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 137440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 133029);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 137441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иллограф.
pub(crate) fn n114568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113360);
	s.is_empty()
}

/// Words ending -олограф.
pub(crate) fn n95334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94317);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 113361);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -улограф.
pub(crate) fn n95335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94318);
	s.is_empty()
}

/// Words ending -мограф.
pub(crate) fn n69655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68931);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 94319);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 113362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 94320);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 113363);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 125624);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133030);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 137442);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 94321);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 94322);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 113364);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нограф.
pub(crate) fn n69656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68932);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 113365);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 125626);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 113366);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 113367);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 113368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 94323);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 113369);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 125627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 94324);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 94325);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 94326);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 125628);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133032);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 125629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 113370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 94327);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 113371);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 125630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 94328);
			match s {
				[] => true,
				['э', s @ ..] => {
					super::push(out, 113372);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пограф.
pub(crate) fn n69657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68933);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 94329);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 94330);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 113373);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125631);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 125632);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 133033);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137443);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140082);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143094(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 94331);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 94332);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 113374);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 125633);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133034);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 113375);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 94333);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рограф.
pub(crate) fn n69658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68934);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 94334);
			match s {
				[] => true,
				['и', 'г', '\u{1}', s @ ..] => {
					super::push(out, 113376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 94335);
			s.is_empty()
		}
		['т', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 113377);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 125634);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133035);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 113378);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 125635);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ксограф.
pub(crate) fn n69659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68935);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 94336);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 113379);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тограф.
pub(crate) fn n69660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68936);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 94337);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 113380);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 125636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 94338);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 113381);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 94339);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 113382);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 125637);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133036);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 137444);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 94340);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 113383);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 125638);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 133037);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 113384);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 125639);
							s.is_empty()
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
				['а', 'ф', s @ ..] => {
					super::push(out, 113385);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 113386);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 94341);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 113387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 94342);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 113388);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 125641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ф', s @ ..] => {
					super::push(out, 113389);
					match s {
						[] => true,
						['-', s @ ..] => {
							super::push(out, 125642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 94343);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 113390);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 125643);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 133038);
									s.is_empty()
								}
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
			super::push(out, 94344);
			match s {
				[] => true,
				['а', 'к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 125644);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 125645);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133039);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фограф.
pub(crate) fn n69661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68937);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 94345);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 113391);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хограф.
pub(crate) fn n69662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68938);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 94346);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -болотниковец.
pub(crate) fn n141526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140085);
	s.is_empty()
}

/// Words ending -анасенковец.
pub(crate) fn n138881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137462);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 140086);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 141649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 142554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оковец.
pub(crate) fn n70018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69293);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 94597);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 113540);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 125741);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133089);
									s.is_empty()
								}
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
			super::push(out, 94598);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 113541);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рковец.
pub(crate) fn n70019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69294);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 94599);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 94600);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 113542);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сковец.
pub(crate) fn n70020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69295);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 94601);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 113543);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 94602);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 113544);
					match s {
						[] => true,
						['ц', s @ ..] => {
							super::push(out, 125742);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уковец.
pub(crate) fn n70021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69296);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 94603);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 113545);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 125743);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 133090);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137463);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьковец.
pub(crate) fn n70022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69297);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 94604);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 113546);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 125744);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133091);
									s.is_empty()
								}
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
			super::push(out, 94605);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 113547);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 125745);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133092);
									s.is_empty()
								}
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
			super::push(out, 94606);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 113548);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 125746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -юковец.
pub(crate) fn n70024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69299);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 94607);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 113549);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 125747);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 133093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ой-панфиловец.
pub(crate) fn n141528(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140087);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141650);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 142555);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 143041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word тельмановец.
pub(crate) fn n141529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140088);
	s.is_empty()
}

/// The word клинтоновец.
pub(crate) fn n141530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140089);
	s.is_empty()
}

/// Words ending -владимировец.
pub(crate) fn n141531(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140090);
	s.is_empty()
}

/// Words ending -хнеднепровец.
pub(crate) fn n141532(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140091);
	s.is_empty()
}

/// Words ending -нестровец.
pub(crate) fn n127087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125787);
	match s {
		[] => true,
		['д', s @ ..] => {
			super::push(out, 133118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ыстровец.
pub(crate) fn n114811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113600);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 125788);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -уровец.
pub(crate) fn n70062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69337);
	match s {
		[] => true,
		['м', s @ ..] => {
			super::push(out, 94662);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 94663);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 113601);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыровец.
pub(crate) fn n70063(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69338);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 94664);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 113602);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -эровец.
pub(crate) fn n70064(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69339);
	match s {
		[] => true,
		['э', s @ ..] => {
			super::push(out, 94665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -юровец.
pub(crate) fn n70065(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69340);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 94666);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ана-первопроходец.
pub(crate) fn n144008(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142556);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 143042);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 143293);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 143412);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -воскормленец.
pub(crate) fn n141544(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140103);
	s.is_empty()
}

/// Words ending -благовещенец.
pub(crate) fn n141545(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140104);
	s.is_empty()
}

/// Words ending -сист-ленинец.
pub(crate) fn n141546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140105);
	match s {
		[] => true,
		['к', s @ ..] => {
			super::push(out, 141656);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 142558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ок-месхетинец.
pub(crate) fn n141547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140106);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 141657);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 142559);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 143044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ах-августинец.
pub(crate) fn n141548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140107);
	match s {
		[] => true,
		['н', s @ ..] => {
			super::push(out, 141658);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 142560);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 143045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ородулихинец.
pub(crate) fn n141549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140108);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 141659);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ватель.
pub(crate) fn n71859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71111);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96087);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 114491);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126266);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133341);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137566);
											match s {
												[] => true,
												['р', 'п', '-', s @ ..] => {
													super::push(out, 140132);
													match s {
														[] => true,
														['р', s @ ..] => super::t53::n143116(s, out),
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
									super::push(out, 133342);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137567);
											s.is_empty()
										}
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
					super::push(out, 114492);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 126267);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 114493);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 126268);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133344);
									s.is_empty()
								}
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
			super::push(out, 96088);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 126269);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133345);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137568);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126270);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133346);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 114494);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126271);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133347);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 114495);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 126272);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133348);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137569);
											s.is_empty()
										}
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
					super::push(out, 114496);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126273);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133349);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137570);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 126274);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133350);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 137571);
											s.is_empty()
										}
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
					super::push(out, 114497);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126275);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133351);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 133352);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 126276);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133353);
									match s {
										[] => true,
										['б', 'о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 140133);
													s.is_empty()
												}
												['о', s @ ..] => {
													match s {
														[] => true,
														['д', s @ ..] => super::t53::n143117(s, out),
														['т', s @ ..] => super::t53::n143118(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => super::t53::n115712(s, out),
				['ц', s @ ..] => super::t53::n115713(s, out),
				['ч', s @ ..] => super::t53::n115714(s, out),
				['ш', s @ ..] => super::t53::n115715(s, out),
				['щ', s @ ..] => super::t53::n115716(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t53::n97116(s, out),
		['к', s @ ..] => super::t53::n97117(s, out),
		['о', s @ ..] => super::t53::n97118(s, out),
		['у', s @ ..] => super::t53::n97119(s, out),
		['ы', s @ ..] => super::t53::n97120(s, out),
		_ => false,
	}
}

/// Words ending -ередвигатель.
pub(crate) fn n141627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140183);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 141701);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 142584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -раздвигатель.
pub(crate) fn n141628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140184);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 141702);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оросжигатель.
pub(crate) fn n141629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140185);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 141703);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 142585);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -вымогатель.
pub(crate) fn n134813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133443);
	match s {
		[] => true,
		['-', s @ ..] => {
			super::push(out, 137657);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 140186);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 141704);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 142586);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 143061);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 143302);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ргатель.
pub(crate) fn n97125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96098);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 114540);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 126360);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ругатель.
pub(crate) fn n115755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114541);
	s.is_empty()
}

/// Words ending -ягатель.
pub(crate) fn n97127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96100);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 114542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -датель.
pub(crate) fn n71861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71113);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 96101);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 96102);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 96103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -жатель.
pub(crate) fn n71862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71114);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96104);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114543);
					s.is_empty()
				}
				['р', s @ ..] => {
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126361);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133444);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137658);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 126362);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 126363);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133445);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137659);
											s.is_empty()
										}
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
					super::push(out, 114544);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126364);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 133446);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137660);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96105);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114545);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 126365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96106);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114546);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 96107);
			match s {
				[] => true,
				['е', 'д', s @ ..] => {
					match s {
						[] => true,
						['е', 'ц', s @ ..] => {
							super::push(out, 126367);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 133447);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137661);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140187);
													s.is_empty()
												}
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
								['б', s @ ..] => {
									super::push(out, 133448);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 137662);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 140188);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n143153(s, out),
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
									super::push(out, 133449);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137663);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 140189);
													match s {
														[] => true,
														['у', s @ ..] => super::t53::n143154(s, out),
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
									super::push(out, 133450);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 137664);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 140190);
													match s {
														[] => true,
														['щ', '\u{1}', s @ ..] => super::t53::n143155(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ш', s @ ..] => {
											super::push(out, 137665);
											match s {
												[] => true,
												['у', s @ ..] => {
													super::push(out, 140191);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 137666);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 140192);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['н', 'и', s @ ..] => {
									super::push(out, 133451);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137667);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 140193);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 133452);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 133453);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 137668);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96108);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 114547);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 126368);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 133454);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137669);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96109);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 114548);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -затель.
pub(crate) fn n71863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96110);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114549);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126370);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133455);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 133456);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137670);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 140194);
													match s {
														[] => true,
														['п', s @ ..] => super::t53::n143156(s, out),
														_ => false,
													}
												}
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
							super::push(out, 126371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 96111);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 114550);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 96112);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 114551);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126373);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 133457);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137671);
											s.is_empty()
										}
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
					super::push(out, 114552);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126374);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -катель.
pub(crate) fn n71864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71116);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96113);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 114553);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 96114);
			match s {
				[] => true,
				['л', 'в', 'з', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126375);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133458);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', '\u{1}', s @ ..] => {
							super::push(out, 126376);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 114554);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126377);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 133459);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 114555);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126378);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 133460);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137672);
											s.is_empty()
										}
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
					super::push(out, 114556);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126379);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 126380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 114557);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 126381);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133461);
									s.is_empty()
								}
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
			super::push(out, 96115);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 114558);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 126382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 96116);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 114559);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 114560);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 126383);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 133462);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 137673);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 140195);
													s.is_empty()
												}
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
							super::push(out, 126384);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 126385);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 133463);
									s.is_empty()
								}
								['д', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 137674);
											match s {
												[] => true,
												['л', s @ ..] => {
													super::push(out, 140196);
													match s {
														[] => true,
														['к', s @ ..] => super::t53::n143157(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 137675);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 137676);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 140197);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 137677);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 133464);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 133465);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 137678);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 140198);
													match s {
														[] => true,
														['т', s @ ..] => super::t53::n143158(s, out),
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
									super::push(out, 133466);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 137679);
											match s {
												[] => true,
												['м', s @ ..] => {
													super::push(out, 140199);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => super::t53::n134838(s, out),
								['с', s @ ..] => {
									super::push(out, 133468);
									s.is_empty()
								}
								['т', s @ ..] => super::t53::n134840(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t53::n115778(s, out),
				['у', s @ ..] => super::t53::n115779(s, out),
				['ы', s @ ..] => super::t53::n115780(s, out),
				_ => false,
			}
		}
		['у', s @ ..] => super::t53::n97144(s, out),
		['ы', s @ ..] => super::t53::n97145(s, out),
		_ => false,
	}
}

/// Words ending -латель.
pub(crate) fn n71865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71117);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 96119);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 114566);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 114567);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -матель.
pub(crate) fn n71866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71118);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 96120);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 114568);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 126393);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 133474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 114569);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126394);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 126395);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 133476);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 137683);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 126396);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133477);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 137684);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 137685);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96121);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 114570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -натель.
pub(crate) fn n71867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71119);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 96122);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 114571);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126398);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133478);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 137686);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 96123);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 114572);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -патель.
pub(crate) fn n71868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71120);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 96124);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114573);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 126400);
							s.is_empty()
						}
						['о', 'б', s @ ..] => {
							super::push(out, 126401);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 133479);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 137687);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 140205);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 114574);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 126402);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 96125);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 114575);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 126403);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 133480);
									s.is_empty()
								}
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
			super::push(out, 96126);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ратель.
pub(crate) fn n71869(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71121);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 96127);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 96128);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 114576);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 126404);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 133481);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 137688);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 140206);
													match s {
														[] => true,
														['о', s @ ..] => super::t53::n143164(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 140207);
													match s {
														[] => true,
														['е', s @ ..] => super::t53::n143165(s, out),
														_ => false,
													}
												}
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
							super::push(out, 126405);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 133482);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 126406);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['-', s @ ..] => {
											super::push(out, 137689);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 140208);
													match s {
														[] => true,
														['и', s @ ..] => super::t53::n143166(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 137690);
											match s {
												[] => true,
												['г', s @ ..] => {
													super::push(out, 140209);
													s.is_empty()
												}
												['н', s @ ..] => {
													super::push(out, 140210);
													match s {
														[] => true,
														['е', s @ ..] => super::t53::n143167(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 114577);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 126407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 114578);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 114579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 96129);
			s.is_empty()
		}
		_ => false,
	}
}

