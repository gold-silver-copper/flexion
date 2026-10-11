use flexion::Path;

/// Words ending -сать.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 1916);
					match s {
						[] => true,
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 13499);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 21990);
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
				['ж', s @ ..] => {
					super::push(out, 1917);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 1918);
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
		['е', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 1919);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6135);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13500);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 13501);
									match s {
										[] => true,
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
				['т', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6136);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 6137);
							match s {
								[] => true,
								['о', s @ ..] => {
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
						['в', '\u{1}', s @ ..] => {
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6138);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13502);
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
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 6139);
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
						['ы', s @ ..] => {
							super::push(out, 6140);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13503);
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
				['ч', s @ ..] => {
					super::push(out, 1921);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6141);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6142);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 13504);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 21991);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6143);
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
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 6144);
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
						['ы', s @ ..] => {
							super::push(out, 6145);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13505);
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
		['и', s @ ..] => super::t4::n385(s, out),
		['к', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 1928);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 6160);
							match s {
								[] => true,
								['\u{1}', s @ ..] => super::t4::n17916(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => super::t4::n2034(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t4::n387(s, out),
		['о', s @ ..] => super::t4::n388(s, out),
		['с', s @ ..] => super::t4::n389(s, out),
		['у', s @ ..] => super::t4::n390(s, out),
		['ы', s @ ..] => super::t4::n391(s, out),
		['я', s @ ..] => super::t4::n392(s, out),
		_ => false,
	}
}

/// Words ending -тать.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 389);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 1942);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 6190);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13553);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 13554);
									match s {
										[] => true,
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
								['у', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 13555);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 22008);
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
							super::push(out, 6191);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13556);
									s.is_empty()
								}
								['а', s @ ..] => super::t4::n17996(s, out),
								['б', s @ ..] => super::t4::n17997(s, out),
								['д', s @ ..] => super::t4::n17998(s, out),
								['е', s @ ..] => super::t4::n17999(s, out),
								['о', s @ ..] => super::t4::n18000(s, out),
								['с', s @ ..] => super::t4::n18001(s, out),
								['т', s @ ..] => super::t4::n18002(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => super::t4::n2048(s, out),
				['л', s @ ..] => super::t4::n2049(s, out),
				['р', s @ ..] => super::t4::n2050(s, out),
				['ч', s @ ..] => super::t4::n2051(s, out),
				['ш', s @ ..] => super::t4::n2052(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t4::n394(s, out),
		['и', s @ ..] => super::t4::n395(s, out),
		['к', s @ ..] => super::t4::n396(s, out),
		['л', s @ ..] => super::t4::n397(s, out),
		['о', s @ ..] => super::t4::n398(s, out),
		['п', s @ ..] => super::t4::n399(s, out),
		['р', s @ ..] => super::t4::n400(s, out),
		['с', s @ ..] => super::t4::n401(s, out),
		['у', s @ ..] => super::t4::n402(s, out),
		['х', s @ ..] => super::t5::n403(s, out),
		['ч', s @ ..] => super::t5::n404(s, out),
		['ш', s @ ..] => super::t5::n405(s, out),
		['ы', s @ ..] => super::t5::n406(s, out),
		['я', s @ ..] => super::t5::n407(s, out),
		_ => false,
	}
}

/// Words ending -фать.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['д', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 406);
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

/// Words ending -хать.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2003);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2004);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6363);
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
					super::push(out, 2005);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6364);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6365);
							match s {
								[] => true,
								['б', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22184);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 22185);
											match s {
												[] => true,
												['з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 13722);
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
							super::push(out, 6366);
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
					super::push(out, 2006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6367);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6368);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6369);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 13723);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22186);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 13724);
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
							super::push(out, 6370);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13725);
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
							super::push(out, 6371);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 13726);
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
							super::push(out, 6372);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13727);
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
						['и', s @ ..] => {
							super::push(out, 6373);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 13728);
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
						['о', s @ ..] => {
							super::push(out, 6374);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 13729);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 13730);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 13731);
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
							super::push(out, 6375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 6376);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13732);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 6377);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 6378);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13733);
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
				['о', s @ ..] => super::t5::n2117(s, out),
				['п', s @ ..] => super::t5::n2118(s, out),
				['р', s @ ..] => super::t5::n2119(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t5::n412(s, out),
		['и', s @ ..] => super::t5::n413(s, out),
		['о', s @ ..] => super::t5::n414(s, out),
		['п', s @ ..] => super::t5::n415(s, out),
		['р', s @ ..] => super::t5::n416(s, out),
		['у', s @ ..] => super::t5::n417(s, out),
		['ч', s @ ..] => super::t5::n418(s, out),
		['ы', s @ ..] => super::t5::n419(s, out),
		['э', s @ ..] => super::t5::n420(s, out),
		['ю', s @ ..] => super::t5::n421(s, out),
		['я', s @ ..] => super::t5::n422(s, out),
		_ => false,
	}
}

/// Words ending -цать.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 419);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2066);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6481);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6482);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 6483);
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
					super::push(out, 2067);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6484);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13823);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 22223);
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
				['м', s @ ..] => {
					super::push(out, 2068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6485);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6486);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 13824);
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
			super::push(out, 420);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2069);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 6487);
							match s {
								[] => true,
								['с', 'о', 'в', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 22224);
											match s {
												[] => true,
												['з', s @ ..] => {
													super::push(out, 30345);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n65451(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2070);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6488);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 13825);
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
							super::push(out, 6489);
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
		['м', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 2071);
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
			super::push(out, 422);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2072);
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
			super::push(out, 423);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 2073);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6490);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 13826);
									match s {
										[] => true,
										['с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 30346);
													match s {
														[] => true,
														['п', s @ ..] => super::t5::n65452(s, out),
														_ => false,
													}
												}
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
							super::push(out, 6491);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 13827);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 13828);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 22225);
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
									super::push(out, 13829);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22226);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 22227);
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
								['т', s @ ..] => {
									super::push(out, 13830);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 22228);
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
		['с', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2074);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2075);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 6492);
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
					super::push(out, 2076);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6493);
							match s {
								[] => true,
								['о', s @ ..] => super::t5::n18498(s, out),
								_ => false,
							}
						}
						['п', s @ ..] => super::t5::n7678(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => super::t5::n429(s, out),
		['я', s @ ..] => super::t5::n430(s, out),
		_ => false,
	}
}

/// Words ending -чать.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 427);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 2080);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 2081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6496);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6497);
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
							super::push(out, 6498);
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
							super::push(out, 6499);
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
						['ы', s @ ..] => {
							super::push(out, 6500);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 13835);
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
					super::push(out, 2082);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6501);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 22232);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 30348);
													match s {
														[] => true,
														['е', 'п', '\u{1}', s @ ..] => super::t5::n65454(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['з', s @ ..] => {
											super::push(out, 22233);
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
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2083);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6502);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 6503);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 13836);
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
						['з', s @ ..] => {
							super::push(out, 6504);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 13837);
									match s {
										[] => true,
										['н', s @ ..] => {
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
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22234);
											s.is_empty()
										}
										['б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2084);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 6505);
							match s {
								[] => true,
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
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2085);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6506);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6507);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => super::t5::n7705(s, out),
						['с', s @ ..] => super::t5::n7706(s, out),
						['у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['ы', s @ ..] => super::t5::n7708(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t5::n432(s, out),
		['е', s @ ..] => super::t5::n433(s, out),
		['и', s @ ..] => super::t5::n434(s, out),
		['л', s @ ..] => super::t5::n435(s, out),
		['м', s @ ..] => super::t5::n436(s, out),
		['н', s @ ..] => super::t5::n437(s, out),
		['о', s @ ..] => super::t5::n438(s, out),
		['п', s @ ..] => super::t5::n439(s, out),
		['р', s @ ..] => super::t5::n440(s, out),
		['с', s @ ..] => super::t5::n441(s, out),
		['у', s @ ..] => super::t5::n442(s, out),
		['ы', s @ ..] => super::t5::n443(s, out),
		['ь', s @ ..] => super::t5::n444(s, out),
		['ю', s @ ..] => super::t5::n445(s, out),
		['я', s @ ..] => super::t5::n446(s, out),
		_ => false,
	}
}

/// Words ending -шать.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2135);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'м', 'и', 'а', 'з', 'в', '\u{1}', s @ ..] => {
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
				['л', s @ ..] => {
					super::push(out, 2136);
					match s {
						[] => true,
						['г', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 14010);
									match s {
										[] => true,
										['о', 'в', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['е', 'р', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', 'р', 'п', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n65781(s, out),
														['о', 'м', 'а', 'с', '\u{1}', s @ ..] => super::t5::n65782(s, out),
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
									super::push(out, 14011);
									match s {
										[] => true,
										['р', 'п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 30525);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t5::n65783(s, out),
														['н', 'о', 'п', '\u{1}', s @ ..] => super::t5::n65784(s, out),
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
										['\u{1}', s @ ..] => {
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
					super::push(out, 2137);
					match s {
						[] => true,
						['к', 'у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
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
		['д', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6596);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6597);
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
						['е', s @ ..] => {
							super::push(out, 6598);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 6599);
							match s {
								[] => true,
								['а', 'р', s @ ..] => super::t5::n18786(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t5::n7848(s, out),
						['с', '\u{1}', s @ ..] => super::t5::n7849(s, out),
						['т', 'о', '\u{1}', s @ ..] => super::t5::n7850(s, out),
						['у', s @ ..] => super::t5::n7851(s, out),
						_ => false,
					}
				}
				['м', s @ ..] => super::t5::n2253(s, out),
				['п', s @ ..] => super::t5::n2254(s, out),
				['р', s @ ..] => super::t5::n2255(s, out),
				['т', s @ ..] => super::t5::n2256(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n450(s, out),
		['м', s @ ..] => super::t5::n451(s, out),
		['о', s @ ..] => super::t5::n452(s, out),
		['р', s @ ..] => super::t5::n453(s, out),
		['т', s @ ..] => super::t5::n454(s, out),
		['у', s @ ..] => super::t5::n455(s, out),
		['ч', s @ ..] => super::t5::n456(s, out),
		['ы', s @ ..] => super::t5::n457(s, out),
		['ь', s @ ..] => super::t5::n458(s, out),
		_ => false,
	}
}

/// Words ending -щать.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 455);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['з', 'о', s @ ..] => {
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
						['с', s @ ..] => {
							match s {
								[] => true,
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'е', 'п', '\u{1}', s @ ..] => {
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
				['м', 'у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2170);
					match s {
						[] => true,
						['с', s @ ..] => {
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
										['о', 'д', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', 'а', 'р', '\u{1}', s @ ..] => {
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
				['р', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['а', 'н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 6647);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
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
						['в', s @ ..] => {
							super::push(out, 6648);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 14050);
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
													super::push(out, 30530);
													match s {
														[] => true,
														['п', s @ ..] => super::t5::n65789(s, out),
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
									super::push(out, 14051);
									match s {
										[] => true,
										['л', 'о', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 22471);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30531);
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
								['т', 'о', s @ ..] => {
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
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 6649);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 6650);
							match s {
								[] => true,
								['о', 'с', s @ ..] => {
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
						['т', s @ ..] => super::t5::n7932(s, out),
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 2172);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 456);
			s.is_empty()
		}
		['е', s @ ..] => super::t5::n461(s, out),
		['и', s @ ..] => super::t5::n462(s, out),
		['м', s @ ..] => super::t5::n463(s, out),
		['о', s @ ..] => super::t5::n464(s, out),
		['р', 'и', 'м', 'б', 'о', '\u{1}', s @ ..] => super::t5::n465(s, out),
		['у', s @ ..] => super::t5::n466(s, out),
		['ы', s @ ..] => super::t5::n467(s, out),
		['ь', s @ ..] => super::t5::n468(s, out),
		['я', s @ ..] => super::t5::n469(s, out),
		_ => false,
	}
}

/// The word засвербеть.
pub(crate) fn n51284(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word поскорбеть.
pub(crate) fn n51285(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -рябеть.
pub(crate) fn n2326(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6686);
			s.is_empty()
		}
		['а', 'з', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['о', 'п', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -веть.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2200);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6687);
							match s {
								[] => true,
								['р', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 2201);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14078);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14079);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 22489);
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
								['е', s @ ..] => {
									super::push(out, 14080);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 22490);
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
								['и', s @ ..] => {
									super::push(out, 14081);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 22491);
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
								['о', s @ ..] => {
									super::push(out, 14082);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 22492);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 30541);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n65798(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 22493);
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
								['у', s @ ..] => {
									super::push(out, 14083);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30542);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 30543);
													match s {
														[] => true,
														['з', '\u{1}', s @ ..] => super::t5::n65799(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2202);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 6688);
							match s {
								[] => true,
								['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2203);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 6689);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14084);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14085);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22494);
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
							super::push(out, 6690);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 6691);
							match s {
								[] => true,
								['м', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 2204);
					match s {
						[] => true,
						['р', 'у', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14086);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14087);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2205);
					match s {
						[] => true,
						['р', 'е', 'ш', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14088);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14089);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2206);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 6692);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22495);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22496);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t5::n478(s, out),
		['з', s @ ..] => {
			super::push(out, 473);
			match s {
				[] => true,
				['е', s @ ..] => super::t5::n2341(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n480(s, out),
		['л', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['о', s @ ..] => super::t5::n2351(s, out),
				_ => false,
			}
		}
		['о', s @ ..] => super::t5::n482(s, out),
		['т', s @ ..] => super::t5::n483(s, out),
		['я', s @ ..] => super::t5::n484(s, out),
		_ => false,
	}
}

/// Words ending -геть.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['ф', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6746);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 6747);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6748);
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

/// Words ending -деть.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 2239);
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
				['л', s @ ..] => {
					super::push(out, 2240);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 6749);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14153);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14154);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
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
										['с', s @ ..] => {
											super::push(out, 22530);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14155);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14156);
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
					super::push(out, 2241);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2242);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6750);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 14157);
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
							super::push(out, 6751);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14158);
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
					super::push(out, 2243);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 6752);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 14159);
									s.is_empty()
								}
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
			super::push(out, 482);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2244);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 2245);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 6753);
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
					super::push(out, 2246);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6754);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 14160);
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
						['р', s @ ..] => {
							super::push(out, 6755);
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
		['в', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2247);
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
		['е', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2248);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6756);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6757);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14161);
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
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6758);
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
				['с', s @ ..] => {
					super::push(out, 2249);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6759);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6760);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14162);
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
		['з', s @ ..] => super::t5::n492(s, out),
		['и', s @ ..] => super::t5::n493(s, out),
		['л', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2260);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6790);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14190);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => super::t5::n19146(s, out),
								_ => false,
							}
						}
						['г', s @ ..] => super::t5::n8146(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t5::n495(s, out),
		['о', s @ ..] => super::t5::n496(s, out),
		['р', s @ ..] => super::t5::n497(s, out),
		['с', s @ ..] => super::t5::n498(s, out),
		['т', s @ ..] => super::t5::n499(s, out),
		['у', s @ ..] => super::t5::n500(s, out),
		['ь', s @ ..] => super::t5::n501(s, out),
		['ю', s @ ..] => super::t5::n502(s, out),
		['я', s @ ..] => super::t5::n503(s, out),
		_ => false,
	}
}

/// Words ending -жеть.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2291);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14219);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14220);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14221);
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
				['н', s @ ..] => {
					super::push(out, 2292);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 6828);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2293);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 6829);
							match s {
								[] => true,
								['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 500);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 6830);
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
				['г', s @ ..] => {
					super::push(out, 2295);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 6831);
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
				['н', s @ ..] => {
					super::push(out, 2296);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2297);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 6833);
							match s {
								[] => true,
								['с', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 501);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 2298);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6834);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6835);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14222);
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
				['ч', s @ ..] => {
					super::push(out, 2299);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6836);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6837);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14223);
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
			super::push(out, 502);
			match s {
				[] => true,
				['р', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6838);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6839);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6840);
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
		['ю', s @ ..] => {
			super::push(out, 503);
			match s {
				[] => true,
				['д', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6841);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6842);
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

/// Words ending -зеть.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['л', 'г', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6843);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6844);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14224);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 14225);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22559);
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
		['и', s @ ..] => {
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6845);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6846);
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
				['ш', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6847);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6848);
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
			super::push(out, 505);
			match s {
				[] => true,
				['т', 'о', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['е', 'м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6849);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6850);
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
				['о', 'б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6851);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6852);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -леть.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 507);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2301);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 2302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6853);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 6854);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14226);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 14227);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22560);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22561);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 30570);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n65821(s, out),
														_ => false,
													}
												}
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
							super::push(out, 6855);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 6856);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14228);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2303);
					match s {
						[] => true,
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2304);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6857);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6858);
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
		['б', s @ ..] => {
			super::push(out, 508);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 509);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2306);
					match s {
						[] => true,
						['д', s @ ..] => {
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
		['г', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['н', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14229);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 14230);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', 'и', '\u{1}', s @ ..] => {
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
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 6859);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14231);
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
						['м', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14232);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14233);
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
						['р', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14234);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14235);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14236);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['о', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6860);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6861);
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
		['е', s @ ..] => super::t5::n519(s, out),
		['з', s @ ..] => {
			super::push(out, 513);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2316);
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
			super::push(out, 514);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 2317);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6874);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6875);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14250);
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
				['с', s @ ..] => super::t5::n2459(s, out),
				['т', s @ ..] => super::t5::n2460(s, out),
				['х', s @ ..] => super::t5::n2461(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => super::t5::n522(s, out),
		['м', s @ ..] => super::t5::n523(s, out),
		['о', s @ ..] => super::t5::n524(s, out),
		['п', s @ ..] => super::t5::n525(s, out),
		['с', s @ ..] => super::t5::n526(s, out),
		['т', s @ ..] => super::t5::n527(s, out),
		['х', s @ ..] => super::t5::n528(s, out),
		['ш', s @ ..] => super::t5::n529(s, out),
		['ы', s @ ..] => super::t5::n530(s, out),
		_ => false,
	}
}

/// Words ending -меть.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 2343);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6912);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6913);
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
			super::push(out, 525);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2344);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6914);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 6915);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 6916);
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
					super::push(out, 2345);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 6917);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14265);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14266);
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
									match s {
										[] => true,
										['в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'в', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14267);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2346);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2347);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2348);
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
		['о', s @ ..] => {
			super::push(out, 527);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2349);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 6918);
							match s {
								[] => true,
								['е', 'б', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2350);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14268);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14269);
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
		['с', s @ ..] => {
			super::push(out, 528);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2351);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2352);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 6919);
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
			super::push(out, 529);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2353);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6920);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 14270);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22573);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22574);
											match s {
												[] => true,
												['д', s @ ..] => {
													super::push(out, 30572);
													match s {
														[] => true,
														['е', 'н', '\u{1}', s @ ..] => super::t5::n65822(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 22575);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['ы', s @ ..] => {
											super::push(out, 22576);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 30573);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t5::n65823(s, out),
														_ => false,
													}
												}
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
							super::push(out, 6921);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 14271);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22577);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22578);
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
				['о', s @ ..] => {
					super::push(out, 2355);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 6922);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t5::n2502(s, out),
				['ч', s @ ..] => super::t5::n2503(s, out),
				['ш', s @ ..] => super::t5::n2504(s, out),
				_ => false,
			}
		}
		['ы', s @ ..] => super::t5::n537(s, out),
		['ю', s @ ..] => super::t5::n538(s, out),
		_ => false,
	}
}

/// Words ending -неть.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2362);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 6931);
							match s {
								[] => true,
								['к', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2363);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 6932);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['а', 'с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14277);
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
							super::push(out, 6933);
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
			super::push(out, 533);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2365);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 6934);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14278);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 14279);
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
						['с', s @ ..] => {
							super::push(out, 6935);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14280);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2366);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14281);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14282);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14283);
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
						['л', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14284);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14285);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22581);
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
					super::push(out, 2367);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 6936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14286);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 14287);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 22582);
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
								['о', s @ ..] => {
									super::push(out, 14288);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22583);
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
							super::push(out, 6937);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', 'л', 'о', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6938);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6939);
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
				['у', 'к', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6940);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 6941);
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
				['ю', s @ ..] => {
					super::push(out, 2368);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 6942);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14290);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14291);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22584);
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
		['е', s @ ..] => super::t5::n541(s, out),
		['ж', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 6959);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', 'в', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14320);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14321);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14322);
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
				['е', s @ ..] => {
					super::push(out, 2380);
					match s {
						[] => true,
						['н', s @ ..] => super::t5::n8375(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => super::t5::n543(s, out),
		['и', s @ ..] => super::t5::n544(s, out),
		['й', 'о', 'р', 'т', 'с', s @ ..] => super::t5::n545(s, out),
		['к', s @ ..] => super::t5::n546(s, out),
		['л', s @ ..] => super::t5::n547(s, out),
		['м', s @ ..] => super::t5::n548(s, out),
		['н', 'и', 'л', 'д', s @ ..] => super::t5::n549(s, out),
		['о', s @ ..] => super::t5::n550(s, out),
		['п', 'у', 'р', 'к', s @ ..] => super::t5::n551(s, out),
		['р', s @ ..] => super::t5::n552(s, out),
		['с', s @ ..] => super::t5::n553(s, out),
		['т', s @ ..] => super::t5::n554(s, out),
		['у', s @ ..] => super::t5::n555(s, out),
		['ч', s @ ..] => super::t5::n556(s, out),
		['ш', s @ ..] => super::t5::n557(s, out),
		['щ', s @ ..] => super::t5::n558(s, out),
		['ь', s @ ..] => super::t5::n559(s, out),
		['ю', s @ ..] => super::t5::n560(s, out),
		['я', s @ ..] => super::t5::n561(s, out),
		_ => false,
	}
}

/// Words ending -оеть.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['п', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2424);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -петь.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 553);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 2425);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2426);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2427);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 7005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14373);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									super::push(out, 14374);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14375);
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
								['с', 'в', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 555);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2428);
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
		['е', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2429);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7006);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14376);
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
							super::push(out, 7007);
							match s {
								[] => true,
								['в', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22626);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22627);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 22628);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7008);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7009);
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
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7010);
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
							super::push(out, 7011);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'в', '\u{1}', s @ ..] => {
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
							super::push(out, 7012);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 14377);
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
				['р', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 7013);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22629);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 22630);
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
										['о', s @ ..] => super::t5::n34690(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['п', s @ ..] => super::t5::n8467(s, out),
						['х', s @ ..] => super::t5::n8468(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t5::n2589(s, out),
				['ш', s @ ..] => super::t5::n2590(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t5::n568(s, out),
		['о', s @ ..] => super::t5::n569(s, out),
		['р', s @ ..] => super::t5::n570(s, out),
		['с', s @ ..] => super::t5::n571(s, out),
		['т', s @ ..] => super::t5::n572(s, out),
		['у', s @ ..] => super::t6::n573(s, out),
		['ы', s @ ..] => super::t6::n574(s, out),
		_ => false,
	}
}

/// Words ending -реть.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['ж', 'з', 'а', 'р', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 2457);
					match s {
						[] => true,
						['о', 'ф', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14399);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14400);
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
					super::push(out, 2458);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14401);
									s.is_empty()
								}
								['а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14402);
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
								['у', s @ ..] => {
									super::push(out, 14403);
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
								['ы', s @ ..] => {
									super::push(out, 14404);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 22636);
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
		['б', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['а', 'р', 'х', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7043);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7044);
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
				['о', s @ ..] => {
					super::push(out, 2459);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14405);
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 14406);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14407);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 22637);
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
		['в', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2461);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2462);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 7046);
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
					super::push(out, 2463);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', 'в', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 2464);
					match s {
						[] => true,
						['р', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 7047);
							match s {
								[] => true,
								['о', s @ ..] => {
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
						['д', s @ ..] => {
							super::push(out, 7048);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14409);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14410);
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
						['з', s @ ..] => {
							super::push(out, 7049);
							match s {
								[] => true,
								['а', 'р', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22638);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 30590);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t6::n65838(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
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
						['с', s @ ..] => {
							super::push(out, 7050);
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
						['т', s @ ..] => super::t6::n8523(s, out),
						_ => false,
					}
				}
				['у', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t6::n579(s, out),
		['е', s @ ..] => super::t6::n580(s, out),
		['з', s @ ..] => super::t6::n581(s, out),
		['и', s @ ..] => super::t6::n582(s, out),
		['к', s @ ..] => super::t6::n583(s, out),
		['м', s @ ..] => super::t6::n584(s, out),
		['о', s @ ..] => super::t6::n585(s, out),
		['п', s @ ..] => super::t6::n586(s, out),
		['т', s @ ..] => super::t6::n587(s, out),
		['у', s @ ..] => super::t6::n588(s, out),
		['ы', s @ ..] => super::t6::n589(s, out),
		_ => false,
	}
}

/// Words ending -сеть.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 580);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2502);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 7142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 581);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7143);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7144);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 14475);
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
							super::push(out, 7145);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7146);
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
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 582);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7147);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7148);
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
					super::push(out, 2505);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7149);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2506);
					match s {
						[] => true,
						['п', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2507);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 7150);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14476);
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
							super::push(out, 7151);
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
		['ы', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 2508);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7152);
							s.is_empty()
						}
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 7153);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7154);
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

/// Words ending -теть.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2509);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7155);
							match s {
								[] => true,
								['о', 'г', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2510);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 14477);
									match s {
										[] => true,
										['у', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22681);
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
				['г', s @ ..] => {
					super::push(out, 2511);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7156);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 14478);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', s @ ..] => {
											super::push(out, 22682);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 30600);
													match s {
														[] => true,
														['р', '\u{1}', s @ ..] => super::t6::n65854(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 22683);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30601);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 30602);
													match s {
														[] => true,
														['о', s @ ..] => super::t6::n65855(s, out),
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
									super::push(out, 14479);
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
				['д', s @ ..] => {
					super::push(out, 2512);
					match s {
						[] => true,
						['о', 'р', 'о', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14480);
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
					super::push(out, 2513);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 7157);
							match s {
								[] => true,
								['о', 'х', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 2514);
					match s {
						[] => true,
						['с', 'о', 'к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14481);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14482);
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
						['х', s @ ..] => {
							super::push(out, 7158);
							match s {
								[] => true,
								['о', 'л', 'а', 'з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 2515);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7159);
							match s {
								[] => true,
								['ф', s @ ..] => {
									super::push(out, 14483);
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
						['х', s @ ..] => {
							super::push(out, 7160);
							match s {
								[] => true,
								['о', 'м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['б', s @ ..] => {
											super::push(out, 22684);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 30603);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t6::n65856(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2516);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7161);
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
				['с', s @ ..] => {
					super::push(out, 2517);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7162);
							match s {
								[] => true,
								['л', 'о', 'в', s @ ..] => super::t6::n19702(s, out),
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 7163);
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
				['х', s @ ..] => super::t6::n2685(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t6::n597(s, out),
		['л', s @ ..] => super::t6::n598(s, out),
		['о', s @ ..] => super::t6::n599(s, out),
		['п', s @ ..] => super::t6::n600(s, out),
		['р', s @ ..] => super::t6::n601(s, out),
		['с', s @ ..] => super::t6::n602(s, out),
		['у', s @ ..] => super::t6::n603(s, out),
		['х', s @ ..] => super::t6::n604(s, out),
		['ы', s @ ..] => super::t6::n605(s, out),
		['ю', s @ ..] => super::t6::n606(s, out),
		['я', s @ ..] => super::t6::n607(s, out),
		_ => false,
	}
}

/// Words ending -уеть.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 598);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2546);
					s.is_empty()
				}
				['и', 'р', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2547);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['т', 'с', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хеть.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['л', 'п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7201);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7202);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14531);
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

/// Words ending -четь.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 2548);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14532);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14533);
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
		['о', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 2549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 2550);
					match s {
						[] => true,
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
			super::push(out, 604);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2551);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шеть.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 2552);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7203);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 14534);
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
					super::push(out, 2553);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7204);
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
		['м', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2554);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 2555);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2556);
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
		['о', s @ ..] => {
			super::push(out, 607);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 2557);
					match s {
						[] => true,
						['о', 'х', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14535);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14536);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14537);
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
		['у', s @ ..] => {
			super::push(out, 608);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 2558);
					match s {
						[] => true,
						['з', 'е', 'б', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'н', 'в', 'а', 'р', 'о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 2559);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 7205);
							match s {
								[] => true,
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
		['ь', s @ ..] => {
			super::push(out, 610);
			match s {
				[] => true,
				['н', 'о', 'т', 'о', 'п', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ыеть.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2560);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -бить.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 620);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 621);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7216);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7217);
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
				['в', s @ ..] => {
					super::push(out, 2576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7218);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 7219);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 7220);
							match s {
								[] => true,
								['р', 'е', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 7221);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14542);
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
							super::push(out, 7222);
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
					super::push(out, 2577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2578);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 7223);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 14543);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 22711);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 7224);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14544);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14545);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 22712);
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
									super::push(out, 14546);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
									super::push(out, 14547);
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
				['н', s @ ..] => {
					super::push(out, 2579);
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
				['р', s @ ..] => {
					super::push(out, 2580);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 7225);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14548);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 14549);
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
								['з', s @ ..] => {
									super::push(out, 14550);
									match s {
										[] => true,
										['а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 14551);
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
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', 'с', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 2581);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 7226);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22713);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 22714);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 22715);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 30610);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t6::n65863(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 622);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2582);
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
		['в', s @ ..] => {
			super::push(out, 623);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 624);
			match s {
				[] => true,
				['а', 'н', '\u{1}', s @ ..] => super::t6::n2765(s, out),
				['о', s @ ..] => super::t6::n2766(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t6::n635(s, out),
		['з', s @ ..] => super::t6::n636(s, out),
		['и', s @ ..] => super::t6::n637(s, out),
		['л', s @ ..] => super::t6::n638(s, out),
		['м', s @ ..] => super::t6::n639(s, out),
		['о', s @ ..] => super::t6::n640(s, out),
		['р', s @ ..] => super::t6::n641(s, out),
		['с', s @ ..] => super::t6::n642(s, out),
		['т', s @ ..] => super::t6::n643(s, out),
		['у', s @ ..] => super::t6::n644(s, out),
		['ы', s @ ..] => super::t6::n645(s, out),
		['ю', s @ ..] => super::t6::n646(s, out),
		['я', s @ ..] => super::t6::n647(s, out),
		_ => false,
	}
}

/// Words ending -вить.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		['а', s @ ..] => super::t6::n649(s, out),
		['б', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 2632);
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
		['в', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2633);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 2634);
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
		['е', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 2635);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 7351);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 14700);
									s.is_empty()
								}
								['з', 'а', 'р', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 14701);
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
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 2636);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7352);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14702);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ч', s @ ..] => {
									super::push(out, 14703);
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
					super::push(out, 2637);
					match s {
						[] => true,
						['х', s @ ..] => {
							super::push(out, 7353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 2638);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7354);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 22816);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 22817);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 30649);
													match s {
														[] => true,
														['е', 'п', '\u{1}', s @ ..] => super::t6::n65898(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 22818);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 30650);
													match s {
														[] => true,
														['п', '\u{1}', s @ ..] => super::t6::n65899(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 22819);
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
							super::push(out, 7355);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 14704);
									match s {
										[] => true,
										['о', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2639);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7356);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 14705);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 22820);
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
									super::push(out, 14706);
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
					super::push(out, 2640);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 2641);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7357);
							match s {
								[] => true,
								['т', s @ ..] => super::t6::n20091(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 2642);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t6::n8957(s, out),
						_ => false,
					}
				}
				['я', s @ ..] => super::t6::n2831(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t6::n655(s, out),
		['л', s @ ..] => super::t6::n656(s, out),
		['о', s @ ..] => super::t6::n657(s, out),
		['р', s @ ..] => super::t6::n658(s, out),
		['с', s @ ..] => super::t6::n659(s, out),
		['т', s @ ..] => super::t6::n660(s, out),
		['у', s @ ..] => super::t6::n661(s, out),
		['ы', s @ ..] => super::t6::n662(s, out),
		['я', s @ ..] => super::t6::n663(s, out),
		_ => false,
	}
}

/// Words ending -дить.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 654);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 2680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7430);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 7431);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 14769);
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
						['о', s @ ..] => {
							super::push(out, 7432);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 14770);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 14771);
									match s {
										[] => true,
										['п', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 7433);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14772);
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
				['г', s @ ..] => super::t6::n2870(s, out),
				['з', s @ ..] => {
					super::push(out, 2682);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7442);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 14781);
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
				['к', s @ ..] => super::t6::n2872(s, out),
				['л', s @ ..] => super::t6::n2873(s, out),
				['м', s @ ..] => super::t6::n2874(s, out),
				['р', s @ ..] => super::t6::n2875(s, out),
				['с', s @ ..] => super::t6::n2876(s, out),
				['ч', s @ ..] => super::t6::n2877(s, out),
				['ш', s @ ..] => super::t6::n2878(s, out),
				['щ', s @ ..] => super::t6::n2879(s, out),
				_ => false,
			}
		}
		['б', s @ ..] => super::t6::n665(s, out),
		['е', s @ ..] => super::t6::n666(s, out),
		['ж', s @ ..] => super::t6::n667(s, out),
		['з', s @ ..] => super::t6::n668(s, out),
		['и', s @ ..] => super::t6::n669(s, out),
		['й', s @ ..] => super::t6::n670(s, out),
		['л', s @ ..] => super::t6::n671(s, out),
		['н', s @ ..] => super::t6::n672(s, out),
		['о', s @ ..] => super::t6::n673(s, out),
		['р', s @ ..] => super::t6::n674(s, out),
		['у', s @ ..] => super::t6::n675(s, out),
		['ы', s @ ..] => super::t6::n676(s, out),
		['ю', s @ ..] => {
			super::push(out, 667);
			match s {
				[] => true,
				['л', s @ ..] => super::t6::n2937(s, out),
				_ => false,
			}
		}
		['я', s @ ..] => super::t6::n678(s, out),
		_ => false,
	}
}

/// The word заклеить.
pub(crate) fn n20560(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word наклеить.
pub(crate) fn n20561(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word обклеить.
pub(crate) fn n9288(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word вклеить.
pub(crate) fn n9289(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -дклеить.
pub(crate) fn n9290(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word переклеить.
pub(crate) fn n9291(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word приклеить.
pub(crate) fn n9292(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -оклеить.
pub(crate) fn n9293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7658);
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
		_ => false,
	}
}

/// Words ending -склеить.
pub(crate) fn n9294(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// The word отклеить.
pub(crate) fn n9295(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word уклеить.
pub(crate) fn n9296(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -ыклеить.
pub(crate) fn n9297(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7659);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 15028);
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

/// Words ending -плеить.
pub(crate) fn n2942(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2753);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 7660);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 15029);
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
		['у', s @ ..] => {
			super::push(out, 7661);
			match s {
				[] => true,
				['о', 'л', 'с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 23073);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 23074);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word оевропеить.
pub(crate) fn n682(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -феить.
pub(crate) fn n683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 672);
	match s {
		[] => true,
		['о', 'р', 'т', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 7662);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 7663);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -чеить.
pub(crate) fn n684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 673);
	match s {
		[] => true,
		['я', s @ ..] => {
			super::push(out, 2754);
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

/// Words ending -жить.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 674);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 2755);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7664);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 2756);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 7665);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				['з', s @ ..] => {
					super::push(out, 2757);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7666);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 2758);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 7667);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15030);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15031);
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
									super::push(out, 15032);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 15033);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 15034);
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
						['в', s @ ..] => {
							super::push(out, 7668);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 15035);
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
							super::push(out, 7669);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 15036);
									match s {
										[] => true,
										['а', 'т', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30777);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 30778);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t6::n66041(s, out),
														_ => false,
													}
												}
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
							super::push(out, 7670);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 15037);
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
						['ф', s @ ..] => {
							super::push(out, 7671);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15038);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2759);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 7672);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 15039);
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
					super::push(out, 2760);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7673);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 7674);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15040);
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
							super::push(out, 7675);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15041);
									match s {
										[] => true,
										['д', 'а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2761);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 7676);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15042);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 7677);
							match s {
								[] => true,
								['в', s @ ..] => super::t6::n20590(s, out),
								['т', s @ ..] => super::t6::n20591(s, out),
								_ => false,
							}
						}
						['о', s @ ..] => super::t6::n9318(s, out),
						['у', s @ ..] => super::t6::n9319(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t6::n687(s, out),
		['д', s @ ..] => super::t6::n688(s, out),
		['е', s @ ..] => super::t6::n689(s, out),
		['з', s @ ..] => super::t6::n690(s, out),
		['и', s @ ..] => super::t6::n691(s, out),
		['л', s @ ..] => super::t6::n692(s, out),
		['м', s @ ..] => super::t6::n693(s, out),
		['н', s @ ..] => super::t6::n694(s, out),
		['о', s @ ..] => super::t6::n695(s, out),
		['р', s @ ..] => super::t6::n696(s, out),
		['с', s @ ..] => super::t6::n697(s, out),
		['т', s @ ..] => super::t6::n698(s, out),
		['у', s @ ..] => super::t6::n699(s, out),
		['ы', s @ ..] => super::t6::n700(s, out),
		['ю', s @ ..] => super::t6::n701(s, out),
		['я', s @ ..] => super::t6::n702(s, out),
		_ => false,
	}
}

/// Words ending -зить.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2814);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7764);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 7765);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 15145);
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
						['о', s @ ..] => {
							super::push(out, 7766);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15146);
									match s {
										[] => true,
										['е', 'к', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2815);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7767);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 7768);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23153);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23154);
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 23155);
											match s {
												[] => true,
												['п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 23156);
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
							super::push(out, 7769);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 15147);
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
				['л', s @ ..] => super::t6::n3006(s, out),
				['п', s @ ..] => {
					super::push(out, 2817);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7781);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 7782);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 15157);
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
				['р', s @ ..] => super::t7::n3008(s, out),
				['у', s @ ..] => {
					super::push(out, 2819);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 7791);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15164);
									match s {
										[] => true,
										['у', 'б', 'о', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => super::t7::n704(s, out),
		['е', s @ ..] => super::t7::n705(s, out),
		['и', s @ ..] => super::t7::n706(s, out),
		['й', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['е', s @ ..] => super::t7::n3020(s, out),
				_ => false,
			}
		}
		['м', s @ ..] => super::t7::n708(s, out),
		['н', s @ ..] => super::t7::n709(s, out),
		['о', s @ ..] => super::t7::n710(s, out),
		['р', s @ ..] => super::t7::n711(s, out),
		['у', s @ ..] => super::t7::n712(s, out),
		['ь', s @ ..] => super::t7::n713(s, out),
		['ю', s @ ..] => super::t7::n714(s, out),
		['я', s @ ..] => super::t7::n715(s, out),
		_ => false,
	}
}

/// Words ending -лить.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 705);
			s.is_empty()
		}
		['а', s @ ..] => super::t7::n717(s, out),
		['б', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2874);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 7938);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 15326);
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
				['й', s @ ..] => {
					super::push(out, 2875);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 7939);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 15327);
									match s {
										[] => true,
										['и', 'т', 'ь', 'л', 'у', 'м', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 2876);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 7940);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 7941);
							match s {
								[] => true,
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23274);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23275);
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
										['и', s @ ..] => {
											super::push(out, 23276);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 23277);
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
										['ы', s @ ..] => {
											super::push(out, 23278);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 30864);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t7::n66127(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 708);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2877);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 2878);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 7942);
							match s {
								[] => true,
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
		['г', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 2879);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 7943);
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
				['и', s @ ..] => {
					super::push(out, 2880);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 7944);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 15328);
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
				['у', s @ ..] => super::t7::n3071(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2882);
					s.is_empty()
				}
				['е', s @ ..] => super::t7::n3073(s, out),
				['о', s @ ..] => super::t7::n3074(s, out),
				['ы', s @ ..] => {
					super::push(out, 2885);
					match s {
						[] => true,
						['б', s @ ..] => super::t7::n9636(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t7::n722(s, out),
		['з', s @ ..] => super::t7::n723(s, out),
		['и', s @ ..] => super::t7::n724(s, out),
		['й', s @ ..] => super::t7::n725(s, out),
		['к', s @ ..] => super::t7::n726(s, out),
		['л', s @ ..] => super::t7::n727(s, out),
		['м', s @ ..] => super::t7::n728(s, out),
		['о', s @ ..] => super::t7::n729(s, out),
		['п', s @ ..] => super::t7::n730(s, out),
		['р', s @ ..] => super::t7::n731(s, out),
		['с', s @ ..] => super::t7::n732(s, out),
		['т', s @ ..] => super::t7::n733(s, out),
		['у', s @ ..] => super::t7::n734(s, out),
		['ф', s @ ..] => super::t7::n735(s, out),
		['х', s @ ..] => super::t7::n736(s, out),
		['ш', s @ ..] => super::t7::n737(s, out),
		['ы', s @ ..] => super::t7::n738(s, out),
		['ю', s @ ..] => super::t7::n739(s, out),
		['я', s @ ..] => super::t7::n740(s, out),
		_ => false,
	}
}

/// Words ending -мить.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 2992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 2993);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8210);
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
				['к', s @ ..] => {
					super::push(out, 2994);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8211);
							match s {
								[] => true,
								['е', 'в', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 8212);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15582);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23415);
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
					super::push(out, 2995);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 8213);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 15583);
									match s {
										[] => true,
										['р', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 30905);
													s.is_empty()
												}
												['о', s @ ..] => {
													super::push(out, 30906);
													match s {
														[] => true,
														['р', s @ ..] => super::t7::n66172(s, out),
														_ => false,
													}
												}
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
							super::push(out, 8214);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15584);
									match s {
										[] => true,
										['о', 'п', 'с', 'а', 'р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 8215);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15585);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15586);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 23416);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 23417);
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
									super::push(out, 15587);
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
				['н', s @ ..] => {
					super::push(out, 2996);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 8216);
							match s {
								[] => true,
								['д', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23418);
											s.is_empty()
										}
										['а', s @ ..] => {
											super::push(out, 23419);
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
											super::push(out, 23420);
											match s {
												[] => true,
												['п', s @ ..] => {
													super::push(out, 30907);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t7::n66173(s, out),
														_ => false,
													}
												}
												['р', s @ ..] => {
													super::push(out, 30908);
													match s {
														[] => true,
														['п', s @ ..] => super::t7::n66174(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 2997);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 8217);
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
					super::push(out, 2998);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 8218);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15588);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23421);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => super::t7::n9937(s, out),
						_ => false,
					}
				}
				['с', s @ ..] => super::t7::n3189(s, out),
				['х', s @ ..] => super::t7::n3190(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t7::n742(s, out),
		['и', s @ ..] => super::t7::n743(s, out),
		['й', s @ ..] => super::t7::n744(s, out),
		['л', s @ ..] => super::t7::n745(s, out),
		['о', s @ ..] => super::t7::n746(s, out),
		['р', s @ ..] => super::t7::n747(s, out),
		['т', s @ ..] => super::t7::n748(s, out),
		['у', s @ ..] => super::t7::n749(s, out),
		['х', s @ ..] => super::t7::n750(s, out),
		['ы', s @ ..] => super::t7::n751(s, out),
		['ь', s @ ..] => super::t7::n752(s, out),
		['ю', s @ ..] => super::t7::n753(s, out),
		['я', s @ ..] => super::t7::n754(s, out),
		_ => false,
	}
}

/// Words ending -нить.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['а', s @ ..] => super::t7::n755(s, out),
		['б', s @ ..] => {
			super::push(out, 745);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3047);
					match s {
						[] => true,
						['б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15711);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15712);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 15713);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23520);
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
								['т', s @ ..] => {
									super::push(out, 15714);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 23521);
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
		['в', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3048);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8342);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23522);
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
				['е', s @ ..] => {
					super::push(out, 3049);
					match s {
						[] => true,
						['р', 'д', 'у', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 3050);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 8343);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15715);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 15716);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 23523);
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
						['р', s @ ..] => {
							super::push(out, 8344);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 15717);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 15718);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23524);
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
		['г', s @ ..] => {
			super::push(out, 747);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3051);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3052);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 8345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 8346);
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
					super::push(out, 3053);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8347);
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
					super::push(out, 3054);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8348);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 15719);
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
							super::push(out, 8349);
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
				['е', s @ ..] => {
					super::push(out, 3055);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8350);
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
					super::push(out, 3056);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 8351);
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
					super::push(out, 3057);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 8352);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 8353);
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
						['с', s @ ..] => {
							super::push(out, 8354);
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
				['с', s @ ..] => super::t7::n3248(s, out),
				['т', s @ ..] => {
					super::push(out, 3059);
					match s {
						[] => true,
						['о', s @ ..] => super::t7::n10096(s, out),
						_ => false,
					}
				}
				['у', s @ ..] => super::t7::n3250(s, out),
				['ы', s @ ..] => super::t7::n3251(s, out),
				['я', s @ ..] => super::t7::n3252(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t7::n759(s, out),
		['е', s @ ..] => super::t7::n760(s, out),
		['ж', s @ ..] => super::t7::n761(s, out),
		['з', s @ ..] => super::t7::n762(s, out),
		['и', s @ ..] => super::t7::n763(s, out),
		['й', s @ ..] => super::t7::n764(s, out),
		['л', s @ ..] => super::t7::n765(s, out),
		['м', s @ ..] => super::t7::n766(s, out),
		['н', s @ ..] => super::t7::n767(s, out),
		['о', s @ ..] => super::t7::n768(s, out),
		['п', s @ ..] => super::t7::n769(s, out),
		['р', s @ ..] => super::t7::n770(s, out),
		['с', s @ ..] => super::t7::n771(s, out),
		['т', s @ ..] => super::t7::n772(s, out),
		['у', s @ ..] => super::t7::n773(s, out),
		['ч', s @ ..] => super::t7::n774(s, out),
		['ш', s @ ..] => super::t7::n775(s, out),
		['ы', s @ ..] => super::t7::n776(s, out),
		['ю', s @ ..] => super::t7::n777(s, out),
		['я', s @ ..] => super::t7::n778(s, out),
		_ => false,
	}
}

/// Words ending -оить.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 3164);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8563);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15963);
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
		['в', s @ ..] => {
			super::push(out, 769);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 3165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8564);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 8565);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 15964);
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
							super::push(out, 8566);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 15965);
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
							super::push(out, 8567);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 15966);
									match s {
										[] => true,
										['р', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 15967);
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
							super::push(out, 8568);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 8569);
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
					super::push(out, 3166);
					match s {
						[] => true,
						['и', 'р', 'п', s @ ..] => {
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
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['о', 'д', 'е', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 3167);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 3168);
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
			super::push(out, 771);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3170);
					match s {
						[] => true,
						['н', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 3171);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8571);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15968);
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
					super::push(out, 3172);
					match s {
						[] => true,
						['р', 'е', 'п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 3173);
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
					super::push(out, 3174);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 8572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 15969);
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
						['п', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 3175);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3176);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 8573);
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
			super::push(out, 772);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3178);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15970);
									s.is_empty()
								}
								['с', s @ ..] => {
									match s {
										[] => true,
										['е', 'б', s @ ..] => super::t7::n37022(s, out),
										['у', s @ ..] => {
											super::push(out, 23715);
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
								['у', '\u{1}', s @ ..] => super::t7::n22031(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => super::t7::n784(s, out),
		['н', s @ ..] => super::t7::n785(s, out),
		['п', s @ ..] => super::t7::n786(s, out),
		['р', s @ ..] => super::t7::n787(s, out),
		['т', s @ ..] => super::t7::n788(s, out),
		_ => false,
	}
}

/// Words ending -пить.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 3198);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 8610);
							match s {
								[] => true,
								['с', 'е', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 780);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 3199);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8611);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 15993);
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
				['з', s @ ..] => {
					super::push(out, 3200);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 3201);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 8613);
							match s {
								[] => true,
								['ф', 'о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15994);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15995);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 8614);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['э', 'б', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15996);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 15997);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 3202);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8615);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 8616);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 15998);
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
							super::push(out, 8617);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 15999);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31024);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 31025);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t7::n66293(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 3203);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 3204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 8618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 3205);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16000);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 16001);
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
							super::push(out, 8619);
							match s {
								[] => true,
								['о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 3206);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 8620);
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
		['б', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3207);
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
		['в', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3209);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 8621);
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
					super::push(out, 3210);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 8622);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16002);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => super::t7::n795(s, out),
		['и', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3215);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 8643);
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
				['щ', s @ ..] => {
					super::push(out, 3216);
					match s {
						[] => true,
						['а', 'з', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 3217);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 8644);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 8645);
							match s {
								[] => true,
								['с', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => super::t7::n10460(s, out),
						['ч', s @ ..] => super::t7::n10461(s, out),
						_ => false,
					}
				}
				['е', s @ ..] => super::t7::n3410(s, out),
				_ => false,
			}
		}
		['л', s @ ..] => super::t7::n798(s, out),
		['м', s @ ..] => super::t7::n799(s, out),
		['о', s @ ..] => super::t8::n800(s, out),
		['п', s @ ..] => super::t8::n801(s, out),
		['р', s @ ..] => super::t8::n802(s, out),
		['с', s @ ..] => super::t8::n803(s, out),
		['т', s @ ..] => super::t8::n804(s, out),
		['у', s @ ..] => super::t8::n805(s, out),
		['ы', s @ ..] => super::t8::n806(s, out),
		['я', s @ ..] => super::t8::n807(s, out),
		_ => false,
	}
}

/// Words ending -рить.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['а', s @ ..] => super::t8::n808(s, out),
		['б', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3269);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 3270);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 8801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 8802);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 16154);
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
						['ф', s @ ..] => {
							super::push(out, 8803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16155);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['д', 'о', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['и', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 8804);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16156);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 16157);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23805);
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
									super::push(out, 16158);
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
				['д', s @ ..] => {
					super::push(out, 3271);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 8805);
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
				['е', s @ ..] => {
					super::push(out, 3272);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 8806);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 23806);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31047);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 23807);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31048);
													s.is_empty()
												}
												['а', 'з', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['д', 'о', 'п', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['о', s @ ..] => super::t8::n52778(s, out),
												['ы', s @ ..] => super::t8::n52779(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => super::t8::n22374(s, out),
								['с', s @ ..] => super::t8::n22375(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t8::n3465(s, out),
				['о', s @ ..] => super::t8::n3466(s, out),
				['с', s @ ..] => super::t8::n3467(s, out),
				['т', s @ ..] => super::t8::n3468(s, out),
				['у', s @ ..] => super::t8::n3469(s, out),
				['ы', s @ ..] => super::t8::n3470(s, out),
				['я', s @ ..] => super::t8::n3471(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t8::n810(s, out),
		['г', s @ ..] => super::t8::n811(s, out),
		['д', s @ ..] => super::t8::n812(s, out),
		['е', s @ ..] => super::t8::n813(s, out),
		['з', s @ ..] => super::t8::n814(s, out),
		['и', s @ ..] => super::t8::n815(s, out),
		['к', s @ ..] => super::t8::n816(s, out),
		['о', s @ ..] => super::t8::n817(s, out),
		['т', s @ ..] => super::t8::n818(s, out),
		['у', s @ ..] => super::t8::n819(s, out),
		['ф', s @ ..] => super::t8::n820(s, out),
		['х', s @ ..] => super::t8::n821(s, out),
		['щ', s @ ..] => super::t8::n822(s, out),
		['ы', s @ ..] => super::t8::n823(s, out),
		['ю', s @ ..] => super::t8::n824(s, out),
		['я', s @ ..] => super::t8::n825(s, out),
		_ => false,
	}
}

/// Words ending -сить.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 3375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9072);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9073);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16462);
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
							super::push(out, 9074);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16463);
									match s {
										[] => true,
										['к', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 31139);
													s.is_empty()
												}
												['а', s @ ..] => {
													super::push(out, 31140);
													match s {
														[] => true,
														['з', s @ ..] => super::t8::n66402(s, out),
														['н', s @ ..] => super::t8::n66403(s, out),
														_ => false,
													}
												}
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
							super::push(out, 9075);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 16464);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 23996);
											match s {
												[] => true,
												['у', 'т', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 9076);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16465);
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
						['у', s @ ..] => {
							super::push(out, 9077);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16466);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23997);
											s.is_empty()
										}
										['а', 'з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['з', 'и', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['о', 'п', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['в', s @ ..] => super::t8::n3569(s, out),
				['г', s @ ..] => super::t8::n3570(s, out),
				['к', s @ ..] => {
					super::push(out, 3378);
					match s {
						[] => true,
						['в', s @ ..] => super::t8::n10947(s, out),
						_ => false,
					}
				}
				['л', s @ ..] => super::t8::n3572(s, out),
				['м', s @ ..] => super::t8::n3573(s, out),
				['п', s @ ..] => super::t8::n3574(s, out),
				['р', s @ ..] => super::t8::n3575(s, out),
				['у', s @ ..] => super::t8::n3576(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t8::n827(s, out),
		['и', s @ ..] => super::t8::n828(s, out),
		['й', s @ ..] => super::t8::n829(s, out),
		['к', s @ ..] => super::t8::n830(s, out),
		['м', s @ ..] => super::t8::n831(s, out),
		['н', s @ ..] => super::t8::n832(s, out),
		['о', s @ ..] => super::t8::n833(s, out),
		['п', s @ ..] => super::t8::n834(s, out),
		['р', s @ ..] => super::t8::n835(s, out),
		['с', s @ ..] => super::t8::n836(s, out),
		['у', s @ ..] => super::t8::n837(s, out),
		['ы', s @ ..] => super::t8::n838(s, out),
		['э', s @ ..] => super::t8::n839(s, out),
		_ => false,
	}
}

/// Words ending -тить.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['а', s @ ..] => super::t8::n840(s, out),
		['г', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 3432);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 9228);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16651);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16652);
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 831);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3433);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 3434);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9229);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9230);
							match s {
								[] => true,
								['р', 'т', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9231);
							match s {
								[] => true,
								['н', 'о', 'к', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => super::t8::n3629(s, out),
				['и', s @ ..] => {
					super::push(out, 3436);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9236);
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
					super::push(out, 3437);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9237);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16670);
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
							super::push(out, 9238);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 16671);
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
						['п', s @ ..] => {
							super::push(out, 9239);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 16672);
									match s {
										[] => true,
										['д', 'с', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => super::t8::n3632(s, out),
				['н', s @ ..] => super::t8::n3633(s, out),
				['п', s @ ..] => super::t8::n3634(s, out),
				['р', s @ ..] => super::t8::n3635(s, out),
				['с', s @ ..] => super::t8::n3636(s, out),
				['у', s @ ..] => super::t8::n3637(s, out),
				['ф', s @ ..] => super::t8::n3638(s, out),
				['ш', s @ ..] => super::t8::n3639(s, out),
				['щ', s @ ..] => super::t8::n3640(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t8::n843(s, out),
		['й', s @ ..] => super::t8::n844(s, out),
		['к', s @ ..] => super::t8::n845(s, out),
		['л', s @ ..] => super::t8::n846(s, out),
		['н', s @ ..] => super::t8::n847(s, out),
		['о', s @ ..] => super::t8::n848(s, out),
		['п', s @ ..] => super::t8::n849(s, out),
		['р', s @ ..] => super::t8::n850(s, out),
		['с', s @ ..] => super::t8::n851(s, out),
		['у', s @ ..] => super::t8::n852(s, out),
		['ф', s @ ..] => super::t8::n853(s, out),
		['ч', s @ ..] => super::t8::n854(s, out),
		['ы', s @ ..] => super::t8::n855(s, out),
		['ю', s @ ..] => super::t8::n856(s, out),
		['я', s @ ..] => super::t8::n857(s, out),
		_ => false,
	}
}

/// Words ending -уить.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 847);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 3514);
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

/// Words ending -фить.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 3515);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 9408);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16913);
									s.is_empty()
								}
								['а', 'н', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 16914);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 24321);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 31296);
													match s {
														[] => true,
														['т', '\u{1}', s @ ..] => super::t8::n66566(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 24322);
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
									super::push(out, 16915);
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
								['о', 'р', 'п', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['т', 'о', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9409);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16916);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16917);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24323);
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
								['у', s @ ..] => {
									super::push(out, 16918);
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
			super::push(out, 849);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 3516);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16919);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16920);
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
				['р', s @ ..] => {
					super::push(out, 3517);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9410);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9411);
							match s {
								[] => true,
								['з', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 850);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3518);
					match s {
						[] => true,
						['р', 'д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16921);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 16922);
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
			super::push(out, 851);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3519);
					match s {
						[] => true,
						['с', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16923);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 16924);
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
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 3520);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9412);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9413);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16925);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 16926);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24324);
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

