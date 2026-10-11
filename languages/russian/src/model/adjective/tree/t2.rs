use flexion::Path;

/// Words ending -яный.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 408);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1331);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1332);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1334);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4028);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4029);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 9454);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4030);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4031);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9456);
							match s {
								[] => true,
								['м', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23409);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 23410);
											match s {
												[] => true,
												['н', 'и', 'м', 'а', '-', s @ ..] => {
													super::push(out, 28227);
													match s {
														[] => true,
														['а', s @ ..] => super::t5::n32555(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1335);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 4032);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9457);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 4033);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 9458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1336);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4034);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9459);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4035);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 9460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4036);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9461);
							match s {
								[] => true,
								['л', 'о', 'п', '\u{1}', s @ ..] => {
									super::push(out, 16606);
									s.is_empty()
								}
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
			super::push(out, 1337);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4037);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9462);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', 'с', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28228);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 28229);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', '\u{1}', s @ ..] => {
											super::push(out, 23411);
											s.is_empty()
										}
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
					super::push(out, 4038);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9463);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4039);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4040);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 4041);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1338);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4043);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4044);
					s.is_empty()
				}
				['ф', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9466);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 16607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 9467);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16608);
									s.is_empty()
								}
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
			super::push(out, 1340);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4045);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9468);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9469);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16609);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9470);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16610);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9471);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4046);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word зелено-серый.
pub(crate) fn n29641(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28230);
	s.is_empty()
}

/// Words ending -мокрый.
pub(crate) fn n4216(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9495);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 9496);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 16624);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -орый.
pub(crate) fn n421(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 419);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1361);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9497);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1362);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4069);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4070);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4072);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 9498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трый.
pub(crate) fn n422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 420);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9499);
							s.is_empty()
						}
						['е', s @ ..] => {
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 16625);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 16626);
									s.is_empty()
								}
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
			super::push(out, 1366);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4074);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9500);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9501);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 9502);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16627);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 9503);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16628);
									match s {
										[] => true,
										['п', '\u{1}', s @ ..] => {
											super::push(out, 23421);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9504);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9505);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4076);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 9506);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 4077);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16632);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 16633);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23422);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 16634);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23423);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -урый.
pub(crate) fn n423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1367);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4078);
					s.is_empty()
				}
				['-', s @ ..] => {
					super::push(out, 4079);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16635);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 23424);
											match s {
												[] => true,
												['е', 'т', '\u{1}', s @ ..] => {
													super::push(out, 28231);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 23425);
											match s {
												[] => true,
												['е', 'ч', '\u{1}', s @ ..] => {
													super::push(out, 28232);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 16636);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 23426);
											match s {
												[] => true,
												['е', 'ж', '\u{1}', s @ ..] => {
													super::push(out, 28233);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1369);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1370);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4081);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1371);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4082);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дырый.
pub(crate) fn n1405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1372);
	s.is_empty()
}

/// Words ending -ярый.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 1373);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -иневолосый.
pub(crate) fn n29645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28234);
	s.is_empty()
}

/// Words ending -авоволосый.
pub(crate) fn n29646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28235);
	s.is_empty()
}

/// Words ending -адковолосый.
pub(crate) fn n29647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28236);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31022);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тковолосый.
pub(crate) fn n29648(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28237);
	s.is_empty()
}

/// Words ending -ловолосый.
pub(crate) fn n24626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23431);
	s.is_empty()
}

/// Words ending -новолосый.
pub(crate) fn n24627(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['з', s @ ..] => {
			super::push(out, 28238);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 31023);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 28239);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 28240);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 28241);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 28242);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 31024);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -товолосый.
pub(crate) fn n24628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23432);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 28243);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 28244);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 28245);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -голосый.
pub(crate) fn n9974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9515);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 16638);
			match s {
				[] => true,
				['ч', s @ ..] => {
					super::push(out, 23433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 23434);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 28246);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 23435);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -колосый.
pub(crate) fn n9975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9516);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 16639);
			match s {
				[] => true,
				['к', 'л', s @ ..] => {
					super::push(out, 23436);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 28247);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -носый.
pub(crate) fn n1412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1376);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 4089);
			match s {
				[] => true,
				['ш', s @ ..] => {
					super::push(out, 9517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4090);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 9518);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 9519);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 9520);
					match s {
						[] => true,
						['т', 'о', s @ ..] => {
							super::push(out, 16640);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 9521);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16641);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16642);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 23438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 9522);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9523);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 16643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 9524);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 4091);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -усый.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1377);
			match s {
				[] => true,
				['ж', s @ ..] => {
					super::push(out, 4093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1378);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1379);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4095);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4096);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4097);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['-', s @ ..] => {
					super::push(out, 4099);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9525);
							match s {
								[] => true,
								['н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
									super::push(out, 16644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысый.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1381);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -омясый.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1383);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 1384);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4100);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -атый.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1385);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4101);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 4102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1386);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 9526);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 23439);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28248);
													s.is_empty()
												}
												_ => false,
											}
										}
										['в', s @ ..] => {
											super::push(out, 23440);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ы', s @ ..] => {
									super::push(out, 16645);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 9527);
							s.is_empty()
						}
						['л', 'и', s @ ..] => {
							super::push(out, 9528);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 9529);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 9530);
							match s {
								[] => true,
								['н', 'я', s @ ..] => {
									super::push(out, 16647);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 23441);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', 'ь', s @ ..] => {
							super::push(out, 9531);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 9532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4103);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9533);
							s.is_empty()
						}
						['в', s @ ..] => super::t5::n9994(s, out),
						['г', s @ ..] => {
							super::push(out, 9535);
							s.is_empty()
						}
						['д', s @ ..] => super::t5::n9996(s, out),
						['з', s @ ..] => {
							super::push(out, 9537);
							s.is_empty()
						}
						['к', s @ ..] => super::t5::n9998(s, out),
						['л', s @ ..] => super::t5::n9999(s, out),
						['н', s @ ..] => super::t5::n10000(s, out),
						['п', 'и', 'р', s @ ..] => super::t5::n10001(s, out),
						['р', s @ ..] => super::t5::n10002(s, out),
						['с', s @ ..] => super::t5::n10003(s, out),
						['т', s @ ..] => super::t5::n10004(s, out),
						['х', s @ ..] => super::t5::n10005(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => super::t5::n1423(s, out),
		['д', s @ ..] => super::t5::n1424(s, out),
		['ж', s @ ..] => super::t5::n1425(s, out),
		['з', s @ ..] => super::t5::n1426(s, out),
		['к', s @ ..] => super::t5::n1427(s, out),
		['л', s @ ..] => super::t5::n1428(s, out),
		['м', s @ ..] => super::t5::n1429(s, out),
		['н', s @ ..] => super::t5::n1430(s, out),
		['п', s @ ..] => super::t5::n1431(s, out),
		['р', s @ ..] => super::t5::n1432(s, out),
		['с', s @ ..] => super::t5::n1433(s, out),
		['т', s @ ..] => super::t5::n1434(s, out),
		['х', s @ ..] => super::t5::n1435(s, out),
		['ц', s @ ..] => super::t5::n1436(s, out),
		['ч', s @ ..] => super::t5::n1437(s, out),
		['щ', s @ ..] => super::t5::n1438(s, out),
		_ => false,
	}
}

/// Words ending -битый.
pub(crate) fn n1443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1407);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4167);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4168);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9606);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 9607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4169);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4170);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4171);
			s.is_empty()
		}
		['з', s @ ..] => {
			super::push(out, 4172);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9608);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 9609);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9610);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 4173);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4174);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4175);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4176);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 4177);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 4178);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9611);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -витый.
pub(crate) fn n1444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1408);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4179);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 4180);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9612);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4181);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 4182);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9613);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9614);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 9615);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4183);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9616);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23509);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 23510);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 23511);
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 28280);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 28281);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 4184);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9617);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 16725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4185);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 9618);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 16726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16727);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 23512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 16728);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 16729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9619);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 9620);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4186);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 4187);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -житый.
pub(crate) fn n1446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1410);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 4189);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 9622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4190);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4191);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 4192);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 9625);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4193);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 9626);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9627);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 16733);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4194);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -литый.
pub(crate) fn n1447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1411);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4195);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4196);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9629);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16734);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 16735);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 23513);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 9630);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 4197);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9631);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 4198);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 4199);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9632);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 16737);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4200);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9633);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 16738);
							match s {
								[] => true,
								['п', '\u{1}', s @ ..] => {
									super::push(out, 23514);
									s.is_empty()
								}
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
			super::push(out, 4201);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 9634);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16739);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 9635);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 9636);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 16740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4202);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 9637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16741);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 16742);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 23515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 9638);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 16743);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 23516);
									s.is_empty()
								}
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
			super::push(out, 4203);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 4204);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9639);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4205);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9640);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -питый.
pub(crate) fn n1450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1414);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4207);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4208);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 9642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4209);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 4210);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 9644);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 4211);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9645);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 9646);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 9647);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 9648);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 4212);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9649);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9650);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 9651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 4213);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 9652);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16744);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4214);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9653);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ритый.
pub(crate) fn n1451(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1415);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 4215);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9654);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 9655);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 9656);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 16745);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 9657);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 9658);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 16746);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шитый.
pub(crate) fn n1454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1418);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4218);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 4219);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 4220);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 4221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 4222);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 4223);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 4224);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 4225);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9660);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 9661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 4226);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 9662);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -лтый.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1419);
			match s {
				[] => true,
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9663);
							s.is_empty()
						}
						['-', s @ ..] => {
							super::push(out, 9664);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23517);
											s.is_empty()
										}
										['н', s @ ..] => {
											super::push(out, 23518);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 28282);
													match s {
														[] => true,
														['л', s @ ..] => super::t5::n32566(s, out),
														_ => false,
													}
												}
												['м', s @ ..] => {
													super::push(out, 28283);
													match s {
														[] => true,
														['е', s @ ..] => super::t5::n32567(s, out),
														_ => false,
													}
												}
												['н', s @ ..] => {
													super::push(out, 28284);
													s.is_empty()
												}
												['ч', s @ ..] => {
													super::push(out, 28285);
													match s {
														[] => true,
														['а', s @ ..] => super::t5::n32568(s, out),
														['и', s @ ..] => super::t5::n32569(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 23519);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -отый.
pub(crate) fn n437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1420);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4227);
					match s {
						[] => true,
						['к', s @ ..] => {
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 16747);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16748);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23520);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 9665);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 9666);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 16749);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23521);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1421);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4228);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 9667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9668);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 9669);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 9670);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 9671);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 16751);
									s.is_empty()
								}
								['ч', s @ ..] => {
									super::push(out, 16752);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 9672);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 16753);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 16754);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 23522);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9673);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9674);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16756);
									s.is_empty()
								}
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
			super::push(out, 1422);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4229);
					match s {
						[] => true,
						['е', 'р', s @ ..] => {
							match s {
								[] => true,
								['т', '\u{1}', s @ ..] => {
									super::push(out, 16757);
									s.is_empty()
								}
								['ы', 'т', 'е', 'ч', '\u{1}', s @ ..] => {
									super::push(out, 16758);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ртый.
pub(crate) fn n438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4230);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 9675);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16759);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4231);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9676);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16760);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 9677);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9678);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9679);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 9680);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 9681);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16762);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9682);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9683);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16763);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4232);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9684);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9685);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9686);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 9687);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 16764);
									s.is_empty()
								}
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
			super::push(out, 1424);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 4233);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 9688);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -стый.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 436);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1425);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4234);
					match s {
						[] => true,
						['о', 'з', s @ ..] => {
							super::push(out, 9689);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9691);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 9692);
							s.is_empty()
						}
						['ю', s @ ..] => {
							super::push(out, 9693);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16766);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4235);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4236);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9694);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16767);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4237);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4238);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9695);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 9696);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16768);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 9697);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16769);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', 'л', s @ ..] => {
							super::push(out, 9698);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 16770);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9699);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 16771);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 9700);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 9701);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4239);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 9702);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 16772);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 9703);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 16773);
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
						['е', s @ ..] => {
							super::push(out, 9704);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 9705);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 16774);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4240);
					match s {
						[] => true,
						['о', s @ ..] => {
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 16775);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 16776);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 9706);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9707);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16777);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4241);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4242);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4243);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 4244);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 9708);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16778);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ю', s @ ..] => {
							super::push(out, 9709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', 'ш', s @ ..] => {
					super::push(out, 4245);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 9710);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 4246);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9711);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9712);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 4247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n1462(s, out),
		['л', s @ ..] => {
			super::push(out, 1427);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4268);
					match s {
						[] => true,
						['т', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16917);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 16918);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23613);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1428);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4269);
					match s {
						[] => true,
						['х', s @ ..] => {
							match s {
								[] => true,
								['е', 'р', s @ ..] => {
									super::push(out, 16919);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 23614);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['к', 'о', s @ ..] => {
											super::push(out, 23615);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 28329);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', 'е', s @ ..] => {
											super::push(out, 23616);
											match s {
												[] => true,
												['б', s @ ..] => {
													super::push(out, 28330);
													s.is_empty()
												}
												_ => false,
											}
										}
										['н', s @ ..] => {
											super::push(out, 23617);
											s.is_empty()
										}
										['р', s @ ..] => {
											super::push(out, 23618);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 16920);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23619);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', 'е', 'р', 'т', '\u{1}', s @ ..] => {
									super::push(out, 16921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => super::t5::n4432(s, out),
				['р', s @ ..] => super::t5::n4433(s, out),
				_ => false,
			}
		}
		['р', s @ ..] => super::t5::n1465(s, out),
		['у', s @ ..] => super::t5::n1466(s, out),
		['я', s @ ..] => super::t5::n1467(s, out),
		_ => false,
	}
}

/// Words ending -утый.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4275);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1433);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4276);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 9818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4277);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4278);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4279);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9820);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 9821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4280);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4281);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4282);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4283);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4284);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9823);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1436);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4285);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 9824);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['в', s @ ..] => {
							super::push(out, 9825);
							match s {
								[] => true,
								['е', '\u{1}', s @ ..] => {
									super::push(out, 16925);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 9826);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16926);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23621);
											s.is_empty()
										}
										_ => false,
									}
								}
								['й', s @ ..] => {
									super::push(out, 16927);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 23622);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 9827);
							match s {
								[] => true,
								['и', 'ш', '\u{1}', s @ ..] => {
									super::push(out, 16928);
									s.is_empty()
								}
								['й', s @ ..] => {
									super::push(out, 16929);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 16930);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23623);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 9828);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 16931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4286);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9829);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 16932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => super::t5::n4449(s, out),
				['и', s @ ..] => super::t5::n4450(s, out),
				['к', s @ ..] => super::t5::n4451(s, out),
				['о', s @ ..] => super::t5::n4452(s, out),
				['п', s @ ..] => {
					super::push(out, 4291);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9844);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => super::t5::n4454(s, out),
				['с', s @ ..] => super::t5::n4455(s, out),
				['т', s @ ..] => super::t5::n4456(s, out),
				['у', s @ ..] => super::t5::n4457(s, out),
				['х', s @ ..] => super::t5::n4458(s, out),
				['ю', s @ ..] => super::t5::n4459(s, out),
				['я', s @ ..] => super::t5::n4460(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ытый.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4299);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4300);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 9861);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16962);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4301);
					match s {
						[] => true,
						['д', 'з', s @ ..] => {
							super::push(out, 9862);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 16963);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4302);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4303);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4304);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4305);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4306);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4307);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4308);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4309);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4310);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4311);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4312);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4313);
					match s {
						[] => true,
						['о', 'е', s @ ..] => {
							super::push(out, 9866);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4314);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4315);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1440);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4316);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4317);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4319);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4320);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4321);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9869);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4322);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9871);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9872);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9873);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 9874);
							match s {
								[] => true,
								['р', 'п', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 23653);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 28350);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 23654);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9875);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 9876);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16965);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 16966);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 23655);
											s.is_empty()
										}
										_ => false,
									}
								}
								['в', s @ ..] => {
									super::push(out, 16967);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 16968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9877);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 9878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4323);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4324);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 4325);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 9880);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16969);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 23656);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1441);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4326);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ютый.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4327);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ятый.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1443);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 1444);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1445);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4329);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9882);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9883);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 9884);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16970);
									s.is_empty()
								}
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
			super::push(out, 1446);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4330);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1447);
			match s {
				[] => true,
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9885);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9886);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 16971);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 23657);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 9887);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16972);
									match s {
										[] => true,
										['т', 'с', s @ ..] => {
											super::push(out, 23658);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 28351);
													s.is_empty()
												}
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
							super::push(out, 9888);
							match s {
								[] => true,
								['р', 'п', '\u{1}', s @ ..] => {
									super::push(out, 16973);
									s.is_empty()
								}
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
			super::push(out, 1448);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4331);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 4332);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4333);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4334);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4335);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9889);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 16974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4336);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 9890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1449);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4338);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 9891);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16975);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 23659);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28352);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 16976);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 23660);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 9892);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 4339);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9893);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 4340);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4341);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4342);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 9895);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 9896);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 16977);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4343);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9897);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 16978);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4344);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 9899);
							match s {
								[] => true,
								['п', s @ ..] => {
									match s {
										[] => true,
										['д', s @ ..] => {
											super::push(out, 23661);
											s.is_empty()
										}
										['е', s @ ..] => {
											super::push(out, 23662);
											match s {
												[] => true,
												['щ', s @ ..] => {
													super::push(out, 28353);
													s.is_empty()
												}
												_ => false,
											}
										}
										['с', s @ ..] => {
											super::push(out, 23663);
											s.is_empty()
										}
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
					super::push(out, 4345);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4346);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9900);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 9901);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 9902);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 16979);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 16980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 9903);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 16981);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 9904);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4347);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 9905);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4348);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4349);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 9906);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4350);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4351);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 9907);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ъ', s @ ..] => {
			super::push(out, 1452);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -леднолицый.
pub(crate) fn n29767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28354);
	s.is_empty()
}

/// Words ending -меднолицый.
pub(crate) fn n29768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28355);
	s.is_empty()
}

/// Words ending -ийся.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 482);
	match s {
		[] => true,
		['ш', s @ ..] => super::t5::n1509(s, out),
		['щ', s @ ..] => super::t5::n1510(s, out),
		_ => false,
	}
}

/// The word превеликий.
pub(crate) fn n30329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28884);
	s.is_empty()
}

/// Words ending -епревеликий.
pub(crate) fn n30330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28885);
	s.is_empty()
}

/// The word износостойкий.
pub(crate) fn n30331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28886);
	s.is_empty()
}

/// The word неглубокий.
pub(crate) fn n30332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28887);
	s.is_empty()
}

/// Words ending -преглубокий.
pub(crate) fn n30333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28888);
	s.is_empty()
}

/// Words ending -вский.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1593);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 4474);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4475);
					s.is_empty()
				}
				['л', s @ ..] => {
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10036);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 10037);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4476);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4477);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 4478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1594);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4479);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4480);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4481);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4482);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10038);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 10039);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10040);
							s.is_empty()
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 4483);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 10041);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 17225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 10042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4484);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 10043);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 10044);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17226);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 17227);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 10045);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 10046);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 10047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4485);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10048);
							match s {
								[] => true,
								['к', s @ ..] => {
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 24138);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24139);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 28889);
													s.is_empty()
												}
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
							super::push(out, 10049);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 10050);
							match s {
								[] => true,
								['о', s @ ..] => {
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 24140);
											s.is_empty()
										}
										['л', s @ ..] => {
											super::push(out, 24141);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 28890);
													s.is_empty()
												}
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
							super::push(out, 10051);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 17228);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24142);
											s.is_empty()
										}
										_ => false,
									}
								}
								['г', s @ ..] => {
									super::push(out, 17229);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 17230);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24143);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['и', s @ ..] => super::t5::n10536(s, out),
						['й', s @ ..] => {
							super::push(out, 10052);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17236);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => super::t5::n10538(s, out),
						['м', s @ ..] => super::t5::n10539(s, out),
						['о', s @ ..] => super::t5::n10540(s, out),
						['у', s @ ..] => super::t5::n10541(s, out),
						_ => false,
					}
				}
				['н', s @ ..] => super::t5::n4662(s, out),
				['о', s @ ..] => super::t5::n4663(s, out),
				['р', s @ ..] => super::t5::n4664(s, out),
				['у', s @ ..] => super::t5::n4665(s, out),
				['ц', s @ ..] => super::t5::n4666(s, out),
				['ч', s @ ..] => super::t5::n4667(s, out),
				['ш', s @ ..] => super::t5::n4668(s, out),
				['щ', s @ ..] => super::t5::n4669(s, out),
				['ь', s @ ..] => super::t5::n4670(s, out),
				['э', s @ ..] => super::t5::n4671(s, out),
				_ => false,
			}
		}
		['и', s @ ..] => super::t5::n1642(s, out),
		['о', s @ ..] => super::t5::n1643(s, out),
		['р', s @ ..] => super::t5::n1644(s, out),
		_ => false,
	}
}

/// Words ending -йенчепингский.
pub(crate) fn n30361(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28916);
	s.is_empty()
}

/// Words ending -сембургский.
pub(crate) fn n30362(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28917);
	s.is_empty()
}

/// Words ending -дский.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1606);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1607);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4526);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 10191);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17436);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4527);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4528);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 10192);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17437);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4529);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', 'п', s @ ..] => {
			super::push(out, 1608);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4530);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1609);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4531);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4532);
					match s {
						[] => true,
						['л', 'д', s @ ..] => {
							super::push(out, 10194);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 17438);
									s.is_empty()
								}
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
			super::push(out, 1610);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4533);
					match s {
						[] => true,
						['о', 'г', 'ж', s @ ..] => {
							super::push(out, 10195);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 17439);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24283);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1611);
			match s {
				[] => true,
				['а', 'в', s @ ..] => {
					super::push(out, 4534);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10196);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 4535);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10197);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 10198);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4536);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -еский.
pub(crate) fn n596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4537);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 1613);
			s.is_empty()
		}
		['м', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 4538);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 1615);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4539);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4540);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4541);
					s.is_empty()
				}
				['и', s @ ..] => super::t5::n4720(s, out),
				['н', s @ ..] => {
					super::push(out, 4543);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 4544);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 10215);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 17479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17480);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 17481);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24327);
											s.is_empty()
										}
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
					super::push(out, 4545);
					s.is_empty()
				}
				['ь', s @ ..] => {
					super::push(out, 4546);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 1616);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4547);
					s.is_empty()
				}
				['о', s @ ..] => super::t5::n4726(s, out),
				_ => false,
			}
		}
		['щ', s @ ..] => super::t5::n1664(s, out),
		_ => false,
	}
}

/// Words ending -жский.
pub(crate) fn n597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1618);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1619);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4550);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10219);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 17483);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 17484);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 24328);
											match s {
												[] => true,
												['н', 'а', '\u{1}', s @ ..] => {
													super::push(out, 28967);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4551);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1620);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4552);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4553);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 4554);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10221);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10222);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17485);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 17486);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24329);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 17487);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24330);
											s.is_empty()
										}
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
					super::push(out, 4555);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10223);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1621);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 1622);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1623);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4556);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10224);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 4557);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 4558);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4559);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10226);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 10227);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17489);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', 'у', 'б', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4560);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10228);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17490);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4561);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1624);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 1625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -йский.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 594);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1628);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1629);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4563);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 10229);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 17491);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 24331);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 28968);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4564);
					match s {
						[] => true,
						['о', 'р', 'в', 'е', s @ ..] => {
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17492);
									s.is_empty()
								}
								['ъ', s @ ..] => {
									super::push(out, 17493);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 24332);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 28969);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4565);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4566);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10230);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10231);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17494);
									s.is_empty()
								}
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
			super::push(out, 1630);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4567);
					match s {
						[] => true,
						['с', 'о', 'р', '\u{1}', s @ ..] => {
							super::push(out, 10232);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -кский.
pub(crate) fn n600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 595);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4568);
					match s {
						[] => true,
						['о', '\u{1}', s @ ..] => {
							super::push(out, 10233);
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
					super::push(out, 4569);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4570);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1632);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4571);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4572);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10235);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 10236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 4573);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 4574);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10237);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17495);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ч', s @ ..] => {
					super::push(out, 4575);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 10238);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1633);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', 'и', s @ ..] => {
			super::push(out, 1634);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4577);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 10239);
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
				['р', s @ ..] => {
					super::push(out, 4578);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4579);
					match s {
						[] => true,
						['с', 'о', 'р', s @ ..] => {
							super::push(out, 10241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1635);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 1636);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мский.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 597);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1637);
			s.is_empty()
		}
		['е', 'ф', s @ ..] => {
			super::push(out, 1638);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4580);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 10242);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'ц', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4581);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10243);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4582);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 10244);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 10245);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 17496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', 'х', s @ ..] => {
					super::push(out, 4583);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 10246);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нский.
pub(crate) fn n603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 598);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4584);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 4585);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10247);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 17497);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 17498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 10248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4586);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10249);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17499);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24333);
											s.is_empty()
										}
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
					super::push(out, 4587);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4588);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4589);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4590);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 4591);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10250);
							match s {
								[] => true,
								['е', 'л', 'б', s @ ..] => {
									super::push(out, 17500);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10251);
							s.is_empty()
						}
						['ц', s @ ..] => {
							super::push(out, 10252);
							match s {
								[] => true,
								['е', 'н', 'е', 'в', '-', s @ ..] => {
									super::push(out, 17501);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											super::push(out, 24334);
											match s {
												[] => true,
												['с', s @ ..] => {
													super::push(out, 28970);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 4592);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 4593);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4594);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10253);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 10254);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17502);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 10255);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 10256);
							s.is_empty()
						}
						['п', s @ ..] => {
							super::push(out, 10257);
							match s {
								[] => true,
								['э', s @ ..] => {
									super::push(out, 17503);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17504);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 17505);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 17506);
									match s {
										[] => true,
										['г', s @ ..] => {
											super::push(out, 24335);
											s.is_empty()
										}
										['м', s @ ..] => {
											super::push(out, 24336);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 24337);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 28971);
													s.is_empty()
												}
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
								['о', s @ ..] => {
									super::push(out, 17507);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 17508);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24338);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 10258);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17509);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 10259);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17510);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 24339);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 10260);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17511);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ч', s @ ..] => {
							super::push(out, 10261);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 17512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 10262);
							s.is_empty()
						}
						['ь', s @ ..] => {
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17513);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 17514);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 24340);
											s.is_empty()
										}
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
					super::push(out, 4595);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10263);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4596);
					s.is_empty()
				}
				['р', 'з', s @ ..] => {
					super::push(out, 4597);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 10264);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17516);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4598);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10265);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10266);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17517);
									s.is_empty()
								}
								['и', s @ ..] => super::t5::n18436(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 4599);
					match s {
						[] => true,
						['а', s @ ..] => super::t5::n10765(s, out),
						_ => false,
					}
				}
				['ш', s @ ..] => super::t5::n4778(s, out),
				_ => false,
			}
		}
		['е', s @ ..] => super::t5::n1693(s, out),
		['и', s @ ..] => super::t5::n1694(s, out),
		['й', s @ ..] => super::t5::n1695(s, out),
		['н', s @ ..] => super::t5::n1696(s, out),
		['о', s @ ..] => super::t5::n1697(s, out),
		['р', s @ ..] => super::t5::n1698(s, out),
		['у', s @ ..] => super::t5::n1699(s, out),
		['ы', s @ ..] => super::t5::n1700(s, out),
		['э', s @ ..] => super::t5::n1701(s, out),
		['я', s @ ..] => super::t5::n1702(s, out),
		_ => false,
	}
}

/// Words ending -оский.
pub(crate) fn n604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 599);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1653);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1654);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 1655);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4673);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 10438);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 1656);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 4674);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 1657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4675);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 1658);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 10440);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 10441);
							match s {
								[] => true,
								['н', '\u{1}', s @ ..] => {
									super::push(out, 17633);
									s.is_empty()
								}
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
			super::push(out, 1659);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пский.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 600);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1660);
			s.is_empty()
		}
		['о', 'к', s @ ..] => {
			super::push(out, 1661);
			match s {
				[] => true,
				['с', s @ ..] => {
					super::push(out, 4678);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10442);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рский.
pub(crate) fn n606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 601);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1662);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4679);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 10443);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17634);
									match s {
										[] => true,
										['е', 'в', '\u{1}', s @ ..] => {
											super::push(out, 24373);
											s.is_empty()
										}
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
						['а', s @ ..] => {
							super::push(out, 10444);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 10445);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 10446);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4680);
					s.is_empty()
				}
				['д', s @ ..] => {
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10447);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 10448);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 17637);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 10449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 4681);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 4682);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4683);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10450);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 17638);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 10451);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 10452);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 10453);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 10454);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 10455);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 4684);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10456);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 17639);
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
						['й', s @ ..] => {
							super::push(out, 10457);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17640);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24374);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 10458);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4685);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 4686);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10459);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17642);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 17643);
									s.is_empty()
								}
								['ц', '\u{1}', s @ ..] => {
									super::push(out, 17644);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 10460);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17645);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 10461);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 10462);
							match s {
								[] => true,
								['г', '\u{1}', s @ ..] => {
									super::push(out, 17646);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4687);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10463);
							s.is_empty()
						}
						['й', s @ ..] => {
							super::push(out, 10464);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17647);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24375);
											s.is_empty()
										}
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
					super::push(out, 4688);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 17648);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24376);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 17649);
									s.is_empty()
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 10465);
							s.is_empty()
						}
						['у', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17650);
									s.is_empty()
								}
								['б', s @ ..] => {
									super::push(out, 17651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 4689);
					match s {
						[] => true,
						['й', s @ ..] => {
							super::push(out, 10466);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17652);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 10467);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17653);
									s.is_empty()
								}
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
			super::push(out, 1663);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 1664);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4691);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1665);
			s.is_empty()
		}
		['е', s @ ..] => super::t5::n1716(s, out),
		['и', s @ ..] => {
			super::push(out, 1667);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 4709);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4710);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 4711);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 4712);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 4713);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10555);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 17719);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => super::t5::n11076(s, out),
						['ш', s @ ..] => {
							super::push(out, 10557);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => super::t5::n4913(s, out),
				_ => false,
			}
		}
		['к', s @ ..] => {
			super::push(out, 1668);
			s.is_empty()
		}
		['о', s @ ..] => super::t5::n1719(s, out),
		['п', s @ ..] => super::t5::n1720(s, out),
		['р', s @ ..] => {
			super::push(out, 1671);
			s.is_empty()
		}
		['т', 'с', 'и', s @ ..] => super::t5::n1722(s, out),
		['у', s @ ..] => super::t5::n1723(s, out),
		['ф', s @ ..] => super::t5::n1724(s, out),
		['ы', s @ ..] => super::t5::n1725(s, out),
		['э', s @ ..] => super::t5::n1726(s, out),
		['ю', s @ ..] => super::t5::n1727(s, out),
		['я', s @ ..] => super::t5::n1728(s, out),
		_ => false,
	}
}

/// Words ending -сский.
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1677);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4739);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10588);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17743);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 4740);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10589);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 10590);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17744);
									s.is_empty()
								}
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
			super::push(out, 1678);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4741);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10591);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 17745);
									s.is_empty()
								}
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
			super::push(out, 1679);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 1680);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4742);
					s.is_empty()
				}
				['б', s @ ..] => {
					super::push(out, 4743);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 10592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 4744);
					match s {
						[] => true,
						['а', 'л', s @ ..] => {
							super::push(out, 10593);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'у', s @ ..] => {
					super::push(out, 4745);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 10594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 1681);
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 4746);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10595);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17746);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4747);
					match s {
						[] => true,
						['о', 'р', s @ ..] => {
							super::push(out, 10596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4748);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10597);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17747);
									s.is_empty()
								}
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
			super::push(out, 1682);
			match s {
				[] => true,
				['л', 'е', s @ ..] => {
					super::push(out, 4749);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 10598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 1683);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4750);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 10599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тский.
pub(crate) fn n608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4751);
					match s {
						[] => true,
						['р', 'о', 'х', 'о', 'б', s @ ..] => {
							super::push(out, 10600);
							match s {
								[] => true,
								['р', 'е', 'с', '\u{1}', s @ ..] => {
									super::push(out, 17748);
									s.is_empty()
								}
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
			super::push(out, 1685);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 4752);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 4753);
					match s {
						[] => true,
						['-', 'о', 'н', 'ж', 'е', 'д', 'о', 'л', 'о', 'м', '\u{1}', s @ ..] => {
							super::push(out, 10601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['п', s @ ..] => {
					super::push(out, 4754);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 4755);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 10603);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 17749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 4756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1686);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1687);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4757);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 4758);
					match s {
						[] => true,
						['г', 'е', s @ ..] => {
							super::push(out, 10604);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 10605);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 10606);
							s.is_empty()
						}
						['к', s @ ..] => {
							super::push(out, 10607);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 10608);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 17751);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 17752);
									match s {
										[] => true,
										['н', s @ ..] => {
											super::push(out, 24405);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 10609);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4759);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 1688);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1689);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 10611);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 17753);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10612);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 10613);
							s.is_empty()
						}
						['ф', s @ ..] => {
							super::push(out, 10614);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 17754);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['э', s @ ..] => {
					super::push(out, 4760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1690);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 4761);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 10615);
							match s {
								[] => true,
								['и', 'л', s @ ..] => {
									super::push(out, 17755);
									match s {
										[] => true,
										['а', 'т', 'с', '\u{1}', s @ ..] => {
											super::push(out, 24406);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1691);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4762);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 10616);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17756);
									match s {
										[] => true,
										['л', 'р', 'е', 'г', '\u{1}', s @ ..] => {
											super::push(out, 24407);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 10617);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4763);
					s.is_empty()
				}
				['м', 'р', s @ ..] => {
					super::push(out, 4764);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 10618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 4765);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 4766);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 1692);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4767);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 10619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 1693);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10620);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 10621);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уский.
pub(crate) fn n609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1694);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 1695);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 4768);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4769);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10622);
							s.is_empty()
						}
						_ => false,
					}
				}
				['б', s @ ..] => {
					super::push(out, 4770);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 4771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1696);
			match s {
				[] => true,
				['ь', s @ ..] => {
					super::push(out, 4772);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 1697);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -фский.
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1698);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 1699);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 4773);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 10624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -хский.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 606);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1700);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 1701);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1702);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 4775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нцский.
pub(crate) fn n1756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1703);
	s.is_empty()
}

/// Words ending -чский.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1704);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4776);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 1705);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 1706);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4777);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -шский.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 1707);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 1708);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 4778);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьский.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 1709);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4779);
					match s {
						[] => true,
						['г', 'н', 'и', s @ ..] => {
							super::push(out, 10626);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17758);
									s.is_empty()
								}
								['ф', s @ ..] => {
									super::push(out, 17759);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 24408);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 10627);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 17760);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 24409);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10628);
							s.is_empty()
						}
						['с', 'п', s @ ..] => {
							super::push(out, 10629);
							match s {
								[] => true,
								['у', s @ ..] => {
									super::push(out, 17761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 10630);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 17762);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24410);
											s.is_empty()
										}
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
						['а', s @ ..] => {
							super::push(out, 10631);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 10632);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 4780);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 10633);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 10634);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 17763);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 24411);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['г', s @ ..] => {
							super::push(out, 10635);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 10636);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 17764);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 10637);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 17765);
									s.is_empty()
								}
								_ => false,
							}
						}
						['й', s @ ..] => {
							super::push(out, 10638);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 10639);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17766);
									match s {
										[] => true,
										['з', s @ ..] => {
											super::push(out, 24412);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => super::t5::n11163(s, out),
						['с', s @ ..] => super::t5::n11164(s, out),
						['т', s @ ..] => super::t5::n11165(s, out),
						['ш', s @ ..] => super::t5::n11166(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t5::n4985(s, out),
				['л', s @ ..] => super::t5::n4986(s, out),
				['о', s @ ..] => super::t5::n4987(s, out),
				['у', s @ ..] => super::t5::n4988(s, out),
				['ы', s @ ..] => super::t5::n4989(s, out),
				['э', s @ ..] => super::t5::n4990(s, out),
				['ю', s @ ..] => super::t5::n4991(s, out),
				['я', s @ ..] => super::t5::n4992(s, out),
				_ => false,
			}
		}
		['н', s @ ..] => super::t5::n1763(s, out),
		['р', s @ ..] => super::t5::n1764(s, out),
		_ => false,
	}
}

/// Words ending -яский.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['р', s @ ..] => {
			super::push(out, 1712);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 4791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 1713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нький.
pub(crate) fn n634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 629);
	match s {
		[] => true,
		['е', s @ ..] => super::t5::n1797(s, out),
		['о', s @ ..] => {
			super::push(out, 1744);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 4839);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10749);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 17889);
									s.is_empty()
								}
								_ => false,
							}
						}
						['я', s @ ..] => {
							super::push(out, 10750);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 17890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 4840);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 10751);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 17891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 17892);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 24497);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 17893);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ь', s @ ..] => {
							super::push(out, 10752);
							s.is_empty()
						}
						['я', s @ ..] => {
							super::push(out, 10753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 4841);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 10754);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 17894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 10755);
							match s {
								[] => true,
								['в', 'о', 'р', s @ ..] => {
									super::push(out, 17895);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 17896);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 17897);
									match s {
										[] => true,
										['м', s @ ..] => {
											super::push(out, 24498);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 24499);
											match s {
												[] => true,
												['т', s @ ..] => {
													super::push(out, 29022);
													s.is_empty()
												}
												_ => false,
											}
										}
										['р', s @ ..] => {
											super::push(out, 24500);
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 29023);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', 'к', s @ ..] => {
							super::push(out, 10756);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10757);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 17898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 10758);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17899);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -трехсторонний.
pub(crate) fn n30474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29024);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 31487);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word четырехсторонний.
pub(crate) fn n30475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29025);
	s.is_empty()
}

/// Words ending -авший.
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 757);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1890);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 4953);
					match s {
						[] => true,
						['л', 'х', 'ы', s @ ..] => {
							super::push(out, 10821);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 10822);
							s.is_empty()
						}
						['ы', s @ ..] => {
							super::push(out, 10823);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 17925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 4954);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 4955);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 10824);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 17926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 4956);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 4957);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17927);
									s.is_empty()
								}
								['а', s @ ..] => {
									super::push(out, 17928);
									s.is_empty()
								}
								['н', s @ ..] => {
									super::push(out, 17929);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 4958);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 10825);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 10826);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 10827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t5::n1956(s, out),
		['г', s @ ..] => {
			super::push(out, 1892);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 4967);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 10890);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 10891);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18149);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18150);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24746);
											s.is_empty()
										}
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
						['б', s @ ..] => {
							super::push(out, 10892);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18151);
									s.is_empty()
								}
								['а', s @ ..] => {
									match s {
										[] => true,
										['з', '\u{1}', s @ ..] => {
											super::push(out, 24747);
											s.is_empty()
										}
										['н', '\u{1}', s @ ..] => {
											super::push(out, 24748);
											s.is_empty()
										}
										_ => false,
									}
								}
								['б', 'о', '\u{1}', s @ ..] => {
									super::push(out, 18152);
									s.is_empty()
								}
								['в', s @ ..] => {
									super::push(out, 18153);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18154);
									s.is_empty()
								}
								['з', 'в', s @ ..] => {
									super::push(out, 18155);
									s.is_empty()
								}
								['и', s @ ..] => {
									super::push(out, 18156);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18157);
									match s {
										[] => true,
										['д', '\u{1}', s @ ..] => {
											super::push(out, 24749);
											s.is_empty()
										}
										['п', s @ ..] => {
											super::push(out, 24750);
											s.is_empty()
										}
										['р', 'п', '\u{1}', s @ ..] => {
											super::push(out, 24751);
											s.is_empty()
										}
										_ => false,
									}
								}
								['у', s @ ..] => {
									super::push(out, 18158);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 18159);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24752);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 10893);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 10894);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 10895);
							match s {
								[] => true,
								['с', 'ы', s @ ..] => {
									super::push(out, 18160);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24753);
											s.is_empty()
										}
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
					super::push(out, 4968);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 10896);
							match s {
								[] => true,
								['р', 'б', 'ы', s @ ..] => {
									super::push(out, 18161);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24754);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => super::t5::n5187(s, out),
				['л', s @ ..] => super::t5::n5188(s, out),
				['о', s @ ..] => super::t5::n5189(s, out),
				['р', s @ ..] => super::t5::n5190(s, out),
				['у', s @ ..] => super::t5::n5191(s, out),
				['ы', s @ ..] => super::t5::n5192(s, out),
				['я', s @ ..] => super::t5::n5193(s, out),
				_ => false,
			}
		}
		['д', s @ ..] => super::t5::n1958(s, out),
		['ж', s @ ..] => super::t5::n1959(s, out),
		['з', s @ ..] => super::t5::n1960(s, out),
		['к', s @ ..] => super::t5::n1961(s, out),
		['л', s @ ..] => super::t5::n1962(s, out),
		['м', s @ ..] => super::t5::n1963(s, out),
		['н', s @ ..] => super::t5::n1964(s, out),
		['п', s @ ..] => super::t5::n1965(s, out),
		['р', s @ ..] => super::t5::n1966(s, out),
		['с', s @ ..] => super::t5::n1967(s, out),
		['т', s @ ..] => super::t5::n1968(s, out),
		['х', s @ ..] => super::t5::n1969(s, out),
		['ц', s @ ..] => super::t5::n1970(s, out),
		['ч', s @ ..] => super::t5::n1971(s, out),
		['ш', s @ ..] => super::t5::n1972(s, out),
		['щ', s @ ..] => super::t5::n1973(s, out),
		_ => false,
	}
}

/// Words ending -евший.
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 758);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['у', 'з', s @ ..] => {
					super::push(out, 5116);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11194);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18469);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5117);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11195);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18470);
									s.is_empty()
								}
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
			super::push(out, 1910);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5118);
					match s {
						[] => true,
						['ж', s @ ..] => {
							super::push(out, 11196);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24932);
											s.is_empty()
										}
										['и', s @ ..] => {
											super::push(out, 24933);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29263);
													s.is_empty()
												}
												_ => false,
											}
										}
										['о', s @ ..] => {
											super::push(out, 24934);
											s.is_empty()
										}
										['у', s @ ..] => {
											super::push(out, 24935);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 11197);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 18471);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ш', s @ ..] => {
							super::push(out, 11198);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 11199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 5119);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11200);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 18472);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 11201);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 11202);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 11203);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 11204);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['з', s @ ..] => {
					super::push(out, 5120);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11205);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5121);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11206);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 11207);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11208);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 18475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11209);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18476);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18477);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 11210);
							s.is_empty()
						}
						['ш', s @ ..] => {
							super::push(out, 11211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5122);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 11212);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 11213);
							s.is_empty()
						}
						['з', s @ ..] => {
							super::push(out, 11214);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 11215);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18478);
									s.is_empty()
								}
								['я', s @ ..] => {
									super::push(out, 18479);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 24936);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 11216);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11217);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 18480);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24937);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 18481);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 24938);
											s.is_empty()
										}
										_ => false,
									}
								}
								['о', s @ ..] => {
									super::push(out, 18482);
									match s {
										[] => true,
										['д', 'з', s @ ..] => {
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29264);
													s.is_empty()
												}
												['ы', s @ ..] => {
													super::push(out, 29265);
													match s {
														[] => true,
														['в', s @ ..] => super::t5::n33175(s, out),
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
									super::push(out, 18483);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 24939);
											s.is_empty()
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 18484);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 24940);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 11218);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 18485);
									s.is_empty()
								}
								['п', s @ ..] => {
									super::push(out, 18486);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5123);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 11219);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 5124);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 11220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 1911);
			s.is_empty()
		}
		['д', s @ ..] => super::t5::n1977(s, out),
		['ж', s @ ..] => super::t5::n1978(s, out),
		['з', s @ ..] => {
			super::push(out, 1914);
			s.is_empty()
		}
		['л', s @ ..] => super::t5::n1980(s, out),
		['м', s @ ..] => super::t5::n1981(s, out),
		['н', s @ ..] => super::t5::n1982(s, out),
		['о', s @ ..] => {
			super::push(out, 1918);
			s.is_empty()
		}
		['п', s @ ..] => super::t5::n1984(s, out),
		['р', s @ ..] => super::t5::n1985(s, out),
		['с', s @ ..] => super::t5::n1986(s, out),
		['т', s @ ..] => super::t5::n1987(s, out),
		['у', s @ ..] => {
			super::push(out, 1923);
			s.is_empty()
		}
		['ч', s @ ..] => super::t5::n1989(s, out),
		['ш', s @ ..] => super::t5::n1990(s, out),
		['ъ', s @ ..] => super::t5::n1991(s, out),
		['ы', s @ ..] => super::t5::n1992(s, out),
		_ => false,
	}
}

/// Words ending -ивший.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 759);
	match s {
		[] => true,
		['а', s @ ..] => {
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5187);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 11282);
							match s {
								[] => true,
								['ы', s @ ..] => {
									super::push(out, 18530);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24961);
											s.is_empty()
										}
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
					super::push(out, 5188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 1928);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5189);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 11283);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 11284);
							s.is_empty()
						}
						['н', s @ ..] => {
							super::push(out, 11285);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5190);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5191);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5192);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11287);
							match s {
								[] => true,
								['е', 'т', 'ы', s @ ..] => {
									super::push(out, 18531);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 24962);
											s.is_empty()
										}
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
					super::push(out, 5193);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5194);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 11288);
							s.is_empty()
						}
						['ш', 'ы', s @ ..] => {
							super::push(out, 11289);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18532);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5195);
					match s {
						[] => true,
						['о', 'д', 'ы', s @ ..] => {
							super::push(out, 11290);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 18533);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5196);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5197);
					match s {
						[] => true,
						['д', s @ ..] => {
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 18534);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18535);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 24963);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 18536);
									s.is_empty()
								}
								_ => false,
							}
						}
						['к', s @ ..] => {
							super::push(out, 11291);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 18537);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 11292);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 18538);
									match s {
										[] => true,
										['ж', s @ ..] => {
											super::push(out, 24964);
											s.is_empty()
										}
										_ => false,
									}
								}
								['з', s @ ..] => {
									super::push(out, 18539);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 11293);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 18540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['п', s @ ..] => {
							super::push(out, 11294);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 11295);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 18541);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 18542);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 18543);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18544);
									match s {
										[] => true,
										['к', s @ ..] => {
											super::push(out, 24965);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 18545);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 18546);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 18547);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 18548);
									match s {
										[] => true,
										['б', s @ ..] => {
											super::push(out, 24966);
											s.is_empty()
										}
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29268);
													s.is_empty()
												}
												['с', s @ ..] => {
													super::push(out, 29269);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5198);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 11296);
							match s {
								[] => true,
								['щ', s @ ..] => {
									super::push(out, 18549);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 24967);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29270);
													s.is_empty()
												}
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
							super::push(out, 11297);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 18550);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 18551);
									match s {
										[] => true,
										['с', s @ ..] => super::t5::n26230(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => super::t5::n5422(s, out),
				['т', s @ ..] => super::t5::n5423(s, out),
				['у', s @ ..] => super::t5::n5424(s, out),
				['ы', s @ ..] => super::t5::n5425(s, out),
				['ю', s @ ..] => super::t5::n5426(s, out),
				['я', s @ ..] => super::t5::n5427(s, out),
				_ => false,
			}
		}
		['в', s @ ..] => super::t5::n1995(s, out),
		['д', s @ ..] => super::t5::n1996(s, out),
		['е', s @ ..] => super::t5::n1997(s, out),
		['ж', s @ ..] => super::t5::n1998(s, out),
		['з', s @ ..] => super::t6::n1999(s, out),
		['л', s @ ..] => super::t6::n2000(s, out),
		['м', s @ ..] => super::t6::n2001(s, out),
		['н', s @ ..] => super::t6::n2002(s, out),
		['о', s @ ..] => super::t6::n2003(s, out),
		['п', s @ ..] => super::t6::n2004(s, out),
		['р', s @ ..] => super::t6::n2005(s, out),
		['с', s @ ..] => super::t6::n2006(s, out),
		['т', s @ ..] => super::t6::n2007(s, out),
		['ф', s @ ..] => super::t6::n2008(s, out),
		['ч', s @ ..] => super::t6::n2009(s, out),
		['ш', s @ ..] => super::t6::n2010(s, out),
		['щ', s @ ..] => super::t6::n2011(s, out),
		_ => false,
	}
}

/// Words ending -увший.
pub(crate) fn n771(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 761);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 1947);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5407);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 1948);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 5408);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 11856);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 1949);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5409);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => super::t6::n2017(s, out),
		_ => false,
	}
}

/// Words ending -смеявший.
pub(crate) fn n12566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11966);
	match s {
		[] => true,
		['о', s @ ..] => {
			super::push(out, 19447);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 19448);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 25616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ысеявший.
pub(crate) fn n12567(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11967);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 19449);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -иявший.
pub(crate) fn n2027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1958);
	s.is_empty()
}

/// Words ending -лявший.
pub(crate) fn n2028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1959);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5450);
			match s {
				[] => true,
				['в', 'ы', s @ ..] => {
					super::push(out, 11968);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 5451);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 5452);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5453);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 11969);
					match s {
						[] => true,
						['т', 'с', 'ы', s @ ..] => {
							super::push(out, 19451);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25617);
									s.is_empty()
								}
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
			super::push(out, 5454);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 5455);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 5456);
			match s {
				[] => true,
				['г', 'ы', s @ ..] => {
					super::push(out, 11970);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 5457);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11971);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 19453);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25618);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25619);
									s.is_empty()
								}
								['ы', s @ ..] => {
									super::push(out, 25620);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29529);
											s.is_empty()
										}
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
					super::push(out, 11972);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 19454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -мявший.
pub(crate) fn n2029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1960);
	match s {
		[] => true,
		['ы', s @ ..] => {
			super::push(out, 5458);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11973);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -нявший.
pub(crate) fn n2030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1961);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5459);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11974);
					s.is_empty()
				}
				['о', 'р', 'ы', s @ ..] => {
					super::push(out, 11975);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5460);
			match s {
				[] => true,
				['м', 'ы', s @ ..] => {
					super::push(out, 11976);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19456);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', 'л', s @ ..] => {
			super::push(out, 5461);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 11977);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 5462);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -оявший.
pub(crate) fn n2031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1962);
	match s {
		[] => true,
		['т', 'с', s @ ..] => {
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11978);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 11979);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рявший.
pub(crate) fn n2032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1963);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5463);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 5464);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 5465);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 11980);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 11981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 19459);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 19460);
							s.is_empty()
						}
						['з', 'а', 'р', '\u{1}', s @ ..] => {
							super::push(out, 19461);
							s.is_empty()
						}
						['и', 'р', 'п', '\u{1}', s @ ..] => {
							super::push(out, 19462);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 19463);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 25621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['у', s @ ..] => {
							super::push(out, 19464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 11982);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 11983);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 19465);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 19466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 5466);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5467);
			s.is_empty()
		}
		['т', s @ ..] => {
			super::push(out, 5468);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 5469);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 19467);
							match s {
								[] => true,
								['к', 'ы', s @ ..] => {
									super::push(out, 25623);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29530);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ш', 'ы', s @ ..] => {
							super::push(out, 19468);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['н', 'ш', s @ ..] => {
					super::push(out, 11984);
					match s {
						[] => true,
						['ы', s @ ..] => {
							super::push(out, 19469);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 25625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уявший.
pub(crate) fn n2033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1964);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5470);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11985);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 5471);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ъявший.
pub(crate) fn n2034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1965);
	s.is_empty()
}

/// Words ending -антливейший.
pub(crate) fn n30987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29531);
	s.is_empty()
}

/// Words ending -лейший.
pub(crate) fn n2048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1978);
	s.is_empty()
}

/// Words ending -мейший.
pub(crate) fn n2049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1979);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5484);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нейший.
pub(crate) fn n2050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1980);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5485);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 11994);
					s.is_empty()
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 5486);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 11995);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 11996);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 11997);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 5487);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 11998);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 11999);
					s.is_empty()
				}
				['и', 'в', 'е', s @ ..] => {
					super::push(out, 12000);
					match s {
						[] => true,
						['ч', s @ ..] => {
							super::push(out, 19477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12001);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 12002);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 19479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12003);
					s.is_empty()
				}
				['я', s @ ..] => {
					super::push(out, 12004);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 5488);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 12005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 5489);
			match s {
				[] => true,
				['а', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19481);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19482);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 25630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', s @ ..] => {
					super::push(out, 12006);
					s.is_empty()
				}
				['о', s @ ..] => {
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 19483);
							s.is_empty()
						}
						['т', s @ ..] => {
							super::push(out, 19484);
							match s {
								[] => true,
								['ч', s @ ..] => {
									super::push(out, 25631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 12007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 5490);
			s.is_empty()
		}
		['й', s @ ..] => {
			super::push(out, 5491);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5492);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 12008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12009);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 19485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 5493);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12011);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19486);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25632);
									s.is_empty()
								}
								_ => false,
							}
						}
						['д', s @ ..] => {
							super::push(out, 19487);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 19488);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 19489);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19490);
							match s {
								[] => true,
								['т', 'с', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 29532);
											s.is_empty()
										}
										['о', s @ ..] => {
											super::push(out, 29533);
											s.is_empty()
										}
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
					super::push(out, 12012);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19491);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 25634);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 25635);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 29534);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 19492);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 19493);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 25636);
									match s {
										[] => true,
										['ы', s @ ..] => {
											super::push(out, 29535);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['м', s @ ..] => {
							super::push(out, 19494);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 19495);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 25637);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 25638);
									match s {
										[] => true,
										['с', s @ ..] => {
											super::push(out, 29536);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ц', s @ ..] => {
							super::push(out, 19496);
							s.is_empty()
						}
						['щ', s @ ..] => {
							super::push(out, 19497);
							match s {
								[] => true,
								['я', s @ ..] => {
									super::push(out, 25639);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 29537);
											s.is_empty()
										}
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
					super::push(out, 12013);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19498);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 19499);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 25640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 5494);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12015);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 12016);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 19500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5495);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12017);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 19501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 12018);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5496);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 12019);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 12020);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 12021);
					match s {
						[] => true,
						['к', 'в', s @ ..] => {
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 25641);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 29538);
											s.is_empty()
										}
										_ => false,
									}
								}
								['и', s @ ..] => {
									super::push(out, 25642);
									s.is_empty()
								}
								_ => false,
							}
						}
						['н', s @ ..] => {
							super::push(out, 19502);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => super::t6::n5729(s, out),
		['ф', s @ ..] => {
			super::push(out, 5498);
			s.is_empty()
		}
		['ч', s @ ..] => super::t6::n5731(s, out),
		['ш', s @ ..] => super::t6::n5732(s, out),
		['щ', s @ ..] => super::t6::n5733(s, out),
		['ь', s @ ..] => super::t6::n5734(s, out),
		_ => false,
	}
}

/// Words ending -пейший.
pub(crate) fn n2051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1981);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 5503);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 12033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 5504);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -рейший.
pub(crate) fn n2052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1982);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 5505);
			match s {
				[] => true,
				['с', 'о', '\u{1}', s @ ..] => {
					super::push(out, 12034);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тейший.
pub(crate) fn n2053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1983);
	match s {
		[] => true,
		['и', s @ ..] => {
			super::push(out, 5506);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12035);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 5507);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тонущий.
pub(crate) fn n5847(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 12100);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 12101);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 12102);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 19535);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 12103);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ьнущий.
pub(crate) fn n2152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2078);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 5609);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -янущий.
pub(crate) fn n2153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2079);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5610);
			s.is_empty()
		}
		['л', s @ ..] => {
			super::push(out, 5611);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 12104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', '\u{1}', s @ ..] => {
			super::push(out, 5612);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -пущий.
pub(crate) fn n822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 809);
	s.is_empty()
}

/// Words ending -рущий.
pub(crate) fn n823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 810);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -сущий.
pub(crate) fn n824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 811);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2081);
			s.is_empty()
		}
		['а', s @ ..] => {
			super::push(out, 2082);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2083);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2084);
			match s {
				[] => true,
				['д', s @ ..] => {
					super::push(out, 5613);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 5614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2085);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5615);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2086);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2087);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2088);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -тущий.
pub(crate) fn n825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 812);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2089);
			match s {
				[] => true,
				['в', 'ц', '\u{1}', s @ ..] => {
					super::push(out, 5616);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5617);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5618);
					match s {
						[] => true,
						['г', '\u{1}', s @ ..] => {
							super::push(out, 12106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2090);
			match s {
				[] => true,
				['а', 'р', '\u{1}', s @ ..] => {
					super::push(out, 5619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 2091);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -цущий.
pub(crate) fn n826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 813);
	match s {
		[] => true,
		['с', s @ ..] => {
			super::push(out, 2092);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -чущий.
pub(crate) fn n827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 814);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2093);
			match s {
				[] => true,
				['л', 'п', '\u{1}', s @ ..] => {
					super::push(out, 5620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -шущий.
pub(crate) fn n828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 815);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2095);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2096);
			match s {
				[] => true,
				['п', 'о', 'н', s @ ..] => {
					super::push(out, 5621);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ь', s @ ..] => {
			super::push(out, 2097);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5622);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12108);
							match s {
								[] => true,
								['б', '\u{1}', s @ ..] => {
									super::push(out, 19536);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -щущий.
pub(crate) fn n829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 816);
	match s {
		[] => true,
		['е', s @ ..] => {
			super::push(out, 2098);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2099);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ающий.
pub(crate) fn n830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 817);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 2100);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5624);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5625);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5626);
					match s {
						[] => true,
						['д', 'о', 'п', '\u{1}', s @ ..] => {
							super::push(out, 12109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5627);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12110);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19537);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5628);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5629);
					match s {
						[] => true,
						['д', '\u{1}', s @ ..] => {
							super::push(out, 12111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 5630);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12112);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 12113);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 19538);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12114);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 19539);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => super::t6::n2175(s, out),
		['г', s @ ..] => {
			super::push(out, 2102);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5637);
					match s {
						[] => true,
						['л', 'о', s @ ..] => {
							super::push(out, 12146);
							match s {
								[] => true,
								['п', 'о', s @ ..] => {
									super::push(out, 19646);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 25758);
											match s {
												[] => true,
												['о', s @ ..] => {
													super::push(out, 29578);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 5638);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12147);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19647);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19648);
									s.is_empty()
								}
								_ => false,
							}
						}
						['л', s @ ..] => {
							super::push(out, 12148);
							match s {
								[] => true,
								['а', 'з', s @ ..] => {
									super::push(out, 19649);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 25759);
											match s {
												[] => true,
												['к', s @ ..] => {
													super::push(out, 29579);
													s.is_empty()
												}
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
							super::push(out, 12149);
							match s {
								[] => true,
								['е', s @ ..] => {
									match s {
										[] => true,
										['б', 'с', s @ ..] => {
											super::push(out, 25760);
											match s {
												[] => true,
												['о', s @ ..] => {
													match s {
														[] => true,
														['д', s @ ..] => super::t6::n33243(s, out),
														['л', 'а', s @ ..] => super::t6::n33244(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['т', s @ ..] => {
											super::push(out, 25761);
											s.is_empty()
										}
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
					super::push(out, 5639);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5640);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12151);
							match s {
								[] => true,
								['д', '\u{1}', s @ ..] => {
									super::push(out, 19650);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ж', s @ ..] => {
							super::push(out, 12152);
							match s {
								[] => true,
								['б', s @ ..] => {
									super::push(out, 19651);
									match s {
										[] => true,
										['о', '\u{1}', s @ ..] => {
											super::push(out, 25762);
											s.is_empty()
										}
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
					super::push(out, 5641);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12153);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12154);
							match s {
								[] => true,
								['т', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 25763);
											s.is_empty()
										}
										['с', s @ ..] => {
											super::push(out, 25764);
											s.is_empty()
										}
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
					super::push(out, 5642);
					match s {
						[] => true,
						['е', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19652);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 19653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 5643);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12155);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19654);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ы', s @ ..] => {
					super::push(out, 5644);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 12157);
							s.is_empty()
						}
						['р', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19655);
									s.is_empty()
								}
								['д', s @ ..] => {
									super::push(out, 19656);
									s.is_empty()
								}
								['з', s @ ..] => {
									super::push(out, 19657);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 25765);
											s.is_empty()
										}
										_ => false,
									}
								}
								['п', s @ ..] => {
									super::push(out, 19658);
									s.is_empty()
								}
								['у', s @ ..] => {
									super::push(out, 19659);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['я', s @ ..] => {
					super::push(out, 5645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => super::t6::n2177(s, out),
		['ж', s @ ..] => super::t6::n2178(s, out),
		['з', s @ ..] => super::t6::n2179(s, out),
		['к', s @ ..] => super::t6::n2180(s, out),
		['л', s @ ..] => super::t6::n2181(s, out),
		['м', s @ ..] => super::t6::n2182(s, out),
		['н', s @ ..] => super::t6::n2183(s, out),
		['п', s @ ..] => super::t6::n2184(s, out),
		['р', s @ ..] => super::t6::n2185(s, out),
		['с', s @ ..] => super::t6::n2186(s, out),
		['т', s @ ..] => super::t6::n2187(s, out),
		['х', s @ ..] => super::t6::n2188(s, out),
		['ц', s @ ..] => super::t6::n2189(s, out),
		['ч', s @ ..] => super::t6::n2190(s, out),
		['ш', s @ ..] => super::t6::n2191(s, out),
		['щ', s @ ..] => super::t6::n2192(s, out),
		_ => false,
	}
}

/// Words ending -ующий.
pub(crate) fn n839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 826);
	match s {
		[] => true,
		['б', s @ ..] => {
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5767);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5768);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5769);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12350);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['в', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 5770);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12351);
							s.is_empty()
						}
						['с', s @ ..] => super::t6::n12963(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['г', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 5771);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 5772);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5773);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5774);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2156);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5775);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5776);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12354);
							s.is_empty()
						}
						['л', s @ ..] => super::t6::n12966(s, out),
						['р', s @ ..] => super::t6::n12967(s, out),
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 5777);
					match s {
						[] => true,
						['а', s @ ..] => super::t6::n12968(s, out),
						_ => false,
					}
				}
				['и', s @ ..] => super::t6::n6020(s, out),
				['л', s @ ..] => super::t6::n6021(s, out),
				['н', s @ ..] => super::t6::n6022(s, out),
				['о', s @ ..] => super::t6::n6023(s, out),
				['у', s @ ..] => super::t6::n6024(s, out),
				['я', s @ ..] => super::t6::n6025(s, out),
				_ => false,
			}
		}
		['ж', s @ ..] => super::t6::n2233(s, out),
		['з', s @ ..] => super::t6::n2234(s, out),
		['к', s @ ..] => super::t6::n2235(s, out),
		['л', s @ ..] => super::t6::n2236(s, out),
		['м', s @ ..] => super::t6::n2237(s, out),
		['н', s @ ..] => super::t6::n2238(s, out),
		['п', s @ ..] => super::t6::n2239(s, out),
		['р', s @ ..] => super::t6::n2240(s, out),
		['с', s @ ..] => super::t6::n2241(s, out),
		['т', s @ ..] => super::t6::n2242(s, out),
		['ф', s @ ..] => super::t6::n2243(s, out),
		['х', s @ ..] => super::t6::n2244(s, out),
		['ц', s @ ..] => super::t6::n2245(s, out),
		['ч', s @ ..] => super::t6::n2246(s, out),
		['ш', s @ ..] => super::t6::n2247(s, out),
		_ => false,
	}
}

/// Words ending -люющий.
pub(crate) fn n2252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2176);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 5827);
			s.is_empty()
		}
		['б', s @ ..] => {
			super::push(out, 5828);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 5829);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 5830);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 5831);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 5832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -яющий.
pub(crate) fn n842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 829);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 2179);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5834);
					s.is_empty()
				}
				['б', 'а', s @ ..] => {
					super::push(out, 5835);
					match s {
						[] => true,
						['л', 'с', 'с', s @ ..] => {
							super::push(out, 12413);
							match s {
								[] => true,
								['а', 'р', '\u{1}', s @ ..] => {
									super::push(out, 19842);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 5836);
					match s {
						[] => true,
						['а', s @ ..] => {
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 19843);
									match s {
										[] => true,
										['о', 'п', '\u{1}', s @ ..] => {
											super::push(out, 25871);
											s.is_empty()
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 25872);
											s.is_empty()
										}
										['т', s @ ..] => {
											super::push(out, 25873);
											match s {
												[] => true,
												['о', '\u{1}', s @ ..] => {
													super::push(out, 29654);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									super::push(out, 19844);
									s.is_empty()
								}
								_ => false,
							}
						}
						['з', s @ ..] => {
							super::push(out, 12414);
							match s {
								[] => true,
								['е', s @ ..] => {
									super::push(out, 19845);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 25874);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['е', 'д', 'е', s @ ..] => {
					super::push(out, 5837);
					match s {
						[] => true,
						['р', 'п', 'о', '\u{1}', s @ ..] => {
							super::push(out, 12415);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', 'о', s @ ..] => {
					super::push(out, 5838);
					match s {
						[] => true,
						['л', 'е', 'ш', s @ ..] => {
							super::push(out, 12416);
							match s {
								[] => true,
								['о', '\u{1}', s @ ..] => {
									super::push(out, 19846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5839);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12417);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 12418);
							s.is_empty()
						}
						['м', s @ ..] => {
							super::push(out, 12419);
							match s {
								[] => true,
								['у', '\u{1}', s @ ..] => {
									super::push(out, 19847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12420);
							match s {
								[] => true,
								['у', 'е', s @ ..] => {
									super::push(out, 19848);
									match s {
										[] => true,
										['л', s @ ..] => {
											super::push(out, 25875);
											s.is_empty()
										}
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
					super::push(out, 5840);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5841);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5842);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12421);
							match s {
								[] => true,
								['ч', 'е', 'п', 'в', '\u{1}', s @ ..] => {
									super::push(out, 19849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 12422);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 5843);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12423);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 19850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', s @ ..] => {
							super::push(out, 12424);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2180);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 5844);
					match s {
						[] => true,
						['ь', 'п', 'о', '\u{1}', s @ ..] => {
							super::push(out, 12425);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2181);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5845);
					match s {
						[] => true,
						['м', '\u{1}', s @ ..] => {
							super::push(out, 12426);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -дящий.
pub(crate) fn n846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 833);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2199);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5864);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 5865);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5866);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5867);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5868);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5869);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 5870);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2200);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2201);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5871);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5872);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12447);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 12448);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 5873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2202);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5874);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5875);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5876);
					match s {
						[] => true,
						['п', '\u{1}', s @ ..] => {
							super::push(out, 12451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2203);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5878);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12452);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12453);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19856);
									s.is_empty()
								}
								['с', s @ ..] => {
									super::push(out, 19857);
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 25876);
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
								['н', 'с', 'я', '\u{1}', s @ ..] => {
									super::push(out, 19858);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19859);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5879);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12454);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12455);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2204);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2205);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5880);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12456);
							match s {
								[] => true,
								['и', 'о', 'р', 'п', 'о', s @ ..] => {
									super::push(out, 19860);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12457);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 19861);
									s.is_empty()
								}
								['к', s @ ..] => {
									super::push(out, 19862);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19863);
									match s {
										[] => true,
										['п', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29655);
													s.is_empty()
												}
												['е', s @ ..] => {
													super::push(out, 29656);
													match s {
														[] => true,
														['н', s @ ..] => super::t6::n33280(s, out),
														_ => false,
													}
												}
												['о', s @ ..] => {
													super::push(out, 29657);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['х', s @ ..] => {
									super::push(out, 19864);
									match s {
										[] => true,
										['р', s @ ..] => {
											super::push(out, 25877);
											match s {
												[] => true,
												['е', s @ ..] => {
													super::push(out, 29658);
													s.is_empty()
												}
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
							super::push(out, 12458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5881);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 5882);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5883);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12459);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19865);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 25878);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12460);
							match s {
								[] => true,
								['в', 'и', 'ж', 'е', s @ ..] => {
									super::push(out, 19866);
									match s {
										[] => true,
										['ц', s @ ..] => {
											super::push(out, 25879);
											s.is_empty()
										}
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
					super::push(out, 5884);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12461);
							s.is_empty()
						}
						['б', s @ ..] => {
							super::push(out, 12462);
							s.is_empty()
						}
						['д', 'о', 'п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19867);
									s.is_empty()
								}
								['е', s @ ..] => {
									super::push(out, 19868);
									s.is_empty()
								}
								_ => false,
							}
						}
						['е', s @ ..] => {
							super::push(out, 12463);
							match s {
								[] => true,
								['р', s @ ..] => {
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25880);
											s.is_empty()
										}
										['п', s @ ..] => super::t6::n27163(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12464);
							s.is_empty()
						}
						['с', s @ ..] => super::t6::n13080(s, out),
						['у', 'е', s @ ..] => super::t6::n13081(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => super::t6::n2282(s, out),
		['у', s @ ..] => super::t6::n2283(s, out),
		['я', s @ ..] => super::t6::n2284(s, out),
		_ => false,
	}
}

/// The word разящий.
pub(crate) fn n13087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12472);
	s.is_empty()
}

/// Words ending -бразящий.
pub(crate) fn n13088(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12473);
	s.is_empty()
}

/// Words ending -езящий.
pub(crate) fn n2287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2211);
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5896);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5897);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -озящий.
pub(crate) fn n2288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2212);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 5898);
			match s {
				[] => true,
				['з', s @ ..] => {
					super::push(out, 12475);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 12476);
					s.is_empty()
				}
				['о', 'д', s @ ..] => {
					super::push(out, 12477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 5899);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 12478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 5900);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 12479);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 12480);
					match s {
						[] => true,
						['м', s @ ..] => {
							super::push(out, 19872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рзящий.
pub(crate) fn n2289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2213);
	s.is_empty()
}

/// Words ending -узящий.
pub(crate) fn n2290(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['б', s @ ..] => {
			super::push(out, 5901);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 5902);
			s.is_empty()
		}
		['ф', s @ ..] => {
			super::push(out, 5903);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 12481);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ьзящий.
pub(crate) fn n2291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2214);
	match s {
		[] => true,
		['л', 'о', 'к', 'с', '\u{1}', s @ ..] => {
			super::push(out, 5904);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -лящий.
pub(crate) fn n849(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 836);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2215);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5905);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12482);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19873);
									s.is_empty()
								}
								_ => false,
							}
						}
						['х', s @ ..] => {
							super::push(out, 12483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5906);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5907);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 5908);
					s.is_empty()
				}
				['к', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12484);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12485);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 5909);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5910);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5911);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5912);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5913);
					s.is_empty()
				}
				['ч', s @ ..] => {
					super::push(out, 5914);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 5915);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2216);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5916);
					s.is_empty()
				}
				_ => false,
			}
		}
		['г', 'у', s @ ..] => {
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 5917);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5918);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12486);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2217);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5919);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5920);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5921);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 5922);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 5923);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ц', s @ ..] => {
					super::push(out, 5924);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12489);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2219);
			s.is_empty()
		}
		['и', s @ ..] => {
			super::push(out, 2220);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2221);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5925);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2222);
			s.is_empty()
		}
		['о', s @ ..] => {
			super::push(out, 2223);
			match s {
				[] => true,
				['б', 'а', s @ ..] => {
					super::push(out, 5926);
					s.is_empty()
				}
				['в', s @ ..] => {
					super::push(out, 5927);
					match s {
						[] => true,
						['з', s @ ..] => {
							super::push(out, 12490);
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 19874);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 5928);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5929);
					s.is_empty()
				}
				['с', 'у', s @ ..] => {
					super::push(out, 5930);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 5931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', 'е', 'т', '\u{1}', s @ ..] => {
			super::push(out, 2224);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2225);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5932);
					s.is_empty()
				}
				['у', s @ ..] => {
					super::push(out, 5933);
					s.is_empty()
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2226);
			match s {
				[] => true,
				['ы', s @ ..] => {
					super::push(out, 5934);
					match s {
						[] => true,
						['м', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19875);
									s.is_empty()
								}
								['о', s @ ..] => {
									match s {
										[] => true,
										['в', s @ ..] => {
											super::push(out, 25882);
											s.is_empty()
										}
										['г', 'а', 'л', 'б', '\u{1}', s @ ..] => {
											super::push(out, 25883);
											s.is_empty()
										}
										['к', s @ ..] => {
											super::push(out, 25884);
											match s {
												[] => true,
												['а', s @ ..] => {
													super::push(out, 29662);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2227);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2228);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5935);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 5936);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5937);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5938);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5939);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 5940);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 2229);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 2230);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5941);
					s.is_empty()
				}
				['ы', s @ ..] => {
					super::push(out, 5942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ы', s @ ..] => {
			super::push(out, 2231);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 5943);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ю', s @ ..] => {
			super::push(out, 2232);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2233);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -мящий.
pub(crate) fn n850(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 837);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2234);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2235);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 5944);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 5945);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12493);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2236);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5946);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5947);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 5948);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12495);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5949);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2237);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2238);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2239);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -нящий.
pub(crate) fn n851(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 838);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2240);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 5951);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 5952);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 5953);
					s.is_empty()
				}
				['м', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12496);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12497);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12498);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5954);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12500);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 19876);
									s.is_empty()
								}
								_ => false,
							}
						}
						['б', s @ ..] => {
							super::push(out, 12501);
							s.is_empty()
						}
						['г', s @ ..] => {
							super::push(out, 12502);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12503);
							s.is_empty()
						}
						['х', s @ ..] => {
							super::push(out, 12504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2241);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 5955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2242);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5956);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5957);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2243);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5958);
					match s {
						[] => true,
						['з', '\u{1}', s @ ..] => {
							super::push(out, 12506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5959);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 5960);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5961);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5962);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5963);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 5964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2244);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2245);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5966);
					s.is_empty()
				}
				['ж', s @ ..] => {
					super::push(out, 5967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2246);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 5968);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 5969);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2247);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5970);
					s.is_empty()
				}
				['и', s @ ..] => {
					super::push(out, 5971);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 5972);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2248);
			s.is_empty()
		}
		['с', s @ ..] => {
			super::push(out, 2249);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2250);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2251);
			match s {
				[] => true,
				['у', s @ ..] => {
					super::push(out, 5974);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 5975);
					match s {
						[] => true,
						['п', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19877);
									s.is_empty()
								}
								['о', s @ ..] => {
									super::push(out, 19878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -оящий.
pub(crate) fn n852(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 839);
	match s {
		[] => true,
		['в', s @ ..] => {
			super::push(out, 2252);
			s.is_empty()
		}
		['д', s @ ..] => {
			super::push(out, 2253);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2254);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 5976);
					s.is_empty()
				}
				_ => false,
			}
		}
		['л', s @ ..] => {
			super::push(out, 2255);
			s.is_empty()
		}
		['н', s @ ..] => {
			super::push(out, 2256);
			s.is_empty()
		}
		['п', s @ ..] => {
			super::push(out, 2257);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2258);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5977);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 5978);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2259);
			match s {
				[] => true,
				['с', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12512);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12513);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19879);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 25885);
											s.is_empty()
										}
										['е', s @ ..] => {
											match s {
												[] => true,
												['н', '\u{1}', s @ ..] => {
													super::push(out, 29663);
													s.is_empty()
												}
												['р', s @ ..] => {
													super::push(out, 29664);
													s.is_empty()
												}
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
							super::push(out, 12514);
							s.is_empty()
						}
						['д', s @ ..] => {
							super::push(out, 12515);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12516);
							match s {
								[] => true,
								['ж', s @ ..] => {
									super::push(out, 19880);
									match s {
										[] => true,
										['и', s @ ..] => {
											super::push(out, 25886);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ш', s @ ..] => {
									super::push(out, 19881);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12517);
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19882);
									s.is_empty()
								}
								['г', s @ ..] => {
									super::push(out, 19883);
									match s {
										[] => true,
										['о', s @ ..] => {
											super::push(out, 25887);
											s.is_empty()
										}
										_ => false,
									}
								}
								['к', s @ ..] => {
									super::push(out, 19884);
									s.is_empty()
								}
								['л', s @ ..] => {
									super::push(out, 19885);
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 25888);
											s.is_empty()
										}
										_ => false,
									}
								}
								['с', s @ ..] => {
									super::push(out, 19886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12518);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -пящий.
pub(crate) fn n853(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 840);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2260);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2261);
			match s {
				[] => true,
				['л', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12519);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 5979);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2262);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5980);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 5981);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 5982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2263);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 5983);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 5984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2264);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5985);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 5986);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 5987);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12522);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2265);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2266);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 5988);
					s.is_empty()
				}
				['т', '\u{1}', s @ ..] => {
					super::push(out, 5989);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -рящий.
pub(crate) fn n854(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 841);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2267);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 5990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 5991);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ж', s @ ..] => {
					super::push(out, 5992);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 5993);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 5994);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 5995);
					match s {
						[] => true,
						['ш', s @ ..] => {
							super::push(out, 12525);
							s.is_empty()
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 5996);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 5997);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 5998);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2268);
			s.is_empty()
		}
		['г', s @ ..] => {
			super::push(out, 2269);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 5999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['д', s @ ..] => {
			super::push(out, 2270);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6000);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6001);
					match s {
						[] => true,
						['б', '\u{1}', s @ ..] => {
							super::push(out, 12527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2271);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6003);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6004);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6005);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6006);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12528);
							s.is_empty()
						}
						['с', s @ ..] => {
							super::push(out, 12529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 6007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2272);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			match s {
				[] => true,
				['м', s @ ..] => {
					super::push(out, 6009);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 6010);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2273);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6011);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12530);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 19888);
									match s {
										[] => true,
										['о', 'к', s @ ..] => {
											super::push(out, 25889);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29665);
													match s {
														[] => true,
														['а', s @ ..] => super::t6::n33281(s, out),
														_ => false,
													}
												}
												['с', s @ ..] => {
													super::push(out, 29666);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 19889);
									match s {
										[] => true,
										['п', s @ ..] => {
											super::push(out, 25890);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['т', s @ ..] => {
							super::push(out, 12531);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19890);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 6012);
					s.is_empty()
				}
				['з', s @ ..] => {
					super::push(out, 6013);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6014);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6015);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6016);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12533);
							match s {
								[] => true,
								['т', s @ ..] => {
									super::push(out, 19891);
									s.is_empty()
								}
								_ => false,
							}
						}
						['с', s @ ..] => {
							super::push(out, 12534);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12535);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 19892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6017);
					match s {
						[] => true,
						['с', s @ ..] => {
							super::push(out, 12536);
							s.is_empty()
						}
						['у', s @ ..] => {
							super::push(out, 12537);
							match s {
								[] => true,
								['м', s @ ..] => {
									super::push(out, 19893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12538);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12539);
							s.is_empty()
						}
						['в', s @ ..] => {
							super::push(out, 12540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2274);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6019);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6020);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2275);
			match s {
				[] => true,
				['б', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12542);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['д', s @ ..] => {
					super::push(out, 6022);
					s.is_empty()
				}
				['ж', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12544);
							s.is_empty()
						}
						['е', s @ ..] => {
							super::push(out, 12545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6023);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12546);
							match s {
								[] => true,
								['д', s @ ..] => {
									super::push(out, 19894);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6024);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6025);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6026);
					s.is_empty()
				}
				['щ', s @ ..] => {
					super::push(out, 6027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2276);
			s.is_empty()
		}
		['ы', s @ ..] => {
			super::push(out, 2277);
			s.is_empty()
		}
		['я', 'у', s @ ..] => {
			super::push(out, 2278);
			match s {
				[] => true,
				['х', s @ ..] => {
					super::push(out, 6028);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -сящий.
pub(crate) fn n855(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 842);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2279);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6029);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12547);
							s.is_empty()
						}
						['л', s @ ..] => {
							super::push(out, 12548);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19895);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6030);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 6031);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12550);
							match s {
								[] => true,
								['р', s @ ..] => {
									super::push(out, 19896);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['л', s @ ..] => {
					super::push(out, 6032);
					match s {
						[] => true,
						['г', s @ ..] => {
							super::push(out, 12551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6033);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12552);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			super::push(out, 2280);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6034);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['в', s @ ..] => {
					super::push(out, 6035);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6036);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2281);
			match s {
				[] => true,
				['в', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12554);
							s.is_empty()
						}
						['а', s @ ..] => {
							super::push(out, 12555);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19897);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25891);
											match s {
												[] => true,
												['н', s @ ..] => {
													super::push(out, 29667);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2282);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2283);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6039);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6040);
					s.is_empty()
				}
				['н', s @ ..] => {
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12556);
							s.is_empty()
						}
						['о', s @ ..] => {
							super::push(out, 12557);
							match s {
								[] => true,
								['п', s @ ..] => {
									super::push(out, 19898);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ы', 'в', 'я', s @ ..] => {
							super::push(out, 12558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', 'о', s @ ..] => {
					super::push(out, 6041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2284);
			match s {
				[] => true,
				['я', s @ ..] => {
					super::push(out, 6042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2285);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2286);
			match s {
				[] => true,
				['р', 'т', '\u{1}', s @ ..] => {
					super::push(out, 6043);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -тящий.
pub(crate) fn n856(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2287);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6044);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6045);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12559);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6046);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6047);
					s.is_empty()
				}
				['п', s @ ..] => {
					super::push(out, 6048);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6049);
					s.is_empty()
				}
				['х', s @ ..] => {
					super::push(out, 6050);
					s.is_empty()
				}
				_ => false,
			}
		}
		['е', s @ ..] => {
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6051);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6052);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12560);
							match s {
								[] => true,
								['к', s @ ..] => {
									super::push(out, 19899);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19900);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 25892);
											s.is_empty()
										}
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
					super::push(out, 6053);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6054);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 6055);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12562);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['н', s @ ..] => {
			super::push(out, 2288);
			s.is_empty()
		}
		['о', s @ ..] => {
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6056);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12563);
							match s {
								[] => true,
								['з', s @ ..] => {
									super::push(out, 19901);
									s.is_empty()
								}
								['р', s @ ..] => {
									super::push(out, 19902);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['г', s @ ..] => {
					super::push(out, 6057);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6058);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6059);
					match s {
						[] => true,
						['д', s @ ..] => {
							super::push(out, 12565);
							match s {
								[] => true,
								['а', s @ ..] => {
									super::push(out, 19903);
									s.is_empty()
								}
								_ => false,
							}
						}
						['о', s @ ..] => {
							super::push(out, 12566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 6060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2289);
			s.is_empty()
		}
		['р', s @ ..] => {
			super::push(out, 2290);
			match s {
				[] => true,
				['е', s @ ..] => {
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12567);
							s.is_empty()
						}
						['ч', '\u{1}', s @ ..] => {
							super::push(out, 12568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6061);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2291);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6062);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6063);
					match s {
						[] => true,
						['л', 'б', s @ ..] => {
							super::push(out, 12570);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19904);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6064);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12571);
							s.is_empty()
						}
						['ч', s @ ..] => {
							super::push(out, 12572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6065);
					s.is_empty()
				}
				['о', s @ ..] => {
					super::push(out, 6066);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12573);
							match s {
								[] => true,
								['х', s @ ..] => {
									super::push(out, 19905);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6067);
					match s {
						[] => true,
						['р', 'х', '\u{1}', s @ ..] => {
							super::push(out, 12574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ь', s @ ..] => {
					super::push(out, 6068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6069);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6070);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12575);
							match s {
								[] => true,
								['л', s @ ..] => {
									super::push(out, 19906);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['р', s @ ..] => {
					super::push(out, 6071);
					s.is_empty()
				}
				['ш', s @ ..] => {
					super::push(out, 6072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['х', s @ ..] => {
			super::push(out, 2292);
			s.is_empty()
		}
		['ч', s @ ..] => {
			super::push(out, 2293);
			s.is_empty()
		}
		['я', s @ ..] => {
			super::push(out, 2294);
			match s {
				[] => true,
				['п', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 12576);
							s.is_empty()
						}
						['и', s @ ..] => {
							super::push(out, 12577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -фящий.
pub(crate) fn n857(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 844);
	match s {
		[] => true,
		['й', s @ ..] => {
			super::push(out, 2295);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word темно-оранжевый.
pub(crate) fn n31130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29668);
	s.is_empty()
}

/// Words ending -то-оранжевый.
pub(crate) fn n31131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29669);
	match s {
		[] => true,
		['л', s @ ..] => {
			super::push(out, 31723);
			match s {
				[] => true,
				['е', 'ж', '\u{1}', s @ ..] => {
					super::push(out, 32792);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -левый.
pub(crate) fn n973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2410);
			s.is_empty()
		}
		['-', s @ ..] => {
			super::push(out, 2411);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['а', s @ ..] => {
			super::push(out, 2412);
			match s {
				[] => true,
				['р', s @ ..] => {
					super::push(out, 6163);
					match s {
						[] => true,
						['т', s @ ..] => {
							super::push(out, 12631);
							match s {
								[] => true,
								['ь', s @ ..] => {
									super::push(out, 19931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['т', s @ ..] => {
					super::push(out, 6164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2413);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6165);
					s.is_empty()
				}
				['м', s @ ..] => {
					super::push(out, 6166);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12632);
							match s {
								[] => true,
								['с', s @ ..] => {
									super::push(out, 19932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['у', s @ ..] => {
					super::push(out, 6167);
					match s {
						[] => true,
						['р', s @ ..] => {
							match s {
								[] => true,
								['и', s @ ..] => {
									super::push(out, 19933);
									s.is_empty()
								}
								['т', s @ ..] => {
									super::push(out, 19934);
									match s {
										[] => true,
										['о', 'с', s @ ..] => {
											match s {
												[] => true,
												['и', s @ ..] => {
													super::push(out, 29670);
													s.is_empty()
												}
												['х', s @ ..] => {
													super::push(out, 29671);
													match s {
														[] => true,
														['е', s @ ..] => super::t6::n33283(s, out),
														['у', s @ ..] => super::t6::n33284(s, out),
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
									super::push(out, 19935);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25898);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29672);
													s.is_empty()
												}
												_ => false,
											}
										}
										['у', s @ ..] => {
											super::push(out, 25899);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29673);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2414);
			s.is_empty()
		}
		['е', s @ ..] => {
			super::push(out, 2415);
			match s {
				[] => true,
				['в', s @ ..] => {
					super::push(out, 6168);
					s.is_empty()
				}
				['г', s @ ..] => {
					super::push(out, 6169);
					s.is_empty()
				}
				['д', s @ ..] => {
					super::push(out, 6170);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12633);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6171);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6172);
					match s {
						[] => true,
						['а', 'л', 'ф', '\u{1}', s @ ..] => {
							super::push(out, 12635);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6173);
					s.is_empty()
				}
				['ц', s @ ..] => {
					super::push(out, 6174);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['и', s @ ..] => {
			super::push(out, 2416);
			match s {
				[] => true,
				['н', s @ ..] => {
					super::push(out, 6176);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6177);
					match s {
						[] => true,
						['к', s @ ..] => {
							super::push(out, 12637);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['й', s @ ..] => {
			super::push(out, 2417);
			s.is_empty()
		}
		['к', s @ ..] => {
			super::push(out, 2418);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2419);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6179);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2420);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6181);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6182);
					match s {
						[] => true,
						['в', 'у', s @ ..] => {
							super::push(out, 12640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['м', s @ ..] => {
					super::push(out, 6183);
					s.is_empty()
				}
				['н', s @ ..] => {
					super::push(out, 6184);
					s.is_empty()
				}
				['с', s @ ..] => {
					super::push(out, 6185);
					s.is_empty()
				}
				['т', s @ ..] => {
					super::push(out, 6186);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12641);
							match s {
								[] => true,
								['н', s @ ..] => {
									super::push(out, 19936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['х', s @ ..] => {
					super::push(out, 6187);
					match s {
						[] => true,
						['у', s @ ..] => {
							super::push(out, 12642);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['п', s @ ..] => {
			super::push(out, 2421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6188);
					s.is_empty()
				}
				['а', s @ ..] => {
					super::push(out, 6189);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12643);
							s.is_empty()
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6190);
					match s {
						[] => true,
						['н', s @ ..] => {
							super::push(out, 12644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['р', s @ ..] => {
			super::push(out, 2422);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6191);
					s.is_empty()
				}
				['е', s @ ..] => {
					super::push(out, 6192);
					match s {
						[] => true,
						['п', s @ ..] => {
							super::push(out, 12645);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2423);
			match s {
				[] => true,
				['о', s @ ..] => {
					super::push(out, 6193);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2424);
			match s {
				[] => true,
				['б', s @ ..] => {
					super::push(out, 6194);
					s.is_empty()
				}
				['к', s @ ..] => {
					super::push(out, 6195);
					match s {
						[] => true,
						['а', s @ ..] => {
							super::push(out, 12647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['н', s @ ..] => {
					super::push(out, 6196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2425);
			match s {
				[] => true,
				['а', s @ ..] => {
					super::push(out, 6197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['э', s @ ..] => {
			super::push(out, 2426);
			s.is_empty()
		}
		['ю', s @ ..] => {
			super::push(out, 2427);
			match s {
				[] => true,
				['т', s @ ..] => {
					super::push(out, 6198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['я', s @ ..] => {
			super::push(out, 2428);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -невый.
pub(crate) fn n974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 949);
	match s {
		[] => true,
		['а', s @ ..] => {
			super::push(out, 2429);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6199);
					s.is_empty()
				}
				_ => false,
			}
		}
		['б', s @ ..] => {
			super::push(out, 2430);
			s.is_empty()
		}
		['в', s @ ..] => {
			super::push(out, 2431);
			match s {
				[] => true,
				['и', s @ ..] => {
					super::push(out, 6200);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12648);
							s.is_empty()
						}
						['р', s @ ..] => {
							super::push(out, 12649);
							match s {
								[] => true,
								['г', s @ ..] => {
									super::push(out, 19937);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['о', s @ ..] => {
					super::push(out, 6201);
					match s {
						[] => true,
						['л', s @ ..] => {
							super::push(out, 12650);
							match s {
								[] => true,
								['о', s @ ..] => {
									super::push(out, 19938);
									s.is_empty()
								}
								_ => false,
							}
						}
						['р', s @ ..] => {
							super::push(out, 12651);
							match s {
								[] => true,
								['у', s @ ..] => {
									match s {
										[] => true,
										['о', 'г', 'о', 'н', 'м', '\u{1}', s @ ..] => {
											super::push(out, 25900);
											s.is_empty()
										}
										['х', 'е', s @ ..] => {
											super::push(out, 25901);
											match s {
												[] => true,
												['р', s @ ..] => {
													super::push(out, 29674);
													match s {
														[] => true,
														['т', '\u{1}', s @ ..] => super::t6::n33285(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2432);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6202);
					s.is_empty()
				}
				['р', s @ ..] => {
					super::push(out, 6203);
					match s {
						[] => true,
						['г', 'и', s @ ..] => {
							super::push(out, 12652);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19939);
									s.is_empty()
								}
								_ => false,
							}
						}
						['и', s @ ..] => {
							super::push(out, 12653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['с', s @ ..] => {
					super::push(out, 6204);
					match s {
						[] => true,
						['я', s @ ..] => {
							super::push(out, 12654);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ш', s @ ..] => {
					super::push(out, 6205);
					match s {
						[] => true,
						['ь', s @ ..] => {
							super::push(out, 12655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ж', s @ ..] => {
			super::push(out, 2433);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['з', s @ ..] => {
			super::push(out, 2434);
			match s {
				[] => true,
				['л', s @ ..] => {
					super::push(out, 6207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['м', s @ ..] => {
			super::push(out, 2435);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6208);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['о', s @ ..] => {
			super::push(out, 2436);
			match s {
				[] => true,
				['г', s @ ..] => {
					super::push(out, 6209);
					match s {
						[] => true,
						['и', s @ ..] => {
							super::push(out, 12657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['к', s @ ..] => {
					super::push(out, 6210);
					s.is_empty()
				}
				['л', s @ ..] => {
					super::push(out, 6211);
					match s {
						[] => true,
						['б', s @ ..] => {
							super::push(out, 12658);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['с', s @ ..] => {
			super::push(out, 2437);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['т', s @ ..] => {
			super::push(out, 2438);
			s.is_empty()
		}
		['у', s @ ..] => {
			super::push(out, 2439);
			match s {
				[] => true,
				['к', s @ ..] => {
					super::push(out, 6213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ч', s @ ..] => {
			super::push(out, 2440);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6214);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6215);
					match s {
						[] => true,
						['р', 'о', 'к', '-', 'о', s @ ..] => {
							match s {
								[] => true,
								['в', s @ ..] => {
									super::push(out, 19940);
									match s {
										[] => true,
										['е', s @ ..] => {
											super::push(out, 25902);
											s.is_empty()
										}
										_ => false,
									}
								}
								['л', s @ ..] => {
									super::push(out, 19941);
									match s {
										[] => true,
										['т', s @ ..] => {
											super::push(out, 25903);
											s.is_empty()
										}
										_ => false,
									}
								}
								['н', s @ ..] => {
									super::push(out, 19942);
									match s {
										[] => true,
										['м', 'е', 'т', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 29675);
													s.is_empty()
												}
												['-', 'о', 'в', 'а', 'ж', 'р', '\u{1}', s @ ..] => {
													super::push(out, 29676);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['р', s @ ..] => {
									super::push(out, 19943);
									match s {
										[] => true,
										['у', s @ ..] => {
											super::push(out, 25904);
											s.is_empty()
										}
										_ => false,
									}
								}
								['т', s @ ..] => {
									match s {
										[] => true,
										['а', s @ ..] => {
											super::push(out, 25905);
											match s {
												[] => true,
												['в', s @ ..] => {
													super::push(out, 29677);
													s.is_empty()
												}
												_ => false,
											}
										}
										['л', 'е', 'ж', '\u{1}', s @ ..] => {
											super::push(out, 25906);
											s.is_empty()
										}
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
					super::push(out, 6216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ш', s @ ..] => {
			super::push(out, 2441);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6217);
					match s {
						[] => true,
						['р', s @ ..] => {
							super::push(out, 12660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['и', s @ ..] => {
					super::push(out, 6218);
					match s {
						[] => true,
						['в', s @ ..] => {
							super::push(out, 12661);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 19944);
									s.is_empty()
								}
								['-', s @ ..] => {
									super::push(out, 19945);
									match s {
										[] => true,
										['о', 'н', 'м', 'е', 'т', '\u{1}', s @ ..] => {
											super::push(out, 25907);
											s.is_empty()
										}
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
					super::push(out, 6219);
					match s {
						[] => true,
						['о', s @ ..] => {
							super::push(out, 12662);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -месевый.
pub(crate) fn n6490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6223);
	s.is_empty()
}

/// Words ending -осевый.
pub(crate) fn n2540(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2447);
	s.is_empty()
}

/// Words ending -тевый.
pub(crate) fn n978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 953);
	match s {
		[] => true,
		['г', s @ ..] => {
			super::push(out, 2448);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['у', s @ ..] => {
			super::push(out, 2449);
			match s {
				[] => true,
				['п', s @ ..] => {
					super::push(out, 6225);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ф', s @ ..] => {
			super::push(out, 2450);
			match s {
				[] => true,
				['ю', s @ ..] => {
					super::push(out, 6226);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -уевый.
pub(crate) fn n979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	match s {
		[] => true,
		['т', s @ ..] => {
			super::push(out, 2451);
			s.is_empty()
		}
		['х', s @ ..] => {
			super::push(out, 2452);
			match s {
				[] => true,
				['е', s @ ..] => {
					super::push(out, 6227);
					s.is_empty()
				}
				['й', s @ ..] => {
					super::push(out, 6228);
					match s {
						[] => true,
						['е', s @ ..] => {
							super::push(out, 12666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

