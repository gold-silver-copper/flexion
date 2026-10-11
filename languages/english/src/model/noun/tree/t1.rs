use flexion::Path;

/// Words ending -ya.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4169);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 4170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1502);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4171);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 7063);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1503);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1504);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1505);
					match s {
						[] => true,
						['b', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 4172);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4173);
							match s {
								[] => true,
								['r', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7064);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4174);
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 7065);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'o', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['t', '\u{1}', s @ ..] => {
							super::push(out, 4175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 1507);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1508);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['a', 'k', 'a', 'z', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1509);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['n', 'a', 'g', 'f', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4176);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4177);
							match s {
								[] => true,
								['a', 'r', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7066);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['u', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4178);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -za.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['g', 'n', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 1512);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4179);
							s.is_empty()
						}
						['r', 'a', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4180);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4181);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4182);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7067);
									match s {
										[] => true,
										['g', 'a', 'n', '\u{1}', s @ ..] => {
											super::push(out, 9187);
											s.is_empty()
										}
										['m', 'i', 'h', 's', '\u{1}', s @ ..] => {
											super::push(out, 9188);
											s.is_empty()
										}
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
					super::push(out, 1515);
					match s {
						[] => true,
						['h', 'r', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9189);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 9190);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 10386);
													match s {
														[] => true,
														['i', 'p', '\u{1}', s @ ..] => super::t2::n12092(s, out),
														['o', 'c', '\u{1}', s @ ..] => super::t2::n12093(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'o', s @ ..] => {
									match s {
										[] => true,
										['c', 'y', 'm', 'o', 't', 'c', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9191);
											s.is_empty()
										}
										['e', 'l', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9192);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1516);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1517);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1519);
					match s {
						[] => true,
						['k', 'a', 'y', '\u{1}', s @ ..] => {
							super::push(out, 4184);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['a', 'r', 'a', 'p', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ned (3 syllables).
pub(crate) fn n1695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1568);
	match s {
		[] => true,
		['e', 't', 'h', 'g', 'i', 'l', 'n', 'e', '\u{1}', s @ ..] => {
			super::push(out, 4217);
			s.is_empty()
		}
		['g', 'i', 's', s @ ..] => {
			match s {
				[] => true,
				['e', 'v', 'o', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 7078);
					s.is_empty()
				}
				['r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 7079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 4218);
			s.is_empty()
		}
		['m', 'e', 'd', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4219);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sed (3 syllables).
pub(crate) fn n1698(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1571);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4220);
			match s {
				[] => true,
				['e', 'c', 'e', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9198);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 9199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -id.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 557);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1572);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1573);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4221);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1575);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4222);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 1577);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['r', 'e', s @ ..] => {
							super::push(out, 4223);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4225);
							match s {
								[] => true,
								['o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7080);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4226);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 1582);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1583);
					match s {
						[] => true,
						['a', 'i', s @ ..] => {
							super::push(out, 4227);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4228);
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 7081);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1584);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1585);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 4229);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', 'h', s @ ..] => {
							super::push(out, 4230);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7083);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1587);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4232);
							match s {
								[] => true,
								['j', 'u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7085);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'u', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4233);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1591);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4236);
							match s {
								[] => true,
								['y', s @ ..] => {
									super::push(out, 7086);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['q', 's', 'e', 'r', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ld.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 560);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 561);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4238);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7087);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7088);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['u', 'r', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 562);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1597);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 4239);
							s.is_empty()
						}
						['t', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4240);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['h', 't', 'i', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7089);
									s.is_empty()
								}
								['r', 'e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7090);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4241);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['n', 'i', 'a', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9200);
											s.is_empty()
										}
										['s', 'h', 't', 'o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 9201);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4242);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 7091);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1599);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1601);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4243);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4244);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1603);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1604);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word wmd (0 syllables).
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 564);
	s.is_empty()
}

/// Words ending -nd.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 565);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 1605);
					s.is_empty()
				}
				['i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 566);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1607);
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 4245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1608);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1609);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4246);
							match s {
								[] => true,
								['s', 'e', 'e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 567);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1610);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4247);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7095);
									match s {
										[] => true,
										['e', 'd', 'n', 'u', 'w', '\u{1}', s @ ..] => {
											super::push(out, 9202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -od.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 568);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1611);
					s.is_empty()
				}
				['e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1612);
					s.is_empty()
				}
				['r', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 569);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1614);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1615);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 570);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1617);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1618);
					match s {
						[] => true,
						['d', 'u', 'm', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4248);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', 'u', s @ ..] => {
									super::push(out, 7096);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 7097);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'b', 'e', 'r', 'r', 'o', 'm', 's', '\u{1}', s @ ..] => {
					super::push(out, 1619);
					s.is_empty()
				}
				['t', 's', 'e', 'b', 'e', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 1620);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pd.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['\u{10}', 'b', '\u{1}', s @ ..] => {
			super::push(out, 571);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rd.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 572);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 573);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4249);
							match s {
								[] => true,
								['c', 'l', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7098);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4250);
							match s {
								[] => true,
								['r', 'u', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7099);
									s.is_empty()
								}
								['y', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 7100);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 4251);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7101);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ud.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', 'b', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 4252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1623);
					s.is_empty()
				}
				['w', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1624);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 574);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4253);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 575);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4254);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4255);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1627);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zd.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	s.is_empty()
}

/// The word phallace (3 syllables).
pub(crate) fn n11420(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10387);
	s.is_empty()
}

/// Words ending -ee.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 585);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 586);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1653);
					s.is_empty()
				}
				['r', 'k', 'i', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['g', 'o', s @ ..] => {
					super::push(out, 1656);
					s.is_empty()
				}
				['k', 'o', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['r', 'a', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'h', 's', 'o', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1658);
					s.is_empty()
				}
				['r', 'a', 'g', 'u', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['s', 'a', 'm', s @ ..] => {
					super::push(out, 1660);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1661);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4279);
							match s {
								[] => true,
								['b', 'e', 'e', 'b', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fe.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 588);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 589);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1663);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4280);
							s.is_empty()
						}
						['n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4281);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 4282);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4283);
							s.is_empty()
						}
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4285);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 4286);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 590);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1664);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4287);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4288);
							s.is_empty()
						}
						['z', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1665);
					match s {
						[] => true,
						['e', 'l', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4290);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7113);
									s.is_empty()
								}
								['l', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7114);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4291);
							s.is_empty()
						}
						['n', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4292);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 1666);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4293);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7115);
									s.is_empty()
								}
								['w', 'r', 'e', 'd', 'n', 'u', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7116);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1667);
					match s {
						[] => true,
						['e', 'f', 'p', 'h', 'c', 's', 'u', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4294);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['h', 'g', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7117);
									s.is_empty()
								}
								['w', 'o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 7118);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4295);
							s.is_empty()
						}
						['r', 't', 's', 'e', 's', 'o', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4296);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 4297);
							match s {
								[] => true,
								['s', 'u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1668);
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4298);
							s.is_empty()
						}
						['t', 'r', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4299);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ge.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 591);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1669);
					s.is_empty()
				}
				['d', 'n', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1670);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1671);
					match s {
						[] => true,
						['i', 'z', '\u{1}', s @ ..] => {
							super::push(out, 4300);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1672);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 592);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1673);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4302);
							match s {
								[] => true,
								['a', 'm', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7120);
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 7121);
									match s {
										[] => true,
										['o', 'l', '\u{1}', s @ ..] => {
											super::push(out, 9205);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 7122);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'h', 'c', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4303);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4304);
							match s {
								[] => true,
								['e', 's', 'n', 'a', 'r', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4305);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4306);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 7124);
									match s {
										[] => true,
										['c', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 9206);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7125);
									s.is_empty()
								}
								['n', 'o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7126);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7127);
									match s {
										[] => true,
										['h', 'c', s @ ..] => {
											super::push(out, 9207);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 10388);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n12094(s, out),
														['o', '\u{1}', s @ ..] => super::t2::n12095(s, out),
														_ => false,
													}
												}
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
							super::push(out, 4307);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['m', 'i', 'r', 'a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 9208);
											s.is_empty()
										}
										['r', 'u', 'g', 'a', 'y', '\u{1}', s @ ..] => {
											super::push(out, 9209);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 7128);
									match s {
										[] => true,
										['k', 'a', 'k', '\u{1}', s @ ..] => {
											super::push(out, 9210);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 9211);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 10389);
													s.is_empty()
												}
												['o', 'k', '\u{1}', s @ ..] => {
													super::push(out, 10390);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 7129);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9212);
											s.is_empty()
										}
										['b', 'u', 'k', '\u{1}', s @ ..] => {
											super::push(out, 9213);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 9214);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 10391);
													match s {
														[] => true,
														['a', s @ ..] => super::t2::n12096(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['m', 'a', 'k', 'u', 's', 't', '\u{1}', s @ ..] => {
											super::push(out, 9215);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 9216);
											match s {
												[] => true,
												['u', s @ ..] => {
													super::push(out, 10392);
													match s {
														[] => true,
														['k', s @ ..] => super::t2::n12097(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', 'k', 'u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9217);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 7130);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 7131);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 9218);
											match s {
												[] => true,
												['c', 'i', 'n', '\u{1}', s @ ..] => {
													super::push(out, 10393);
													s.is_empty()
												}
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
							super::push(out, 4308);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4309);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4310);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4311);
							match s {
								[] => true,
								['i', 'm', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7132);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1674);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							super::push(out, 4312);
							match s {
								[] => true,
								['l', 'o', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'k', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				['n', s @ ..] => super::t2::n1806(s, out),
				['o', s @ ..] => super::t2::n1807(s, out),
				['r', s @ ..] => {
					super::push(out, 1676);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -he.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 593);
			match s {
				[] => true,
				['c', 'r', s @ ..] => {
					super::push(out, 1677);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							super::push(out, 4318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['i', 's', '\u{1}', s @ ..] => {
							super::push(out, 4319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1679);
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', s @ ..] => {
					super::push(out, 1680);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 4321);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 594);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4322);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7135);
									match s {
										[] => true,
										['p', 's', 'r', 'u', '\u{1}', s @ ..] => {
											super::push(out, 9219);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['b', 'a', 'b', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4323);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4324);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7136);
									match s {
										[] => true,
										['t', 's', 'n', 'e', 't', 'n', 'i', 't', '\u{1}', s @ ..] => {
											super::push(out, 9220);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'a', 'h', 'c', 't', '\u{1}', s @ ..] => {
									super::push(out, 7137);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'p', s @ ..] => {
							super::push(out, 4325);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', 'r', 'a', 'i', 'a', 'p', 'u', 't', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1681);
					s.is_empty()
				}
				['g', 'a', 'r', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1683);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ie.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 595);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 596);
			match s {
				[] => true,
				['g', 'n', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1685);
					s.is_empty()
				}
				['l', 'e', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4327);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 7139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4328);
							match s {
								[] => true,
								['t', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'p', 'a', 'r', s @ ..] => {
					super::push(out, 1688);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 1689);
					match s {
						[] => true,
						['u', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 597);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1690);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4331);
							match s {
								[] => true,
								['e', 's', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7141);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'l', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4332);
							match s {
								[] => true,
								['u', 'q', s @ ..] => {
									match s {
										[] => true,
										['o', 'n', 's', '\u{1}', s @ ..] => {
											super::push(out, 9221);
											s.is_empty()
										}
										['u', 'n', 's', '\u{1}', s @ ..] => {
											super::push(out, 9222);
											s.is_empty()
										}
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
					super::push(out, 1692);
					match s {
						[] => true,
						['a', 'v', 'i', 'h', 'z', 'e', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['a', 'g', s @ ..] => {
							super::push(out, 4334);
							match s {
								[] => true,
								['u', s @ ..] => {
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 9223);
											s.is_empty()
										}
										['y', '\u{1}', s @ ..] => {
											super::push(out, 9224);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ke.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 599);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1695);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4335);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7142);
									match s {
										[] => true,
										['h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 9225);
											s.is_empty()
										}
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
					super::push(out, 1696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 600);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4336);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4337);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4338);
							match s {
								[] => true,
								['i', 'k', 'o', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 7143);
									s.is_empty()
								}
								['u', 'k', 'i', 'h', 's', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7144);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4339);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7145);
									match s {
										[] => true,
										['h', 'c', 'u', '\u{1}', s @ ..] => {
											super::push(out, 9226);
											s.is_empty()
										}
										['r', 'u', 'k', 'o', '\u{1}', s @ ..] => {
											super::push(out, 9227);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7146);
									s.is_empty()
								}
								['o', 't', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 7147);
									s.is_empty()
								}
								['u', 'z', 'a', 'w', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 4340);
							match s {
								[] => true,
								['i', 'h', 'c', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7149);
									s.is_empty()
								}
								['k', 'i', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4341);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4342);
							s.is_empty()
						}
						['t', 'i', s @ ..] => {
							super::push(out, 4343);
							match s {
								[] => true,
								['i', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4344);
							s.is_empty()
						}
						['w', 'i', s @ ..] => {
							super::push(out, 4345);
							match s {
								[] => true,
								['k', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 7152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1698);
					match s {
						[] => true,
						['r', 'o', 't', 'i', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1699);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4347);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1700);
					match s {
						[] => true,
						['a', 'k', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4348);
							s.is_empty()
						}
						['o', 'r', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1701);
					match s {
						[] => true,
						['i', 'h', 'c', 's', 't', 'a', 'l', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4350);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1702);
					match s {
						[] => true,
						['c', 'i', 't', s @ ..] => {
							super::push(out, 4351);
							s.is_empty()
						}
						['r', 'a', 's', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1703);
					match s {
						[] => true,
						['s', 'n', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4353);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4354);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7153);
									s.is_empty()
								}
								['o', 's', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 601);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1704);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 4355);
							match s {
								[] => true,
								['q', '\u{1}', s @ ..] => {
									super::push(out, 7155);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1705);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1706);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 4356);
							s.is_empty()
						}
						['i', 't', '\u{1}', s @ ..] => {
							super::push(out, 4357);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 4359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4360);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1709);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1710);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1711);
					s.is_empty()
				}
				['z', 't', s @ ..] => {
					super::push(out, 1712);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 602);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1713);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4361);
							s.is_empty()
						}
						['d', 'e', s @ ..] => {
							super::push(out, 4362);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 7156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4363);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4364);
							match s {
								[] => true,
								['b', 'i', 't', '\u{1}', s @ ..] => {
									super::push(out, 7157);
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 9228);
											s.is_empty()
										}
										['o', 'p', 'a', 't', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9229);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4365);
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4366);
							match s {
								[] => true,
								['i', 'g', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7159);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4367);
							match s {
								[] => true,
								['o', 'l', 'o', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 7160);
									s.is_empty()
								}
								['r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7161);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 't', s @ ..] => {
							super::push(out, 4368);
							match s {
								[] => true,
								['n', 'e', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7162);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'r', s @ ..] => {
							super::push(out, 4369);
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 7163);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 1714);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4370);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4371);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7164);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4372);
							s.is_empty()
						}
						['z', 'n', 'e', 'b', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['b', 'a', s @ ..] => {
							super::push(out, 4374);
							match s {
								[] => true,
								['r', 'o', 'm', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7165);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 4375);
							match s {
								[] => true,
								['n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4376);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4377);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4378);
							s.is_empty()
						}
						['t', 'r', 'o', s @ ..] => {
							super::push(out, 4379);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 4380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7167);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9230);
											match s {
												[] => true,
												['k', 'i', 'r', 'f', '\u{1}', s @ ..] => {
													super::push(out, 10394);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', 'a', s @ ..] => {
									super::push(out, 7168);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7169);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7170);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9231);
											match s {
												[] => true,
												['i', 'l', 'g', 'a', 't', '\u{1}', s @ ..] => {
													super::push(out, 10395);
													s.is_empty()
												}
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
							super::push(out, 4381);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 7171);
									match s {
										[] => true,
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9232);
											s.is_empty()
										}
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
					super::push(out, 1719);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4382);
							match s {
								[] => true,
								['a', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7172);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4383);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4384);
							match s {
								[] => true,
								['e', 'l', 'o', 'h', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7173);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4385);
							s.is_empty()
						}
						['t', s @ ..] => super::t2::n4701(s, out),
						_ => false,
					}
				}
				['u', s @ ..] => super::t2::n1853(s, out),
				['y', s @ ..] => super::t2::n1854(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 603);
			match s {
				[] => true,
				['h', 'k', 'a', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1722);
					s.is_empty()
				}
				['m', 'o', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				['n', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1724);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1725);
					s.is_empty()
				}
				['u', '\u{1}', s @ ..] => {
					super::push(out, 1726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 604);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1727);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4391);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4392);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 7174);
									match s {
										[] => true,
										['i', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 9233);
											s.is_empty()
										}
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
					super::push(out, 1728);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4393);
							match s {
								[] => true,
								['e', 'r', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 7175);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4394);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4395);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1729);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 4396);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7176);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9234);
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											super::push(out, 9235);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'h', s @ ..] => {
							super::push(out, 4397);
							match s {
								[] => true,
								['s', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7177);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1730);
					match s {
						[] => true,
						['o', 'h', s @ ..] => {
							super::push(out, 4398);
							match s {
								[] => true,
								['l', 'i', 't', 'n', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7178);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1731);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4399);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4400);
							match s {
								[] => true,
								['o', 'n', s @ ..] => {
									super::push(out, 7179);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9236);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4401);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7180);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4402);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1732);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1733);
					match s {
						[] => true,
						['b', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4403);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4404);
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 7182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'u', 's', 'o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 4405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ne.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 605);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1734);
					s.is_empty()
				}
				['e', 'm', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1736);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4407);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1738);
					match s {
						[] => true,
						['e', 'y', 'e', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4409);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 7184);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 606);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4410);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4411);
							match s {
								[] => true,
								['o', 'r', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 4412);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1741);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4413);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7186);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 4414);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 7187);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4415);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4416);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7188);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['i', 's', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4417);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1743);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4418);
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 7189);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4419);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4420);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 4421);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 7191);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4422);
							match s {
								[] => true,
								['a', 'r', s @ ..] => {
									super::push(out, 7192);
									match s {
										[] => true,
										['a', 'h', '\u{1}', s @ ..] => {
											super::push(out, 9237);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4423);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 7193);
									match s {
										[] => true,
										['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9238);
											s.is_empty()
										}
										['r', 'e', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9239);
											s.is_empty()
										}
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
					super::push(out, 1744);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 4424);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7194);
									match s {
										[] => true,
										['t', '\u{1}', s @ ..] => {
											super::push(out, 9240);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4425);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4426);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7195);
									s.is_empty()
								}
								['f', 'f', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 7196);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 7197);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9241);
											match s {
												[] => true,
												['l', 'e', 'r', '\u{1}', s @ ..] => {
													super::push(out, 10396);
													s.is_empty()
												}
												['r', 'f', '\u{1}', s @ ..] => {
													super::push(out, 10397);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4427);
							match s {
								[] => true,
								['l', 'e', 'n', 'n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 4428);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4429);
							match s {
								[] => true,
								['a', 'k', s @ ..] => {
									super::push(out, 7199);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4430);
							match s {
								[] => true,
								['m', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 7200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4431);
							match s {
								[] => true,
								['a', 'z', s @ ..] => {
									super::push(out, 7201);
									s.is_empty()
								}
								['d', 'a', s @ ..] => {
									super::push(out, 7202);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 4432);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4433);
							match s {
								[] => true,
								['a', s @ ..] => super::t2::n7808(s, out),
								['y', s @ ..] => super::t2::n7809(s, out),
								_ => false,
							}
						}
						['z', s @ ..] => super::t2::n4749(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t2::n1878(s, out),
				['u', s @ ..] => super::t2::n1879(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oe.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['\u{11}', 'd', '\u{1}', s @ ..] => {
			super::push(out, 607);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['h', 'k', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['e', 'o', 'h', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4438);
							s.is_empty()
						}
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 4439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'i', 'k', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 608);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'o', 'n', s @ ..] => {
							super::push(out, 4440);
							s.is_empty()
						}
						['k', 'o', 'n', 'i', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pe.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 609);
			match s {
				[] => true,
				['e', 'e', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				['i', 'o', s @ ..] => {
					super::push(out, 1751);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				['p', 'i', s @ ..] => {
					super::push(out, 1753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 610);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1754);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4443);
							s.is_empty()
						}
						['g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4444);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4446);
							match s {
								[] => true,
								['n', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1756);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4447);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4448);
							match s {
								[] => true,
								['e', 't', s @ ..] => {
									super::push(out, 7210);
									match s {
										[] => true,
										['n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 9244);
											s.is_empty()
										}
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
					super::push(out, 1757);
					match s {
						[] => true,
						['u', 'r', 'g', 'z', 't', 'a', 's', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 1758);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -re.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 611);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1759);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7211);
									s.is_empty()
								}
								['k', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7212);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 7213);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1760);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['i', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1762);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 4452);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4453);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7214);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 7215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 612);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 4454);
							match s {
								[] => true,
								['n', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 7216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 's', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4455);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4456);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 7217);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 7218);
									match s {
										[] => true,
										['i', 'x', 'a', 'm', 'o', 'g', 'y', 'z', '\u{1}', s @ ..] => {
											super::push(out, 9245);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'b', 'i', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7219);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4457);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7220);
									match s {
										[] => true,
										['m', 'o', 'r', 'i', 'h', 's', 'u', '\u{1}', s @ ..] => {
											super::push(out, 9246);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4458);
							match s {
								[] => true,
								['l', 'a', s @ ..] => {
									super::push(out, 7221);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 9247);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'o', s @ ..] => {
									super::push(out, 7222);
									match s {
										[] => true,
										['s', 'n', 'a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 9248);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', 'p', 'u', 't', '\u{1}', s @ ..] => {
									super::push(out, 7223);
									s.is_empty()
								}
								['t', 'f', s @ ..] => {
									super::push(out, 7224);
									match s {
										[] => true,
										['o', 's', '\u{1}', s @ ..] => {
											super::push(out, 9249);
											s.is_empty()
										}
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
					super::push(out, 1764);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4459);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7225);
									match s {
										[] => true,
										['a', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9250);
											s.is_empty()
										}
										['u', 's', 't', '\u{1}', s @ ..] => {
											super::push(out, 9251);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 4460);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4461);
							match s {
								[] => true,
								['l', 'g', s @ ..] => {
									super::push(out, 7227);
									match s {
										[] => true,
										['a', 's', 'r', 'e', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9252);
											s.is_empty()
										}
										['i', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9253);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', s @ ..] => {
									super::push(out, 7228);
									match s {
										[] => true,
										['b', 'a', 'r', 'a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9254);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4462);
							s.is_empty()
						}
						['n', 'a', 'h', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4463);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 4464);
							match s {
								[] => true,
								['k', 'e', 'r', 'e', 'w', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1765);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1766);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4465);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7230);
									match s {
										[] => true,
										['s', 'i', 'v', 'v', 'o', 'r', 'p', 'm', 'i', '\u{1}', s @ ..] => {
											super::push(out, 9255);
											s.is_empty()
										}
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
					super::push(out, 1767);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 613);
			match s {
				[] => true,
				['a', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1768);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1769);
					match s {
						[] => true,
						['o', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4466);
							s.is_empty()
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1770);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4468);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 7231);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 7232);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9256);
											s.is_empty()
										}
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
					super::push(out, 1771);
					s.is_empty()
				}
				['r', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 1772);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1773);
					match s {
						[] => true,
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1774);
					match s {
						[] => true,
						['e', 's', 't', '\u{1}', s @ ..] => {
							super::push(out, 4470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1775);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 7233);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9257);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 7234);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9258);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7235);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 9259);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 614);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1776);
					s.is_empty()
				}
				['e', s @ ..] => super::t2::n1911(s, out),
				['i', s @ ..] => {
					super::push(out, 1778);
					match s {
						[] => true,
						['d', 'r', 'a', 'z', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1779);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1780);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4481);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7257);
									match s {
										[] => true,
										['e', 'h', 't', 'e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 9288);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4482);
							s.is_empty()
						}
						['o', s @ ..] => super::t2::n4803(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1781);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				['s', s @ ..] => super::t2::n1917(s, out),
				['u', s @ ..] => super::t3::n1918(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4488);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 7264);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4489);
							s.is_empty()
						}
						['q', '\u{1}', s @ ..] => {
							super::push(out, 4490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 1788);
					match s {
						[] => true,
						['e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4491);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4492);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7265);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'i', s @ ..] => {
					super::push(out, 1791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4494);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7266);
									match s {
										[] => true,
										['s', 'o', '\u{1}', s @ ..] => {
											super::push(out, 9304);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4495);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4496);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 7267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1793);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['s', 'i', 'l', 'a', 't', 'i', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1795);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 4498);
							match s {
								[] => true,
								['n', 'i', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7268);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 4499);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 7269);
									match s {
										[] => true,
										['s', 't', '\u{1}', s @ ..] => {
											super::push(out, 9305);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4500);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7270);
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 9306);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4501);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7271);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 9307);
											match s {
												[] => true,
												['i', 'k', '\u{1}', s @ ..] => {
													super::push(out, 10412);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1796);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 7272);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 9308);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													super::push(out, 10413);
													s.is_empty()
												}
												['y', 'i', '\u{1}', s @ ..] => {
													super::push(out, 10414);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['t', 't', 'e', 'l', 'i', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7273);
									s.is_empty()
								}
								['v', 'a', s @ ..] => {
									super::push(out, 7274);
									match s {
										[] => true,
										['h', s @ ..] => {
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													super::push(out, 10415);
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													super::push(out, 10416);
													s.is_empty()
												}
												_ => false,
											}
										}
										['x', '\u{1}', s @ ..] => {
											super::push(out, 9309);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 4502);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7275);
									match s {
										[] => true,
										['s', s @ ..] => {
											super::push(out, 9310);
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 10417);
													match s {
														[] => true,
														['n', s @ ..] => super::t3::n12109(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1797);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4503);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1798);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1799);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4505);
							match s {
								[] => true,
								['b', 'e', 'l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7276);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1801);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 4506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ue.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 617);
			match s {
				[] => true,
				['y', '\u{1}', s @ ..] => {
					super::push(out, 1804);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['q', s @ ..] => {
					match s {
						[] => true,
						['a', 'c', 'i', s @ ..] => {
							super::push(out, 4507);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4508);
							match s {
								[] => true,
								['s', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 7277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gwe (2 syllables).
pub(crate) fn n642(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 618);
	match s {
		[] => true,
		['n', 'o', 'p', 'm', '\u{1}', s @ ..] => {
			super::push(out, 1806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ye.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 620);
			s.is_empty()
		}
		['\u{12}', 'o', 'y', 'i', 'k', 'u', '\u{1}', s @ ..] => {
			super::push(out, 621);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', 'a', 'p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1807);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1808);
					match s {
						[] => true,
						['h', 's', 'u', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4509);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ze.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['t', 'r', 'a', 'v', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1810);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4510);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 7278);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['i', 'd', 'r', 'a', 'z', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1811);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1812);
					match s {
						[] => true,
						['e', 'h', 'n', 'e', 'd', 'u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 4511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -erthief (3 syllables).
pub(crate) fn n10179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9315);
	match s {
		[] => true,
		['b', 'y', 'c', '\u{1}', s @ ..] => {
			super::push(out, 10418);
			s.is_empty()
		}
		['t', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 10419);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rief (3 syllables).
pub(crate) fn n1966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1829);
	s.is_empty()
}

/// Words ending -ff.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 627);
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					super::push(out, 1830);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1831);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['k', 'o', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7294);
									s.is_empty()
								}
								['l', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7295);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 7296);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9316);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 7297);
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9317);
											s.is_empty()
										}
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
					super::push(out, 1832);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1833);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4524);
							match s {
								[] => true,
								['s', 'e', s @ ..] => {
									match s {
										[] => true,
										['l', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9318);
											s.is_empty()
										}
										['s', 'u', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 9319);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'p', 'y', 'l', 'g', 'g', 'i', 'j', '\u{1}', s @ ..] => {
					super::push(out, 1834);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gf.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	s.is_empty()
}

/// Words ending -hf.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['\u{10}', 'w', '\u{1}', s @ ..] => {
			super::push(out, 628);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -if.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['\u{11}', 'e', 'h', s @ ..] => {
			super::push(out, 629);
			match s {
				[] => true,
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1835);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 630);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kf.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	s.is_empty()
}

/// Words ending -lf.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 631);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 632);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4525);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 4526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4527);
							s.is_empty()
						}
						['h', 's', '\u{1}', s @ ..] => {
							super::push(out, 4528);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1836);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4530);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1837);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 633);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1839);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 4531);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 4532);
							s.is_empty()
						}
						['h', 'w', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4533);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4534);
							s.is_empty()
						}
						['t', 's', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4535);
							s.is_empty()
						}
						['u', 'q', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1840);
					match s {
						[] => true,
						['s', 'n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1841);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1842);
					match s {
						[] => true,
						['w', s @ ..] => {
							super::push(out, 4538);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 634);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1843);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1844);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 4539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', 'i', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4540);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 4541);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mf.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	s.is_empty()
}

/// Words ending -nf.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	s.is_empty()
}

/// Words ending -of.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['\u{11}', 'o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				['o', 'h', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['o', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', 'o', 'h', s @ ..] => {
			super::push(out, 635);
			match s {
				[] => true,
				['e', 's', 'r', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1849);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pf.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 123);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['f', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 637);
			match s {
				[] => true,
				['m', 'y', 't', '\u{1}', s @ ..] => {
					super::push(out, 1851);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qf.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['\u{11}', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rf.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 639);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4544);
							s.is_empty()
						}
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1852);
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', 'a', 'c', 's', 'd', 'a', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 640);
			s.is_empty()
		}
		['\u{13}', 'a', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['c', 's', 'r', 'e', 'v', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sf.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1855);
					match s {
						[] => true,
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4546);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1856);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tf.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	s.is_empty()
}

/// Words ending -uf.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['\u{11}', 'a', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 1857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['z', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wf.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	s.is_empty()
}

/// Words ending -ng (2 syllables).
pub(crate) fn n689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1874);
			match s {
				[] => true,
				['d', 'n', 'a', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4556);
					s.is_empty()
				}
				['l', 's', 'm', 'o', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4557);
					s.is_empty()
				}
				['p', 'p', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4558);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 4559);
					match s {
						[] => true,
						['e', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7304);
							s.is_empty()
						}
						['h', 'g', 'n', 'e', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 7305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 1875);
			match s {
				[] => true,
				['r', 'o', '\u{1}', s @ ..] => {
					super::push(out, 4560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 1876);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4561);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 4562);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 7306);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7307);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9323);
									match s {
										[] => true,
										['i', s @ ..] => {
											match s {
												[] => true,
												['j', '\u{1}', s @ ..] => {
													super::push(out, 10981);
													s.is_empty()
												}
												['x', '\u{1}', s @ ..] => {
													super::push(out, 10982);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 7308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 4563);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 7309);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 7310);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 7311);
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 9324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 4564);
					match s {
						[] => true,
						['e', 'f', s @ ..] => {
							super::push(out, 7312);
							s.is_empty()
						}
						['i', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7313);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 7314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['q', 'i', 'h', 'z', '\u{1}', s @ ..] => {
					super::push(out, 4565);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 4566);
					match s {
						[] => true,
						['p', 's', 'f', s @ ..] => {
							super::push(out, 7315);
							match s {
								[] => true,
								['f', 'o', '\u{1}', s @ ..] => {
									super::push(out, 9325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 9326);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 9327);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'u', 'o', s @ ..] => {
					super::push(out, 4567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1877);
			match s {
				[] => true,
				['e', 'y', 'r', 'o', 'e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 4568);
					s.is_empty()
				}
				['h', 'z', 'n', 'a', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 4569);
					s.is_empty()
				}
				['t', 'o', s @ ..] => {
					super::push(out, 4570);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 7316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1878);
			match s {
				[] => true,
				['e', 's', 'g', 'n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4571);
					s.is_empty()
				}
				['s', 'g', 'n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4572);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ng (3 syllables).
pub(crate) fn n690(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1879);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4573);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 7317);
							match s {
								[] => true,
								['i', 'j', 'm', 'i', 'j', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10420);
											s.is_empty()
										}
										['j', '\u{1}', s @ ..] => {
											super::push(out, 10421);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1880);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 4574);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7318);
							match s {
								[] => true,
								['e', 'w', 'g', 'n', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 9328);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 4575);
					match s {
						[] => true,
						['n', 'a', s @ ..] => {
							super::push(out, 7319);
							match s {
								[] => true,
								['t', 's', 'l', 'e', 'e', 'h', 'w', '\u{1}', s @ ..] => {
									super::push(out, 9329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', 'i', 'l', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4576);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 4577);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 7320);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 9330);
									match s {
										[] => true,
										['a', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 10422);
											s.is_empty()
										}
										['o', 's', 'r', 'e', 'f', '\u{1}', s @ ..] => {
											super::push(out, 10423);
											s.is_empty()
										}
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
					super::push(out, 4578);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 7321);
							match s {
								[] => true,
								['s', 'f', 'f', 'o', 'd', 'n', 'a', 'r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 9331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4579);
					match s {
						[] => true,
						['s', 'i', s @ ..] => {
							super::push(out, 7322);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1881);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['o', 'y', 'n', 'a', 'w', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 4580);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -galog (3 syllables).
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	s.is_empty()
}

/// The word mpg (0 syllables).
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	s.is_empty()
}

/// Words ending -rg.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['\u{13}', 'e', 'b', 'l', s @ ..] => {
			super::push(out, 657);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ug.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	s.is_empty()
}

/// The word xg (0 syllables).
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	s.is_empty()
}

/// Words ending -ah (3 syllables).
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 660);
	match s {
		[] => true,
		['b', 'e', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['s', 's', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 4599);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1899);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4600);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'k', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 4601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 1901);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 1902);
			match s {
				[] => true,
				['c', 'u', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4602);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 4603);
					match s {
						[] => true,
						['a', 'l', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7332);
							s.is_empty()
						}
						['i', 'm', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 7333);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 4604);
					match s {
						[] => true,
						['h', 's', 'a', 'w', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 't', 'e', 'p', '\u{1}', s @ ..] => {
					super::push(out, 4605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'a', 'd', 'e', 'z', 't', '\u{1}', s @ ..] => {
			super::push(out, 1903);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 1904);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 4606);
					match s {
						[] => true,
						['v', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7335);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7336);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 7337);
							match s {
								[] => true,
								['h', 'p', 'e', 't', '\u{1}', s @ ..] => {
									super::push(out, 9333);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4607);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 4608);
					s.is_empty()
				}
				['u', 'r', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 4609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7338);
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 9334);
									s.is_empty()
								}
								['k', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 9335);
									s.is_empty()
								}
								_ => false,
							}
						}
						['q', 'q', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 7339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 4610);
					match s {
						[] => true,
						['g', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 7340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'k', s @ ..] => {
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 7341);
							s.is_empty()
						}
						['k', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 7342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1905);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4611);
					match s {
						[] => true,
						['t', 'n', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 4612);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 4613);
					match s {
						[] => true,
						['b', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 7344);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 7345);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10424);
											s.is_empty()
										}
										['h', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 10983);
													s.is_empty()
												}
												['c', '\u{1}', s @ ..] => {
													super::push(out, 10984);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', 'z', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 7347);
							s.is_empty()
						}
						['f', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 7348);
							s.is_empty()
						}
						['h', 'p', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 7349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 4614);
					match s {
						[] => true,
						['g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 7350);
							s.is_empty()
						}
						['n', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7351);
							s.is_empty()
						}
						['s', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 4615);
					s.is_empty()
				}
				['z', 't', 'o', 'z', 't', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 4616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 1908);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1909);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1910);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'a', 'g', 'f', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7353);
							s.is_empty()
						}
						['r', 'i', 'd', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 7354);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 4617);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7355);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 9336);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ch.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['r', 's', '\u{1}', s @ ..] => {
					super::push(out, 1912);
					s.is_empty()
				}
				['t', 'w', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1914);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4618);
							match s {
								[] => true,
								['w', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7356);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4619);
							s.is_empty()
						}
						['l', 'v', '\u{1}', s @ ..] => {
							super::push(out, 4620);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4621);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4622);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 7357);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 7358);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 4623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 1915);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1916);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4624);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4625);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 7359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4626);
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							super::push(out, 4627);
							s.is_empty()
						}
						['y', '\u{1}', s @ ..] => {
							super::push(out, 4628);
							s.is_empty()
						}
						['z', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4630);
							s.is_empty()
						}
						['a', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 4631);
							s.is_empty()
						}
						['l', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 4632);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4633);
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 4634);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4635);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 4636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1918);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1919);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 4637);
							match s {
								[] => true,
								['r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7360);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 7361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4638);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7362);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7363);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 4639);
							s.is_empty()
						}
						['y', 's', '\u{1}', s @ ..] => {
							super::push(out, 4640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 4641);
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4642);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4643);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4644);
							s.is_empty()
						}
						['r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1921);
					match s {
						[] => true,
						['e', 'h', 't', '\u{1}', s @ ..] => {
							super::push(out, 4646);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 4647);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 7364);
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7365);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4648);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1922);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4649);
							s.is_empty()
						}
						['r', 'u', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4650);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7366);
									s.is_empty()
								}
								['n', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7367);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1923);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4651);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 7368);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4652);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4654);
							s.is_empty()
						}
						['s', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => super::t3::n700(s, out),
		['\u{13}', s @ ..] => super::t3::n701(s, out),
		_ => false,
	}
}

/// Words ending -eh.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['\u{11}', 'u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 668);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 669);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 1945);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 4730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 1946);
					s.is_empty()
				}
				['w', 'q', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1947);
					s.is_empty()
				}
				['z', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1948);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1949);
					match s {
						[] => true,
						['a', 'd', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4731);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1950);
					match s {
						[] => true,
						['a', 'q', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gh (1 syllables).
pub(crate) fn n708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 672);
	match s {
		[] => true,
		['i', 'y', 't', '\u{1}', s @ ..] => {
			super::push(out, 1951);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gh (3 syllables).
pub(crate) fn n710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	s.is_empty()
}

/// Words ending -hh.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['\u{10}', 's', '\u{1}', s @ ..] => {
			super::push(out, 675);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ih.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 1953);
					s.is_empty()
				}
				['r', 'd', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1954);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kh.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 677);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['u', 'd', 'd', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 1955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'a', 'p', 'a', 'p', 'a', 'r', 'a', 'k', '\u{1}', s @ ..] => {
			super::push(out, 679);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lh.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 680);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mh.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 681);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nh.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	s.is_empty()
}

/// Words ending -oh.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 682);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['m', 'g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1956);
					s.is_empty()
				}
				['t', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1957);
					s.is_empty()
				}
				['z', 't', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ph.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['k', '\u{1}', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['l', 's', '\u{1}', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 1961);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4733);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 4734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 684);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 685);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4735);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 4736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1962);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 4737);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 7441);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 9386);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rh.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	s.is_empty()
}

/// Words ending -sh.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 1963);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1964);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 1965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{11}', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['e', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1968);
					match s {
						[] => true,
						['e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1971);
					match s {
						[] => true,
						['f', 'd', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4739);
							s.is_empty()
						}
						['l', 't', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4740);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4741);
							match s {
								[] => true,
								['a', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7442);
									s.is_empty()
								}
								['m', 'a', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7443);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 4742);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 7444);
									match s {
										[] => true,
										['i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4743);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1972);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4744);
							match s {
								[] => true,
								['p', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7445);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4745);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4746);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7446);
									s.is_empty()
								}
								['o', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n2122(s, out),
				['o', 'r', s @ ..] => {
					super::push(out, 1974);
					match s {
						[] => true,
						['f', 'e', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1975);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4757);
							s.is_empty()
						}
						['t', 'h', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 4758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t3::n728(s, out),
		_ => false,
	}
}

/// Words ending -th.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1981);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1982);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1983);
					match s {
						[] => true,
						['o', 't', '\u{1}', s @ ..] => {
							super::push(out, 4767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1984);
					match s {
						[] => true,
						['a', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 4768);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1985);
					match s {
						[] => true,
						['m', 'a', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1986);
					match s {
						[] => true,
						['d', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4770);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4771);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4772);
							s.is_empty()
						}
						['n', 'a', s @ ..] => {
							super::push(out, 4773);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 4774);
							match s {
								[] => true,
								['t', 'i', 's', 't', '\u{1}', s @ ..] => {
									super::push(out, 7466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1987);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4775);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4776);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['e', 'y', 'e', '\u{1}', s @ ..] => {
											super::push(out, 9436);
											s.is_empty()
										}
										['g', 'n', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 9437);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 9438);
											match s {
												[] => true,
												['c', s @ ..] => {
													match s {
														[] => true,
														['e', 'n', '\u{1}', s @ ..] => super::t3::n12128(s, out),
														['u', 'b', '\u{1}', s @ ..] => super::t3::n12129(s, out),
														_ => false,
													}
												}
												['e', 'e', 'h', 'c', '\u{1}', s @ ..] => {
													super::push(out, 10491);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 'd', 'n', 'u', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 9439);
											s.is_empty()
										}
										['w', 'a', s @ ..] => {
											match s {
												[] => true,
												['j', '\u{1}', s @ ..] => {
													super::push(out, 10492);
													s.is_empty()
												}
												['s', '\u{1}', s @ ..] => {
													super::push(out, 10493);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1988);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1989);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1990);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4777);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4778);
							match s {
								[] => true,
								['t', s @ ..] => {
									super::push(out, 7467);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['l', 'g', 'g', 'a', 'n', 's', '\u{1}', s @ ..] => {
													super::push(out, 10494);
													s.is_empty()
												}
												['r', 'o', 'f', '\u{1}', s @ ..] => {
													super::push(out, 10495);
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', 'd', 'u', 'e', 's', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9440);
											s.is_empty()
										}
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
					super::push(out, 1991);
					match s {
						[] => true,
						['o', 'w', 's', s @ ..] => {
							super::push(out, 4779);
							match s {
								[] => true,
								['e', 't', 'u', 'n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7468);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uh.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 694);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['h', 'c', 's', 'r', 'e', 't', 't', 'e', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vh.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	s.is_empty()
}

/// Words ending -wh.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zh.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 1993);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kai (3 syllables).
pub(crate) fn n2169(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 's', 'i', '\u{1}', s @ ..] => {
			super::push(out, 4805);
			s.is_empty()
		}
		['i', 'm', 'o', 'n', '\u{1}', s @ ..] => {
			super::push(out, 4806);
			s.is_empty()
		}
		['n', 'e', 'n', 'o', 'b', '\u{1}', s @ ..] => {
			super::push(out, 4807);
			s.is_empty()
		}
		['o', 'y', 'e', 'g', 'n', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 4808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pai (3 syllables).
pub(crate) fn n2172(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', 'k', 'a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 4809);
			s.is_empty()
		}
		['o', 'k', 'e', 'i', 's', '\u{1}', s @ ..] => {
			super::push(out, 4810);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rai (3 syllables).
pub(crate) fn n2173(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 4811);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 7470);
					s.is_empty()
				}
				['s', 'n', 'a', 'v', 'a', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 7471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'm', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 4812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tai (3 syllables).
pub(crate) fn n2174(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'g', 'i', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 4813);
			s.is_empty()
		}
		['i', 'j', 'n', 'i', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 4814);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word tivaivai (3 syllables).
pub(crate) fn n2175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2016);
	s.is_empty()
}

/// Words ending -wai (3 syllables).
pub(crate) fn n2176(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 't', 'n', 'e', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4815);
			s.is_empty()
		}
		['o', 'r', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 4816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bi.
pub(crate) fn n166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 165);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 700);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2017);
					s.is_empty()
				}
				['m', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2018);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 2019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 701);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2020);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 4817);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7472);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 9443);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4818);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2021);
					match s {
						[] => true,
						['n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4819);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2022);
					match s {
						[] => true,
						['u', 'd', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'n', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 2023);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2024);
					match s {
						[] => true,
						['k', 'o', 'r', 'u', 'k', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4821);
							s.is_empty()
						}
						['s', 'u', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7473);
									s.is_empty()
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7474);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ci.
pub(crate) fn n167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 166);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 702);
			s.is_empty()
		}
		['\u{12}', 'i', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 2025);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2026);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -di.
pub(crate) fn n168(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 167);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 704);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2027);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2028);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4822);
							s.is_empty()
						}
						['j', '\u{1}', s @ ..] => {
							super::push(out, 4823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2029);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', 'g', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4824);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 4825);
							match s {
								[] => true,
								['o', 'j', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				['u', 'n', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2032);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4826);
							s.is_empty()
						}
						['g', 'a', 'l', 'a', 'g', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4827);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 4828);
							s.is_empty()
						}
						['l', 'u', 't', '\u{1}', s @ ..] => {
							super::push(out, 4829);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4830);
							match s {
								[] => true,
								['i', 'x', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 4831);
							match s {
								[] => true,
								['l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7477);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2033);
					match s {
						[] => true,
						['e', 'r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4832);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2034);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2035);
					match s {
						[] => true,
						['a', 'h', 'p', 'e', 'k', 'h', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4833);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2036);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ei.
pub(crate) fn n169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 168);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 707);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['b', 'n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2037);
					s.is_empty()
				}
				['h', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2038);
					s.is_empty()
				}
				['k', 'n', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2039);
					s.is_empty()
				}
				['l', 'o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2040);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2041);
					s.is_empty()
				}
				['r', 'u', 'y', '\u{1}', s @ ..] => {
					super::push(out, 2042);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2043);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 7478);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 7479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['e', 's', '\u{1}', s @ ..] => {
									super::push(out, 7480);
									s.is_empty()
								}
								['o', 'y', '\u{1}', s @ ..] => {
									super::push(out, 7481);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'a', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 2044);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['k', 'n', 'e', 'z', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2045);
					s.is_empty()
				}
				['m', 'i', 'r', 'n', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 2046);
					s.is_empty()
				}
				['p', 'p', 'a', 'g', 'i', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2047);
					s.is_empty()
				}
				['t', 'i', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2048);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fi.
pub(crate) fn n170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 169);
	s.is_empty()
}

/// Words ending -gi.
pub(crate) fn n171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 170);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2049);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4834);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2050);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2051);
					match s {
						[] => true,
						['u', 'l', '\u{1}', s @ ..] => {
							super::push(out, 4835);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4836);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4837);
							s.is_empty()
						}
						['s', 'a', 'k', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', 's', 'o', 'y', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2052);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4839);
							s.is_empty()
						}
						['i', 'l', 'i', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 4840);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4841);
							match s {
								[] => true,
								['b', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7482);
									s.is_empty()
								}
								['e', 'r', 'u', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7483);
									s.is_empty()
								}
								['m', 'u', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7484);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['n', 'i', 'b', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4842);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							super::push(out, 4843);
							match s {
								[] => true,
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7485);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 7486);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', 't', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hi.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 171);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 710);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4845);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 4846);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 7487);
									s.is_empty()
								}
								['o', 'n', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2053);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2054);
					s.is_empty()
				}
				['o', 'r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2055);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2056);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4847);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 4848);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 7489);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 9444);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7490);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 7491);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4849);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7492);
									s.is_empty()
								}
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 7493);
									s.is_empty()
								}
								['n', '\u{1}', s @ ..] => {
									super::push(out, 7494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2058);
					match s {
						[] => true,
						['k', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4850);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', s @ ..] => {
					match s {
						[] => true,
						['h', 'j', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4851);
							s.is_empty()
						}
						['j', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2059);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 10496);
													s.is_empty()
												}
												['n', '\u{1}', s @ ..] => {
													super::push(out, 10497);
													s.is_empty()
												}
												['o', '\u{1}', s @ ..] => {
													super::push(out, 10498);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 9445);
											s.is_empty()
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 7495);
									s.is_empty()
								}
								['m', 'a', s @ ..] => {
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 9446);
											s.is_empty()
										}
										['k', 'o', 'j', '\u{1}', s @ ..] => {
											super::push(out, 9447);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'i', 'a', 'm', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'n', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 4853);
							s.is_empty()
						}
						['o', 'm', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7497);
									s.is_empty()
								}
								['i', 'h', 'c', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 7498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4854);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['i', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7499);
									s.is_empty()
								}
								['k', 'o', 'n', 'o', 'j', '\u{1}', s @ ..] => {
									super::push(out, 7500);
									s.is_empty()
								}
								['o', 'n', 'u', 'k', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7501);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2060);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 2061);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2062);
					match s {
						[] => true,
						['p', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2063);
					s.is_empty()
				}
				['p', 'a', 'n', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 4856);
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							super::push(out, 4857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t3::n2231(s, out),
				['t', s @ ..] => super::t3::n2232(s, out),
				['u', 'r', 'i', 'h', 'u', 'r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2065);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ii.
pub(crate) fn n173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['\u{12}', 'r', 'o', 't', '\u{1}', s @ ..] => {
			super::push(out, 713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ji.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 173);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2066);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 4863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2067);
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'a', 'd', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2068);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4865);
							s.is_empty()
						}
						['u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 4867);
							s.is_empty()
						}
						['y', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4868);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2069);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2070);
					s.is_empty()
				}
				['e', 't', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2071);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4869);
							s.is_empty()
						}
						['e', 't', 'n', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2072);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4871);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 7526);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', 'g', 'e', 't', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2073);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4873);
							match s {
								[] => true,
								['i', 'm', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7527);
									s.is_empty()
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7528);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 4874);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ki.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 714);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 4875);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 4876);
							s.is_empty()
						}
						['y', 'i', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2074);
					s.is_empty()
				}
				['n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2075);
					s.is_empty()
				}
				['o', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2076);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2077);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2078);
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'c', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 2079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 716);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2080);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 4879);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7529);
									match s {
										[] => true,
										['m', 'l', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9458);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', 'u', 's', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4880);
							s.is_empty()
						}
						['k', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4881);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 4882);
							match s {
								[] => true,
								['e', 'k', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7530);
									s.is_empty()
								}
								['l', 'e', 'k', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7531);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4883);
							match s {
								[] => true,
								['a', 'r', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7532);
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									super::push(out, 7533);
									s.is_empty()
								}
								['i', 'h', 'c', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7534);
									s.is_empty()
								}
								['o', 's', 'o', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7535);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4884);
							s.is_empty()
						}
						['r', 'h', 't', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 4885);
							s.is_empty()
						}
						['t', 'a', 'r', 'i', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 4886);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 7536);
									match s {
										[] => true,
										['r', 'o', 'd', '\u{1}', s @ ..] => {
											super::push(out, 9459);
											s.is_empty()
										}
										['w', 'a', 'g', 'a', 'm', 'i', '\u{1}', s @ ..] => {
											super::push(out, 9460);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'm', 'o', 'n', 'o', 'k', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7537);
									s.is_empty()
								}
								['o', 'k', 'a', 't', '\u{1}', s @ ..] => {
									super::push(out, 7538);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', 'n', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4887);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7539);
									s.is_empty()
								}
								['o', 'y', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7540);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', 'y', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2081);
					match s {
						[] => true,
						['c', 's', 'o', 'r', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4889);
							s.is_empty()
						}
						['s', 'o', 'r', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4890);
							s.is_empty()
						}
						['z', 'o', 'r', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', 's', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7541);
									s.is_empty()
								}
								['u', 's', 't', 'a', 'p', 'n', 'a', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7542);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 4892);
							match s {
								[] => true,
								['o', 't', 'a', 'm', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7543);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'a', 'b', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4893);
							s.is_empty()
						}
						['r', 'o', 'y', '\u{1}', s @ ..] => {
							super::push(out, 4894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'k', 'e', 'i', 'p', 'a', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2082);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', 's', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4895);
							s.is_empty()
						}
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 7544);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 7545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'n', 'a', 't', '\u{1}', s @ ..] => {
							super::push(out, 4896);
							s.is_empty()
						}
						['z', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2083);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 't', '\u{1}', s @ ..] => {
									super::push(out, 7546);
									s.is_empty()
								}
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 7547);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', 't', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4898);
							s.is_empty()
						}
						['y', 'i', 'h', 'c', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4899);
							s.is_empty()
						}
						['z', 'a', 'k', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -li.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 717);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2084);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2085);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 4901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2086);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 4902);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 7548);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2087);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 4903);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 719);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2088);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 4904);
							match s {
								[] => true,
								['o', 's', '\u{1}', s @ ..] => {
									super::push(out, 7549);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2089);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 4905);
							match s {
								[] => true,
								['e', 't', 'f', 'i', 's', 't', '\u{1}', s @ ..] => {
									super::push(out, 7550);
									s.is_empty()
								}
								['s', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7551);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2090);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4906);
							match s {
								[] => true,
								['w', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 7552);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2091);
					match s {
						[] => true,
						['e', 't', 'a', 'c', 's', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 4907);
							s.is_empty()
						}
						['i', 's', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 4908);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 4909);
							match s {
								[] => true,
								['l', 'i', 'm', 'i', 'u', 'q', 'a', 'l', 't', '\u{1}', s @ ..] => {
									super::push(out, 7553);
									s.is_empty()
								}
								['p', 'l', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'k', 'i', 'r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 2092);
					s.is_empty()
				}
				['t', 'n', 'i', 'u', 'c', 'z', 't', 'i', 'o', 'l', 'o', 'x', '\u{1}', s @ ..] => {
					super::push(out, 2093);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 2094);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mi.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 720);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4910);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 4911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2095);
					s.is_empty()
				}
				['m', 'u', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2096);
					s.is_empty()
				}
				['o', 'y', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2097);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 2098);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 721);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['k', 'i', 'h', 's', '\u{1}', s @ ..] => {
											super::push(out, 9461);
											s.is_empty()
										}
										['n', 'i', 'h', 's', '\u{1}', s @ ..] => {
											super::push(out, 9462);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 7555);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'o', 'n', 'o', 'g', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4912);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 4913);
							s.is_empty()
						}
						['n', 'u', 's', 't', '\u{1}', s @ ..] => {
							super::push(out, 4914);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 4915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', s @ ..] => {
					super::push(out, 2099);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['k', 'e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7556);
									s.is_empty()
								}
								['n', 'o', 'm', 'e', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7557);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4916);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 4917);
							match s {
								[] => true,
								['e', 'm', 'i', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7558);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 7559);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'o', '\u{1}', s @ ..] => {
							super::push(out, 4918);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2100);
					match s {
						[] => true,
						['g', 'a', 'w', 'i', '\u{1}', s @ ..] => {
							super::push(out, 4919);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4920);
							match s {
								[] => true,
								['i', 'm', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7560);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4921);
							s.is_empty()
						}
						['r', 'u', 'g', 'i', s @ ..] => {
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 7561);
									s.is_empty()
								}
								['m', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7562);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ni.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2101);
					s.is_empty()
				}
				['h', 'g', 'u', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 2102);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2103);
					s.is_empty()
				}
				['o', 'g', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2104);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2105);
					match s {
						[] => true,
						['d', 'e', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4922);
							s.is_empty()
						}
						['g', 'e', 'k', 'i', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4923);
							s.is_empty()
						}
						['m', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4924);
							s.is_empty()
						}
						['p', 'o', s @ ..] => {
							super::push(out, 4925);
							s.is_empty()
						}
						['r', 'o', s @ ..] => {
							super::push(out, 4926);
							match s {
								[] => true,
								['a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 7563);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['e', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7564);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 7565);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2106);
					match s {
						[] => true,
						['k', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4927);
							s.is_empty()
						}
						['m', 'm', 'a', 'r', 'g', 'e', 'g', 'o', 'p', 'y', '\u{1}', s @ ..] => {
							super::push(out, 4928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2107);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4929);
							match s {
								[] => true,
								['n', 'o', s @ ..] => {
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											super::push(out, 9463);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 9464);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', 'r', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4930);
							s.is_empty()
						}
						['k', 's', 'o', 'b', 'm', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4931);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 7566);
									s.is_empty()
								}
								['o', 'c', 'c', 'o', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7567);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4932);
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 4933);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 7569);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 9465);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 10500);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'u', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2108);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2109);
					match s {
						[] => true,
						['i', 'h', 's', 'u', '\u{1}', s @ ..] => {
							super::push(out, 4934);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4935);
							match s {
								[] => true,
								['a', 'g', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 7570);
									s.is_empty()
								}
								['r', 'o', 't', '\u{1}', s @ ..] => {
									super::push(out, 7571);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['k', 'a', 'h', 's', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4936);
							s.is_empty()
						}
						['p', 'e', 'r', 'a', 'h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 4937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oi.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['a', 'y', '\u{1}', s @ ..] => {
					super::push(out, 2110);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 2111);
					s.is_empty()
				}
				['h', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2112);
					s.is_empty()
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 2113);
					s.is_empty()
				}
				['k', '\u{1}', s @ ..] => {
					super::push(out, 2114);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 2115);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 2116);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['k', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2117);
					s.is_empty()
				}
				['n', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', 'h', 'i', 'o', 'h', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 724);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pi.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 2119);
					s.is_empty()
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 2120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 725);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', 'k', 'o', '\u{1}', s @ ..] => {
					super::push(out, 2121);
					s.is_empty()
				}
				['o', 'l', 't', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2122);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -qi.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['\u{12}', 'a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 726);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ri.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 727);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 728);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4938);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 4939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', 'l', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2123);
					s.is_empty()
				}
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 2124);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2125);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4940);
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 7573);
									s.is_empty()
								}
								['m', '\u{1}', s @ ..] => {
									super::push(out, 7574);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 4941);
							s.is_empty()
						}
						['y', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 4942);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 4943);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2126);
					s.is_empty()
				}
				['t', 'e', 't', '\u{1}', s @ ..] => {
					super::push(out, 2127);
					s.is_empty()
				}
				['u', 'o', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2128);
					s.is_empty()
				}
				['z', 't', 'o', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2129);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2130);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['e', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7575);
									s.is_empty()
								}
								['u', 's', 'i', 'u', 'g', 'u', '\u{1}', s @ ..] => {
									super::push(out, 7576);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 4944);
							s.is_empty()
						}
						['f', 'a', 't', 's', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 4945);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', 'a', 't', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7577);
									s.is_empty()
								}
								['o', 't', 'o', 's', 'o', '\u{1}', s @ ..] => {
									super::push(out, 7578);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 7579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 't', s @ ..] => {
							super::push(out, 4946);
							match s {
								[] => true,
								['u', '\u{1}', s @ ..] => {
									super::push(out, 7580);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4947);
							s.is_empty()
						}
						['k', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 4948);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7581);
									s.is_empty()
								}
								['e', 't', '\u{1}', s @ ..] => {
									super::push(out, 7582);
									s.is_empty()
								}
								['m', 'i', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7583);
									s.is_empty()
								}
								['o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 7584);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 4949);
							match s {
								[] => true,
								['a', 't', 'u', 'f', '\u{1}', s @ ..] => {
									super::push(out, 7585);
									s.is_empty()
								}
								['o', 'b', 'r', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7586);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 4950);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 4951);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 7587);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 7588);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 9466);
											match s {
												[] => true,
												['t', s @ ..] => {
													super::push(out, 10501);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', 'g', 'g', 'n', 'e', 'a', 'w', 'k', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2131);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 4953);
							s.is_empty()
						}
						['g', 'i', 'a', 'm', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4954);
							s.is_empty()
						}
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7589);
									s.is_empty()
								}
								['h', 'c', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 7590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', 'd', 'n', 'i', 'r', 'r', 'a', 'g', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4955);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 4956);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7591);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 4957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['g', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 4958);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 4959);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 7592);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 9467);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'o', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7593);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 4960);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => super::t3::n2314(s, out),
				['u', s @ ..] => super::t3::n2315(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -si.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 730);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 4977);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 4978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2134);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2135);
					s.is_empty()
				}
				['t', 'a', 'a', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 732);
			match s {
				[] => true,
				['a', 'w', 'u', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2137);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', 'd', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4979);
							s.is_empty()
						}
						['n', 'n', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 4980);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 's', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2138);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2139);
					match s {
						[] => true,
						['f', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 4981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', 'b', 'l', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2140);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ti.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 733);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 734);
			match s {
				[] => true,
				['e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 2141);
					s.is_empty()
				}
				['i', 'w', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2142);
					s.is_empty()
				}
				['l', 'u', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2143);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2144);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 4982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2145);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4983);
							s.is_empty()
						}
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 4984);
							s.is_empty()
						}
						['u', 'p', '\u{1}', s @ ..] => {
							super::push(out, 4985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['h', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4986);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 4987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 735);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4988);
							s.is_empty()
						}
						['n', 'o', 'm', 'm', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 4989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'a', 'h', 's', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2147);
					s.is_empty()
				}
				['i', 'f', 'f', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 2148);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2149);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 4990);
							s.is_empty()
						}
						['e', 'r', 'g', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 4991);
							s.is_empty()
						}
						['i', 'd', 'n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 4992);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', 's', 'i', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7602);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'z', 'n', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9471);
											s.is_empty()
										}
										['e', 'z', 'n', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9472);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'k', 'k', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 4993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2150);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ui.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 2151);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 2152);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 736);
			match s {
				[] => true,
				['l', 'l', 'i', '\u{1}', s @ ..] => {
					super::push(out, 2153);
					s.is_empty()
				}
				['m', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2154);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2155);
					s.is_empty()
				}
				['s', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 737);
			match s {
				[] => true,
				['h', 'a', 'r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2157);
					s.is_empty()
				}
				['k', 'u', 's', 'a', 't', 'a', 'm', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2158);
					s.is_empty()
				}
				['n', 'a', 'p', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2159);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 2160);
					s.is_empty()
				}
				['s', 'n', 'a', 's', 'e', 'r', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vi.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	s.is_empty()
}

/// Words ending -wi.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 738);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 4994);
							s.is_empty()
						}
						['k', '\u{1}', s @ ..] => {
							super::push(out, 4995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 2162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2163);
					s.is_empty()
				}
				['i', 'k', 'i', 'w', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2164);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xi.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['\u{13}', 'o', 'l', 'i', 'b', '\u{1}', s @ ..] => {
			super::push(out, 740);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yi.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', 'o', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2165);
					s.is_empty()
				}
				['n', 'o', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2166);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zi.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['\u{11}', 's', '\u{1}', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', 'o', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2167);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 2168);
					match s {
						[] => true,
						['a', 'h', s @ ..] => {
							super::push(out, 4996);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 7603);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 4997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'a', s @ ..] => {
					super::push(out, 2169);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 4998);
							s.is_empty()
						}
						['i', 'j', '\u{1}', s @ ..] => {
							super::push(out, 4999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2170);
					match s {
						[] => true,
						['g', 'e', 'b', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5000);
							s.is_empty()
						}
						['n', 'e', 'k', 'h', 's', 'a', '\u{1}', s @ ..] => {
							super::push(out, 5001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2171);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5002);
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['m', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9473);
											s.is_empty()
										}
										['p', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9474);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 't', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5003);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lek (2 syllables).
pub(crate) fn n2396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2207);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5024);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 7610);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nek (2 syllables).
pub(crate) fn n2397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2208);
	s.is_empty()
}

/// Words ending -oek (2 syllables).
pub(crate) fn n2398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2209);
	match s {
		[] => true,
		['d', 'p', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rek (2 syllables).
pub(crate) fn n2399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2210);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5026);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 7611);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zek (2 syllables).
pub(crate) fn n2400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2211);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 5027);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 7612);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							super::push(out, 9480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'o', 'r', 'k', '\u{1}', s @ ..] => {
			super::push(out, 5028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ek (3 syllables).
pub(crate) fn n809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 752);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 2212);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 5029);
					match s {
						[] => true,
						['e', 'n', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7613);
							s.is_empty()
						}
						['n', 'e', 'n', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 7614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2213);
			match s {
				[] => true,
				['u', 'b', 'e', 'h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5030);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ik.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 753);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 2214);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					super::push(out, 2215);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5031);
							match s {
								[] => true,
								['i', 'l', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2216);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5032);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7616);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 9481);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5033);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 7617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 5034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2217);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5035);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 7618);
									s.is_empty()
								}
								['q', '\u{1}', s @ ..] => {
									super::push(out, 7619);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2218);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 755);
			match s {
				[] => true,
				['a', 'n', 'a', 'h', 't', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2219);
					s.is_empty()
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['h', 's', 'm', 'o', 'r', 'g', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5036);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5037);
							match s {
								[] => true,
								['a', 'd', 'z', 'i', 'm', 'a', s @ ..] => {
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											super::push(out, 9482);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 9483);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 5038);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2220);
					match s {
						[] => true,
						['a', 'h', 't', 'a', 'd', '\u{1}', s @ ..] => {
							super::push(out, 5040);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 5041);
							s.is_empty()
						}
						['e', 'r', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5042);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5043);
							match s {
								[] => true,
								['e', 'l', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7620);
									s.is_empty()
								}
								['i', 'l', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7621);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'i', 'l', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5044);
							s.is_empty()
						}
						['z', 't', 'u', 'b', 'b', 'i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5045);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2221);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5046);
							match s {
								[] => true,
								['m', 'd', 'n', 'e', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7622);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2222);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lk.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['\u{11}', 'e', '\u{1}', s @ ..] => {
			super::push(out, 756);
			s.is_empty()
		}
		['\u{13}', 'o', s @ ..] => {
			super::push(out, 757);
			match s {
				[] => true,
				['f', 'd', 'r', 'a', 'z', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 2223);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nk.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 2224);
					match s {
						[] => true,
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 5047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2225);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', 'e', s @ ..] => {
			super::push(out, 758);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ok.
pub(crate) fn n208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 759);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2226);
					match s {
						[] => true,
						['g', 'n', 'i', 'r', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 5048);
							s.is_empty()
						}
						['u', 'o', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2227);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 2228);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 761);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2230);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 5050);
							match s {
								[] => true,
								['o', 'r', 'i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7623);
									s.is_empty()
								}
								['u', 'p', 'm', 'a', 'p', '\u{1}', s @ ..] => {
									super::push(out, 7624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'o', 'r', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2231);
					match s {
						[] => true,
						['e', 'l', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2232);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5053);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 5054);
							match s {
								[] => true,
								['n', 'e', 'g', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 7625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rk.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 208);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 762);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 2233);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2234);
					match s {
						[] => true,
						['m', 'h', s @ ..] => {
							super::push(out, 5055);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sk.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	s.is_empty()
}

/// Words ending -uk.
pub(crate) fn n211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2235);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2236);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 764);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 2237);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yk.
pub(crate) fn n212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['\u{12}', 'n', 'r', s @ ..] => {
			super::push(out, 765);
			match s {
				[] => true,
				['y', 's', '\u{1}', s @ ..] => {
					super::push(out, 2238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 766);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 2239);
					match s {
						[] => true,
						['e', 'r', 'a', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5057);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7626);
									s.is_empty()
								}
								['y', 'l', 'a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7627);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -el (3 syllables).
pub(crate) fn n831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 772);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2270);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 2271);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 5079);
					match s {
						[] => true,
						['a', 't', 's', 'z', 't', 'u', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 7635);
							s.is_empty()
						}
						['o', 't', 'r', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 7636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', 'n', 'e', 't', 's', 'e', 'e', 'l', 'k', '\u{1}', s @ ..] => {
			super::push(out, 2272);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 2273);
			match s {
				[] => true,
				['a', 'n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5080);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 2274);
			match s {
				[] => true,
				['u', 'e', 'r', 'k', 'i', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 5081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2275);
			match s {
				[] => true,
				['l', 'a', 'p', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5082);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2276);
			match s {
				[] => true,
				['e', 'k', s @ ..] => {
					super::push(out, 5083);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9486);
									s.is_empty()
								}
								['i', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9487);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -il.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 216);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 773);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 774);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2277);
					match s {
						[] => true,
						['t', 'g', s @ ..] => {
							super::push(out, 5084);
							match s {
								[] => true,
								['n', 'i', 'r', '\u{1}', s @ ..] => {
									super::push(out, 7637);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2278);
					match s {
						[] => true,
						['n', 'a', 'j', '\u{1}', s @ ..] => {
							super::push(out, 5085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2279);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 5086);
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7638);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2280);
					match s {
						[] => true,
						['s', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5087);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2281);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2282);
					match s {
						[] => true,
						['a', 'u', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2283);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5089);
							match s {
								[] => true,
								['s', 'l', 'e', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7639);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 2284);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5090);
							match s {
								[] => true,
								['h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2286);
					match s {
						[] => true,
						['t', 'e', 'l', 't', s @ ..] => {
							super::push(out, 5091);
							match s {
								[] => true,
								['s', 'i', 'r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5092);
							match s {
								[] => true,
								['c', 'a', '\u{1}', s @ ..] => {
									super::push(out, 7642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2289);
					match s {
						[] => true,
						['n', 'e', 'i', 'k', 'l', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 5093);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'u', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 2291);
					match s {
						[] => true,
						['e', 'k', 'h', 's', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ll.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['a', 'i', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				['i', 'r', s @ ..] => {
					super::push(out, 2293);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 5095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 777);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 778);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ol.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 779);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['b', 'e', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 2297);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5097);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 7643);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2298);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2299);
					match s {
						[] => true,
						['e', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5099);
							s.is_empty()
						}
						['i', 'r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'n', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2301);
					match s {
						[] => true,
						['e', 'i', 'l', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rl.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['\u{10}', 'r', s @ ..] => {
			super::push(out, 782);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tl.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 783);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 784);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['a', 'u', 'h', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2302);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['a', 'c', s @ ..] => {
					super::push(out, 2303);
					match s {
						[] => true,
						['e', 't', 'h', 'c', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ul.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2307);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2308);
					match s {
						[] => true,
						['b', 'u', s @ ..] => {
							match s {
								[] => true,
								['r', 'h', 's', '\u{1}', s @ ..] => {
									super::push(out, 7644);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 7645);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5103);
							s.is_empty()
						}
						['g', 'a', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 5104);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5105);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 7646);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5106);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5107);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5108);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5109);
							match s {
								[] => true,
								['r', 'u', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 5110);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2309);
					match s {
						[] => true,
						['l', 'i', 'g', '\u{1}', s @ ..] => {
							super::push(out, 5111);
							s.is_empty()
						}
						['z', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2310);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2315);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 2316);
					match s {
						[] => true,
						['d', 'o', 'o', 'h', 'r', 'o', 'b', 'h', 'g', 'i', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5113);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5114);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 7648);
									s.is_empty()
								}
								['n', 'a', 'l', 'p', s @ ..] => {
									super::push(out, 7649);
									match s {
										[] => true,
										['r', 'i', 'a', '\u{1}', s @ ..] => {
											super::push(out, 9488);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', 'a', 'b', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5115);
							s.is_empty()
						}
						['m', 'o', 'o', 'r', 'd', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5116);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5117);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5118);
							match s {
								[] => true,
								['e', 'k', s @ ..] => {
									match s {
										[] => true,
										['c', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9489);
											s.is_empty()
										}
										['n', 'a', 'l', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9490);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'l', 'l', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 7650);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 5119);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 7651);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wl.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 790);
			match s {
				[] => true,
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 5120);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 5121);
							match s {
								[] => true,
								['b', 'u', 'r', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 7652);
									s.is_empty()
								}
								['d', s @ ..] => {
									match s {
										[] => true,
										['n', 'a', 'l', '\u{1}', s @ ..] => {
											super::push(out, 9491);
											s.is_empty()
										}
										['r', 'a', 'y', '\u{1}', s @ ..] => {
											super::push(out, 9492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'c', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 7653);
									s.is_empty()
								}
								['r', 'o', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', 'o', 'f', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['e', 'l', 'g', 'n', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				['r', 'e', 't', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2319);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tm.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 242);
	s.is_empty()
}

/// Words ending -um.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 819);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 2361);
					s.is_empty()
				}
				['y', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 7658);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 7659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5134);
							match s {
								[] => true,
								['u', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 7660);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2365);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['h', 't', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9493);
											s.is_empty()
										}
										['t', 's', 'a', 'g', '\u{1}', s @ ..] => {
											super::push(out, 9494);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 7661);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 5136);
							match s {
								[] => true,
								['a', 'y', 'a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 7662);
									s.is_empty()
								}
								['e', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 7663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'i', s @ ..] => {
							super::push(out, 5137);
							s.is_empty()
						}
						['r', 't', 's', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 5138);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5139);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2366);
					match s {
						[] => true,
						['r', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 5140);
							s.is_empty()
						}
						['u', 'j', '\u{1}', s @ ..] => {
							super::push(out, 5141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t3::n2560(s, out),
				['k', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5145);
							s.is_empty()
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 5146);
							s.is_empty()
						}
						['g', 'i', 's', '\u{1}', s @ ..] => {
							super::push(out, 5147);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 5148);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 7684);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 7685);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 7686);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 5149);
							s.is_empty()
						}
						['y', 'h', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5150);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				['n', s @ ..] => super::t3::n2564(s, out),
				['o', 'z', 'o', 'y', 'r', 'b', '\u{1}', s @ ..] => super::t3::n2565(s, out),
				['p', s @ ..] => super::t3::n2566(s, out),
				['r', s @ ..] => super::t3::n2567(s, out),
				['s', s @ ..] => super::t3::n2568(s, out),
				['t', s @ ..] => super::t3::n2569(s, out),
				['v', s @ ..] => super::t3::n2570(s, out),
				['x', 'a', 's', '\u{1}', s @ ..] => super::t3::n2571(s, out),
				['z', s @ ..] => super::t3::n2572(s, out),
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t3::n880(s, out),
		_ => false,
	}
}

/// Words ending -vm.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	s.is_empty()
}

/// Words ending -wm.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	s.is_empty()
}

/// Words ending -xm.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['n', '\u{1}', s @ ..] => {
					super::push(out, 2394);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ym.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['\u{12}', 'l', s @ ..] => {
			super::push(out, 823);
			s.is_empty()
		}
		['\u{13}', s @ ..] => {
			super::push(out, 824);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -an.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 2395);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 2396);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2397);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2398);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 2399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5227);
							s.is_empty()
						}
						['s', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['y', '\u{1}', s @ ..] => {
							super::push(out, 5229);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 2401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2402);
					match s {
						[] => true,
						['g', 'n', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 5230);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 5231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['m', 'o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 5232);
							s.is_empty()
						}
						['n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5233);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 5234);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 7870);
									s.is_empty()
								}
								['k', '\u{1}', s @ ..] => {
									super::push(out, 7871);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 5235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 2404);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5236);
							match s {
								[] => true,
								['i', 'j', '\u{1}', s @ ..] => {
									super::push(out, 7872);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5237);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2405);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5238);
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => super::t3::n8599(s, out),
								_ => false,
							}
						}
						['m', 'a', 'l', '\u{1}', s @ ..] => super::t3::n5663(s, out),
						_ => false,
					}
				}
				['e', s @ ..] => super::t3::n2605(s, out),
				['f', s @ ..] => super::t3::n2606(s, out),
				['g', s @ ..] => super::t3::n2607(s, out),
				['h', s @ ..] => super::t3::n2608(s, out),
				['i', s @ ..] => super::t3::n2609(s, out),
				['j', s @ ..] => super::t3::n2610(s, out),
				['k', s @ ..] => super::t3::n2611(s, out),
				['l', s @ ..] => super::t3::n2612(s, out),
				['m', s @ ..] => super::t3::n2613(s, out),
				['n', s @ ..] => super::t3::n2614(s, out),
				['o', s @ ..] => super::t3::n2615(s, out),
				['p', s @ ..] => super::t3::n2616(s, out),
				['r', s @ ..] => super::t3::n2617(s, out),
				['s', s @ ..] => super::t3::n2618(s, out),
				['t', s @ ..] => super::t3::n2619(s, out),
				['u', s @ ..] => super::t3::n2620(s, out),
				['v', s @ ..] => super::t3::n2621(s, out),
				['w', s @ ..] => super::t3::n2622(s, out),
				['y', s @ ..] => super::t3::n2623(s, out),
				['z', s @ ..] => super::t3::n2624(s, out),
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t3::n886(s, out),
		_ => false,
	}
}

/// Words ending -sen (2 syllables).
pub(crate) fn n2666(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2466);
	match s {
		[] => true,
		['n', 'o', '\u{1}', s @ ..] => {
			super::push(out, 5333);
			s.is_empty()
		}
		['u', 'f', '\u{1}', s @ ..] => {
			super::push(out, 5334);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ten (2 syllables).
pub(crate) fn n2667(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2467);
	s.is_empty()
}

/// Words ending -yen (2 syllables).
pub(crate) fn n2668(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2468);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 5335);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 7984);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 9817);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 9818);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
			super::push(out, 5336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zen (2 syllables).
pub(crate) fn n2669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2469);
	s.is_empty()
}

/// Words ending -en (3 syllables).
pub(crate) fn n889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 830);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 2470);
			match s {
				[] => true,
				['i', 'b', 'i', 't', '\u{1}', s @ ..] => {
					super::push(out, 5337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 2471);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5338);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 7985);
							match s {
								[] => true,
								['g', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 9819);
									s.is_empty()
								}
								['n', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9820);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', 'a', 'h', 'n', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5339);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2472);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 5340);
					s.is_empty()
				}
				['t', 'n', s @ ..] => {
					super::push(out, 5341);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 7986);
							match s {
								[] => true,
								['r', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 9821);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 2473);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 5342);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 7987);
							match s {
								[] => true,
								['w', 'k', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 9822);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 5343);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 5344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 2474);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 7988);
							match s {
								[] => true,
								['o', 'r', 'g', 'n', 'e', 'i', 'r', 'a', 'm', '\u{1}', s @ ..] => {
									super::push(out, 9823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'k', 'b', 'e', 'l', '\u{1}', s @ ..] => {
							super::push(out, 7989);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2475);
			match s {
				[] => true,
				['l', 'o', 'p', 'o', 'r', 'o', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 5345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 2476);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 5346);
					match s {
						[] => true,
						['d', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 7990);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 7991);
							match s {
								[] => true,
								['o', 'f', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10712);
											s.is_empty()
										}
										['o', 'r', 'u', 'e', 'n', '\u{1}', s @ ..] => {
											super::push(out, 10713);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'c', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 7992);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'l', 't', 'n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5347);
					s.is_empty()
				}
				['i', 'r', 'c', 's', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 5348);
					s.is_empty()
				}
				['o', 'n', s @ ..] => {
					super::push(out, 5349);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 7993);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 9824);
									s.is_empty()
								}
								['r', 't', '\u{1}', s @ ..] => {
									super::push(out, 9825);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['c', 'a', 'c', '\u{1}', s @ ..] => {
							super::push(out, 7994);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 7995);
							s.is_empty()
						}
						['t', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 7996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 2477);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 5350);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 7997);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 9826);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 10714);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2478);
			match s {
				[] => true,
				['n', 'a', 'k', 'n', 'i', 'h', 's', '\u{1}', s @ ..] => {
					super::push(out, 5351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 2479);
			match s {
				[] => true,
				['t', 'o', 'g', 'e', 'b', 's', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5352);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 5353);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 7998);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 9827);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 10715);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 2480);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -fn.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	s.is_empty()
}

/// Words ending -gn.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	s.is_empty()
}

/// Words ending -hn.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	s.is_empty()
}

/// Words ending -in.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2481);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 5354);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5355);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['a', 'r', 'h', 'k', 'u', 'h', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2484);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['h', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5356);
							s.is_empty()
						}
						['t', 's', 'd', 'n', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5357);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 't', s @ ..] => {
					super::push(out, 2486);
					match s {
						[] => true,
						['o', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 5358);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 's', 'm', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 2487);
					s.is_empty()
				}
				['j', s @ ..] => {
					super::push(out, 2488);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5359);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 9828);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 9829);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 7999);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', 'y', s @ ..] => {
							super::push(out, 5360);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8000);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 5361);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 5362);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 5363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 2489);
					match s {
						[] => true,
						['r', 'a', s @ ..] => {
							super::push(out, 5364);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 8001);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 2490);
					match s {
						[] => true,
						['g', 'n', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 5365);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 5366);
							match s {
								[] => true,
								['o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 8002);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5367);
							match s {
								[] => true,
								['w', '\u{1}', s @ ..] => {
									super::push(out, 8003);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2491);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				['q', s @ ..] => {
					super::push(out, 2493);
					match s {
						[] => true,
						['e', 'u', 'y', '\u{1}', s @ ..] => {
							super::push(out, 5368);
							s.is_empty()
						}
						['i', 'u', 'h', 'z', '\u{1}', s @ ..] => {
							super::push(out, 5369);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 5370);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 8004);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 8005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2494);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5371);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2495);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5372);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2496);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2497);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 2498);
					match s {
						[] => true,
						['t', 's', s @ ..] => {
							super::push(out, 5373);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'w', 'o', 'l', 'l', 'e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 2499);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 5374);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8006);
									match s {
										[] => true,
										['u', 'q', 'u', 'a', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9830);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 2501);
					match s {
						[] => true,
						['e', 'm', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5375);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2502);
					match s {
						[] => true,
						['a', 'i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5376);
							s.is_empty()
						}
						['e', 'c', s @ ..] => {
							super::push(out, 5377);
							match s {
								[] => true,
								['a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 8007);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 5378);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5379);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 8008);
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['a', 'r', 'd', '\u{1}', s @ ..] => {
													super::push(out, 10716);
													s.is_empty()
												}
												['l', 'a', '\u{1}', s @ ..] => {
													super::push(out, 10717);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 't', 'c', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 9831);
											s.is_empty()
										}
										['m', 'e', 'd', '\u{1}', s @ ..] => super::t3::n10796(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'h', s @ ..] => super::t3::n5814(s, out),
						['s', s @ ..] => super::t3::n5815(s, out),
						['u', s @ ..] => super::t3::n5816(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t3::n2704(s, out),
				['m', s @ ..] => super::t3::n2705(s, out),
				['n', s @ ..] => super::t3::n2706(s, out),
				['r', 'r', s @ ..] => super::t3::n2707(s, out),
				['s', 'i', 'm', s @ ..] => super::t3::n2708(s, out),
				['t', 'a', 'r', s @ ..] => super::t3::n2709(s, out),
				['w', s @ ..] => super::t3::n2710(s, out),
				['x', s @ ..] => super::t3::n2711(s, out),
				['z', s @ ..] => super::t3::n2712(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kn.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['\u{10}', '\u{1}', s @ ..] => {
			super::push(out, 834);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ln.
pub(crate) fn n258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	s.is_empty()
}

/// Words ending -mn.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['h', 'c', 'w', 's', '\u{1}', s @ ..] => {
					super::push(out, 2512);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2513);
					match s {
						[] => true,
						['h', 'w', '\u{1}', s @ ..] => {
							super::push(out, 5394);
							s.is_empty()
						}
						['l', 's', '\u{1}', s @ ..] => {
							super::push(out, 5395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 836);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nn.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 837);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['i', 'j', s @ ..] => {
					super::push(out, 2514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 839);
			match s {
				[] => true,
				['a', 'm', 't', 'm', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -on.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2516);
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 5396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2517);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 5397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 2518);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 2519);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5398);
							match s {
								[] => true,
								['a', 'n', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8017);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2520);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 5399);
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 8018);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 5400);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', '\u{1}', s @ ..] => {
					super::push(out, 2521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 841);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2522);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 5401);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8019);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 2523);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 5402);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8020);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5403);
							s.is_empty()
						}
						['d', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5404);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 5405);
							match s {
								[] => true,
								['r', 'u', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 8021);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5406);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'y', 'z', '\u{1}', s @ ..] => {
					super::push(out, 2524);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 2525);
					match s {
						[] => true,
						['c', 'r', s @ ..] => {
							super::push(out, 5407);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 8022);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 5408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 2526);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 5409);
							match s {
								[] => true,
								['n', 'e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 8023);
									s.is_empty()
								}
								['p', 'o', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 8024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 5410);
							match s {
								[] => true,
								['n', 'a', 'c', '\u{1}', s @ ..] => {
									super::push(out, 8025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 5411);
							match s {
								[] => true,
								['a', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 8026);
									s.is_empty()
								}
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8027);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 5412);
							match s {
								[] => true,
								['d', 'o', '\u{1}', s @ ..] => {
									super::push(out, 8028);
									s.is_empty()
								}
								['m', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 8029);
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => super::t3::n8779(s, out),
								_ => false,
							}
						}
						['g', s @ ..] => super::t3::n5848(s, out),
						['l', s @ ..] => super::t3::n5849(s, out),
						['m', 'e', s @ ..] => super::t3::n5850(s, out),
						['n', 'i', s @ ..] => super::t3::n5851(s, out),
						['p', s @ ..] => super::t3::n5852(s, out),
						['r', s @ ..] => super::t3::n5853(s, out),
						['s', 'o', 'r', 'g', '\u{1}', s @ ..] => super::t3::n5854(s, out),
						['t', s @ ..] => super::t3::n5855(s, out),
						_ => false,
					}
				}
				['j', s @ ..] => super::t3::n2729(s, out),
				['k', s @ ..] => super::t3::n2730(s, out),
				['l', s @ ..] => super::t3::n2731(s, out),
				['m', s @ ..] => super::t3::n2732(s, out),
				['n', s @ ..] => super::t3::n2733(s, out),
				['o', s @ ..] => super::t3::n2734(s, out),
				['p', s @ ..] => super::t3::n2735(s, out),
				['r', s @ ..] => super::t3::n2736(s, out),
				['s', s @ ..] => super::t3::n2737(s, out),
				['t', s @ ..] => super::t3::n2738(s, out),
				['x', 'a', 't', '\u{1}', s @ ..] => super::t3::n2739(s, out),
				['y', s @ ..] => super::t3::n2740(s, out),
				['z', s @ ..] => super::t3::n2741(s, out),
				_ => false,
			}
		}
		['\u{13}', s @ ..] => super::t3::n901(s, out),
		_ => false,
	}
}

/// Words ending -pn.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	s.is_empty()
}

/// Words ending -rn.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['l', '\u{1}', s @ ..] => {
					super::push(out, 2557);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', 'o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2558);
					match s {
						[] => true,
						['t', 's', 'r', 'i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'g', 'n', s @ ..] => {
					super::push(out, 2559);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', 'o', 'b', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['l', 'e', 'h', 't', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2560);
					s.is_empty()
				}
				['n', 'o', 'g', 'a', 'r', 'd', '\u{1}', s @ ..] => {
					super::push(out, 2561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sn.
pub(crate) fn n264(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 263);
	match s {
		[] => true,
		['\u{10}', 'c', 'c', '\u{1}', s @ ..] => {
			super::push(out, 845);
			s.is_empty()
		}
		['\u{11}', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 846);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tn.
pub(crate) fn n265(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 264);
	s.is_empty()
}

/// Words ending -un.
pub(crate) fn n266(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['\u{11}', 'j', '\u{1}', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 2562);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 5520);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 8186);
									match s {
										[] => true,
										['h', '\u{1}', s @ ..] => {
											super::push(out, 9966);
											s.is_empty()
										}
										['k', '\u{1}', s @ ..] => {
											super::push(out, 9967);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 2563);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 5521);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 2564);
					match s {
						[] => true,
						['t', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5522);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 2565);
					match s {
						[] => true,
						['k', 'i', 't', '\u{1}', s @ ..] => {
							super::push(out, 5523);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'm', s @ ..] => {
					super::push(out, 2566);
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 5524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2567);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 2568);
					match s {
						[] => true,
						['k', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'n', 'u', 'l', 'e', '\u{1}', s @ ..] => {
					super::push(out, 2569);
					s.is_empty()
				}
				['k', 'i', 'j', 'u', 'k', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 2570);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wn.
pub(crate) fn n267(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['\u{10}', s @ ..] => {
			super::push(out, 849);
			s.is_empty()
		}
		['\u{11}', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 5526);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 5527);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2571);
					match s {
						[] => true,
						['p', 's', 'l', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xn.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['\u{10}', 'm', '\u{1}', s @ ..] => {
			super::push(out, 852);
			s.is_empty()
		}
		['\u{11}', 'm', 'o', 'w', '\u{1}', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yn.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['h', 't', 'w', 'b', '\u{1}', s @ ..] => {
					super::push(out, 2572);
					s.is_empty()
				}
				['y', 't', '\u{1}', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['b', 'm', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2574);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 2575);
					match s {
						[] => true,
						['g', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', 'w', '\u{1}', s @ ..] => {
					super::push(out, 2576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['m', 'o', 'w', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 2577);
					s.is_empty()
				}
				['s', 'n', 'i', 't', 'e', 'y', '\u{1}', s @ ..] => {
					super::push(out, 2578);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -do (2 syllables).
pub(crate) fn n927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 862);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 2604);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5546);
					s.is_empty()
				}
				['e', '\u{1}', s @ ..] => {
					super::push(out, 5547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2605);
			match s {
				[] => true,
				['e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 5548);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 2606);
			match s {
				[] => true,
				['o', 's', '\u{1}', s @ ..] => {
					super::push(out, 5549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2607);
			s.is_empty()
		}
		['o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 2608);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2609);
			match s {
				[] => true,
				['a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5550);
					s.is_empty()
				}
				['i', 'e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 5551);
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					super::push(out, 5552);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -do (3 syllables).
pub(crate) fn n928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 2610);
			match s {
				[] => true,
				['l', 'a', 'c', 's', 'e', '\u{1}', s @ ..] => {
					super::push(out, 5553);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 5554);
					match s {
						[] => true,
						['i', 't', 's', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 8192);
							s.is_empty()
						}
						['r', 'o', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 9969);
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 9970);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'p', 's', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 8194);
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 8195);
							s.is_empty()
						}
						['o', 'm', 'a', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8196);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 5555);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 8197);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 10798);
											s.is_empty()
										}
										['o', 'r', 'c', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 10799);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 8198);
							match s {
								[] => true,
								['o', 't', '\u{1}', s @ ..] => {
									super::push(out, 9971);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 8199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'a', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 2612);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 5557);
					match s {
						[] => true,
						['n', 'a', 'r', 'e', 'p', 's', 'e', '\u{1}', s @ ..] => {
							super::push(out, 8200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['a', 's', 's', 'i', 'l', 'g', '\u{1}', s @ ..] => {
					super::push(out, 5558);
					s.is_empty()
				}
				['e', 'u', s @ ..] => {
					super::push(out, 5559);
					match s {
						[] => true,
						['n', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8201);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 2614);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 2615);
			match s {
				[] => true,
				['a', 'i', 'o', 'v', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 5560);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'f', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 8202);
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 8203);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 2616);
			match s {
				[] => true,
				['r', 'c', 'e', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 5561);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eo (2 syllables).
pub(crate) fn n929(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'h', 't', 'g', 'n', 'o', 's', '\u{1}', s @ ..] => {
			super::push(out, 2617);
			s.is_empty()
		}
		['d', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 2618);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -go.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['\u{11}', '\u{1}', s @ ..] => {
			super::push(out, 864);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2619);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 5562);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 5563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2620);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 5564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2621);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5565);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8204);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 8205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2622);
					match s {
						[] => true,
						['y', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5566);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 2623);
					match s {
						[] => true,
						['a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 5567);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 2624);
					s.is_empty()
				}
				['w', 'o', 'n', 's', '\u{1}', s @ ..] => {
					super::push(out, 2625);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 2626);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 5568);
							match s {
								[] => true,
								['e', 'p', 'i', 'h', 'c', 'r', 'a', '\u{1}', s @ ..] => {
									super::push(out, 8206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 5569);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9972);
											s.is_empty()
										}
										['e', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 9973);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 5570);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 8207);
									match s {
										[] => true,
										['b', 'i', 'k', '\u{1}', s @ ..] => {
											super::push(out, 9974);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 5571);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 8208);
									s.is_empty()
								}
								['r', 'a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 8209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 5572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 2627);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'i', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 5573);
							s.is_empty()
						}
						['d', 'n', s @ ..] => {
							match s {
								[] => true,
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 8210);
									s.is_empty()
								}
								['i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 8211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 5574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 2628);
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							super::push(out, 5575);
							match s {
								[] => true,
								['n', 'a', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8212);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 8213);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 5576);
							match s {
								[] => true,
								['t', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8214);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2629);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 2630);
					match s {
						[] => true,
						['a', 'b', s @ ..] => {
							super::push(out, 5577);
							match s {
								[] => true,
								['m', 'e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 9975);
											s.is_empty()
										}
										['r', 'e', 't', 'n', 'u', 'o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 9976);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 'v', 'o', '\u{1}', s @ ..] => {
							super::push(out, 5578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 2631);
					match s {
						[] => true,
						['h', 's', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 5579);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ho.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['\u{11}', s @ ..] => {
			super::push(out, 867);
			s.is_empty()
		}
		['\u{12}', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 2632);
					match s {
						[] => true,
						['c', 'e', '\u{1}', s @ ..] => {
							super::push(out, 5580);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 5581);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 8215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 2633);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 2634);
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 5582);
							s.is_empty()
						}
						['i', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 5583);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 5584);
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 8216);
									s.is_empty()
								}
								['o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 8217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'o', 's', '\u{1}', s @ ..] => {
					super::push(out, 2635);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			match s {
				[] => true,
				['a', 'k', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 2636);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 2637);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 5586);
							match s {
								[] => true,
								['b', 'u', 's', '\u{1}', s @ ..] => {
									super::push(out, 8218);
									s.is_empty()
								}
								['o', 's', 'i', '\u{1}', s @ ..] => {
									super::push(out, 8219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 2638);
					match s {
						[] => true,
						['a', 'b', 'n', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 5587);
							s.is_empty()
						}
						['n', 'e', 's', 'u', 'f', '\u{1}', s @ ..] => {
							super::push(out, 5588);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 2639);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 5589);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 8220);
									match s {
										[] => true,
										['a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 9977);
											s.is_empty()
										}
										['e', 's', '\u{1}', s @ ..] => {
											super::push(out, 9978);
											s.is_empty()
										}
										['o', 'm', '\u{1}', s @ ..] => {
											super::push(out, 9979);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

