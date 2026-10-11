use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['l', 'i', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 163);
							s.is_empty()
						}
						['s', 's', 'a', 'l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'o', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 165);
							s.is_empty()
						}
						['l', 'l', 'e', 'h', '\u{1}', s @ ..] => {
							super::push(out, 166);
							s.is_empty()
						}
						['n', 'e', 'h', 'c', 'n', 'i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 167);
							s.is_empty()
						}
						['t', 'l', 'a', '\u{1}', s @ ..] => {
							super::push(out, 168);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 25);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 26);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 169);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 545);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 170);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 't', '\u{1}', s @ ..] => {
					super::push(out, 27);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 171);
							match s {
								[] => true,
								['p', 'o', s @ ..] => {
									match s {
										[] => true,
										['n', 'ä', s @ ..] => {
											super::push(out, 1291);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 1292);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 172);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 546);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 173);
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							super::push(out, 174);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'a', 't', '\u{1}', s @ ..] => {
					super::push(out, 29);
					s.is_empty()
				}
				['ü', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 30);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 3);
			s.is_empty()
		}
		['d', s @ ..] => super::t0::n4(s, out),
		['e', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 42);
					s.is_empty()
				}
				['c', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 43);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 44);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 195);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 582);
									s.is_empty()
								}
								['r', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 583);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 196);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 197);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 198);
							s.is_empty()
						}
						['ü', 'm', s @ ..] => {
							super::push(out, 199);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 584);
									s.is_empty()
								}
								['a', 'p', 'o', 'r', 'u', 'e', '\u{1}', s @ ..] => {
									super::push(out, 585);
									s.is_empty()
								}
								['f', 'p', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 586);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 587);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['g', 'e', 'i', 'r', 'k', '\u{1}', s @ ..] => {
											super::push(out, 1386);
											s.is_empty()
										}
										['n', 'e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
											super::push(out, 1387);
											s.is_empty()
										}
										['t', 'm', 'a', '\u{1}', s @ ..] => {
											super::push(out, 1388);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 45);
					match s {
						[] => true,
						['a', 's', '\u{1}', s @ ..] => {
							super::push(out, 200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 46);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 201);
							s.is_empty()
						}
						['g', 'ü', 'l', 'f', '\u{1}', s @ ..] => {
							super::push(out, 202);
							s.is_empty()
						}
						['i', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 588);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 589);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 590);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 591);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => super::t0::n1716(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', s @ ..] => super::t0::n229(s, out),
						['ö', 'r', 'd', '\u{1}', s @ ..] => super::t0::n230(s, out),
						_ => false,
					}
				}
				['h', s @ ..] => super::t0::n50(s, out),
				['k', s @ ..] => super::t0::n51(s, out),
				['l', s @ ..] => super::t0::n52(s, out),
				['m', s @ ..] => super::t0::n53(s, out),
				['n', s @ ..] => super::t0::n54(s, out),
				['r', s @ ..] => super::t0::n55(s, out),
				['s', s @ ..] => super::t0::n56(s, out),
				['t', s @ ..] => super::t0::n57(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t0::n6(s, out),
		['g', s @ ..] => super::t0::n7(s, out),
		['h', s @ ..] => super::t0::n8(s, out),
		['i', s @ ..] => super::t0::n9(s, out),
		['k', s @ ..] => super::t0::n10(s, out),
		['l', s @ ..] => super::t0::n11(s, out),
		['m', s @ ..] => super::t0::n12(s, out),
		['n', s @ ..] => super::t0::n13(s, out),
		['p', s @ ..] => super::t0::n14(s, out),
		['r', s @ ..] => super::t0::n15(s, out),
		['s', s @ ..] => super::t0::n16(s, out),
		['t', s @ ..] => super::t0::n17(s, out),
		['u', s @ ..] => super::t0::n18(s, out),
		['v', s @ ..] => super::t0::n19(s, out),
		['w', s @ ..] => super::t0::n20(s, out),
		['x', s @ ..] => super::t0::n21(s, out),
		['y', s @ ..] => super::t0::n22(s, out),
		['z', s @ ..] => super::t0::n23(s, out),
		['ß', s @ ..] => super::t0::n24(s, out),
		_ => false,
	}
}

/// Words ending -d.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 31);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 175);
					s.is_empty()
				}
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'm', 'e', 'r', 'f', '\u{1}', s @ ..] => {
			super::push(out, 32);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['b', 'r', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 177);
					s.is_empty()
				}
				['e', 'l', s @ ..] => {
					super::push(out, 178);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 179);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					super::push(out, 180);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1293);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 547);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 548);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 550);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 551);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 181);
					s.is_empty()
				}
				['z', 'u', s @ ..] => {
					super::push(out, 182);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['i', 'w', 's', 'l', 'e', 'f', 'u', 'e', 't', 's', 'h', 'c', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 183);
					s.is_empty()
				}
				['u', 'h', 'c', 's', 't', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['e', 'r', 'f', s @ ..] => {
					match s {
						[] => true,
						['d', 'l', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 552);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 553);
							s.is_empty()
						}
						['r', 'e', 'l', 'i', 'e', 't', '\u{1}', s @ ..] => {
							super::push(out, 554);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', 'e', 'i', 'r', 't', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1295);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'r', 'a', '\u{1}', s @ ..] => {
							super::push(out, 555);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n39(s, out),
		['o', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['m', 'm', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 193);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 38);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 39);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 40);
			match s {
				[] => true,
				['l', 'b', 'u', s @ ..] => {
					super::push(out, 194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'm', '\u{1}', s @ ..] => {
			super::push(out, 41);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word orange.
pub(crate) fn n1716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1389);
	s.is_empty()
}

/// Words ending -äge.
pub(crate) fn n229(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	s.is_empty()
}

/// The word dröge.
pub(crate) fn n230(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	s.is_empty()
}

/// Words ending -he.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['a', 'n', '\u{1}', s @ ..] => {
			super::push(out, 205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ke.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['e', 'i', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 207);
			s.is_empty()
		}
		['o', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['r', 'o', 'n', 'k', '\u{1}', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -le.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 592);
					s.is_empty()
				}
				['u', 'h', 'c', 'a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -me.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	s.is_empty()
}

/// Words ending -ne.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	s.is_empty()
}

/// Words ending -re.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -se.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 53);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2396);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2397);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2398);
									match s {
										[] => true,
										['g', 'a', s @ ..] => {
											match s {
												[] => true,
												['l', 'k', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n6184(s, out),
														['r', s @ ..] => super::t0::n6185(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 4419);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 2399);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 2400);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3537);
											s.is_empty()
										}
										['r', 'e', 'z', 'i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 3538);
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 2401);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 2402);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['g', s @ ..] => {
													super::push(out, 4420);
													s.is_empty()
												}
												['h', s @ ..] => {
													super::push(out, 4421);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 3539);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2403);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2404);
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 3540);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2405);
									match s {
										[] => true,
										['h', 'c', 'a', s @ ..] => {
											super::push(out, 3541);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 3542);
											s.is_empty()
										}
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
					super::push(out, 596);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 212);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['b', 'd', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -te.
pub(crate) fn n57(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['a', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 598);
					s.is_empty()
				}
				['u', 'd', '\u{1}', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['ö', 't', '\u{1}', s @ ..] => {
					super::push(out, 600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['c', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['d', 'n', 'e', 's', 'u', 'a', 't', 'n', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'i', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1393);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 1394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -f.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	match s {
		[] => true,
		['e', 'i', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 219);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'u', 'r', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 604);
							s.is_empty()
						}
						['e', 'i', 'n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 605);
							s.is_empty()
						}
						['l', 'e', 'h', 'c', 'ö', 'n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 606);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 55);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1395);
									s.is_empty()
								}
								['h', s @ ..] => {
									match s {
										[] => true,
										['c', 'u', 'r', 'b', 'b', 'a', '\u{1}', s @ ..] => {
											super::push(out, 2406);
											s.is_empty()
										}
										['ü', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2407);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1396);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1397);
									match s {
										[] => true,
										['b', 'u', 'a', 'l', 'r', 'u', '\u{1}', s @ ..] => {
											super::push(out, 2408);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 't', 'o', 'r', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 1398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', '\u{1}', s @ ..] => {
					super::push(out, 220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'o', 'd', 'n', 'e', 't', 't', 'o', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 57);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['m', 'u', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 221);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['a', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 608);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 609);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -g.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['i', s @ ..] => super::t0::n64(s, out),
		['n', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 241);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 721);
							s.is_empty()
						}
						['l', '\u{1}', s @ ..] => {
							super::push(out, 722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 242);
					match s {
						[] => true,
						['l', 'l', 'a', 'n', 'k', '\u{1}', s @ ..] => {
							super::push(out, 723);
							s.is_empty()
						}
						['t', 'u', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 724);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 243);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 244);
					match s {
						[] => true,
						['j', '\u{1}', s @ ..] => {
							super::push(out, 725);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 62);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 245);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 64);
			match s {
				[] => true,
				['l', 'k', 't', 'l', 'a', s @ ..] => {
					super::push(out, 247);
					s.is_empty()
				}
				['n', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 65);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -h.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 727);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 728);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 729);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', s @ ..] => {
									super::push(out, 1548);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1549);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 730);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1550);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1551);
									match s {
										[] => true,
										['n', 'u', 'r', 'e', 'i', 'g', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 2614);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1552);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 731);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 732);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n71(s, out),
		['e', s @ ..] => {
			super::push(out, 68);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 69);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 751);
							s.is_empty()
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1604);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['b', 'r', 'a', 'f', '\u{1}', s @ ..] => {
											super::push(out, 2797);
											s.is_empty()
										}
										['t', 'a', 't', '\u{1}', s @ ..] => {
											super::push(out, 2798);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'n', 'e', 'b', 'e', 'l', '\u{1}', s @ ..] => {
									super::push(out, 1605);
									s.is_empty()
								}
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
			super::push(out, 71);
			match s {
				[] => true,
				['o', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['o', 'g', s @ ..] => {
							super::push(out, 752);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'o', 'm', 'o', 's', s @ ..] => {
					super::push(out, 256);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', 'i', s @ ..] => {
			match s {
				[] => true,
				['k', 'w', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 257);
					s.is_empty()
				}
				['l', 'y', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 72);
			match s {
				[] => true,
				['o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 259);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'a', 'r', '\u{1}', s @ ..] => {
			super::push(out, 73);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 74);
			match s {
				[] => true,
				['z', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 75);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -i.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['e', 'r', 'f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 261);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 262);
					match s {
						[] => true,
						['n', 'a', 'w', 'n', 'i', 'e', '\u{1}', s @ ..] => {
							super::push(out, 754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', s @ ..] => {
					super::push(out, 263);
					match s {
						[] => true,
						['e', 'i', 'r', 'r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 756);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 't', s @ ..] => {
					super::push(out, 264);
					match s {
						[] => true,
						['t', 'i', 'l', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 759);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'g', 'n', 'u', 'r', s @ ..] => {
					super::push(out, 265);
					match s {
						[] => true,
						['ö', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 761);
							s.is_empty()
						}
						['s', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'r', s @ ..] => {
					super::push(out, 266);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -k.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 76);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 77);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['p', 's', '\u{1}', s @ ..] => {
							super::push(out, 763);
							s.is_empty()
						}
						['r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 764);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'k', '\u{1}', s @ ..] => {
					super::push(out, 267);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1606);
									s.is_empty()
								}
								['t', 's', 'u', 'a', 'f', '\u{1}', s @ ..] => {
									super::push(out, 1607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', '\u{1}', s @ ..] => {
							super::push(out, 765);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 268);
					s.is_empty()
				}
				['u', 'm', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 269);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 78);
			s.is_empty()
		}
		['l', 'e', 'w', s @ ..] => {
			super::push(out, 79);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 80);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 767);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1608);
									s.is_empty()
								}
								['h', 'c', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2799);
											s.is_empty()
										}
										['n', 'e', 't', 'r', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 2800);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 768);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1609);
									s.is_empty()
								}
								['k', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2801);
											s.is_empty()
										}
										['n', 'e', 'g', s @ ..] => {
											super::push(out, 2802);
											s.is_empty()
										}
										['r', 'e', 'w', s @ ..] => {
											super::push(out, 2803);
											s.is_empty()
										}
										['s', 'e', 't', s @ ..] => {
											super::push(out, 2804);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 769);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 270);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 271);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1610);
									s.is_empty()
								}
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1611);
									s.is_empty()
								}
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
			super::push(out, 81);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 82);
			match s {
				[] => true,
				['a', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 770);
							s.is_empty()
						}
						['b', 'l', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 771);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1612);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1613);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 772);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 773);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1614);
									s.is_empty()
								}
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
			super::push(out, 83);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'k', 'f', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 774);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'u', 'j', 'n', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 1615);
									s.is_empty()
								}
								['i', 'l', 'p', 'a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 1616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'k', 'i', 'p', '\u{1}', s @ ..] => {
							super::push(out, 775);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 776);
							s.is_empty()
						}
						['u', 't', 'n', 'a', 'g', 'r', 'a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'd', s @ ..] => {
			super::push(out, 84);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 85);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -l.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 86);
			match s {
				[] => true,
				['a', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 272);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 273);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['o', 'l', 'l', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1617);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1618);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 778);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 274);
					match s {
						[] => true,
						['h', 'c', 'a', 'r', 't', 'o', 'd', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 780);
							s.is_empty()
						}
						['n', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 781);
							s.is_empty()
						}
						['r', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 782);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'u', 'r', s @ ..] => {
					super::push(out, 275);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 276);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 783);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1619);
									s.is_empty()
								}
								['o', 'r', 't', 's', 'a', 't', 'a', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1620);
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
						['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 784);
							s.is_empty()
						}
						['d', 'e', 'm', 'i', s @ ..] => {
							super::push(out, 785);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 786);
							s.is_empty()
						}
						['h', 'c', 'a', s @ ..] => {
							super::push(out, 787);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 788);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1621);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1622);
									match s {
										[] => true,
										['g', '\u{1}', s @ ..] => {
											super::push(out, 2805);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 1623);
									match s {
										[] => true,
										['l', 'o', 'k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4020);
													s.is_empty()
												}
												['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
													super::push(out, 4021);
													s.is_empty()
												}
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
							super::push(out, 789);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 790);
							s.is_empty()
						}
						['v', s @ ..] => {
							match s {
								[] => true,
								['i', 'r', s @ ..] => {
									super::push(out, 1624);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1625);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 791);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 792);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1626);
									s.is_empty()
								}
								['e', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 1627);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1628);
									match s {
										[] => true,
										['s', s @ ..] => super::t0::n3558(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => super::t0::n322(s, out),
				['m', s @ ..] => super::t0::n323(s, out),
				['n', s @ ..] => super::t0::n324(s, out),
				['p', 'i', 'd', s @ ..] => super::t0::n325(s, out),
				['r', s @ ..] => super::t0::n326(s, out),
				['s', s @ ..] => super::t0::n327(s, out),
				['t', s @ ..] => super::t0::n328(s, out),
				['u', s @ ..] => super::t0::n329(s, out),
				['v', s @ ..] => super::t0::n330(s, out),
				['y', s @ ..] => super::t0::n331(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n93(s, out),
		['h', s @ ..] => super::t0::n94(s, out),
		['i', s @ ..] => super::t0::n95(s, out),
		['l', s @ ..] => super::t0::n96(s, out),
		['o', s @ ..] => super::t0::n97(s, out),
		['u', s @ ..] => super::t0::n98(s, out),
		['ü', s @ ..] => super::t0::n99(s, out),
		_ => false,
	}
}

/// Words ending -m.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 93);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 843);
							s.is_empty()
						}
						['g', 'n', 'a', 'l', 'k', s @ ..] => {
							super::push(out, 844);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 845);
							match s {
								[] => true,
								['c', 'r', 'e', 'm', 'f', 'u', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1703);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 2856);
											s.is_empty()
										}
										['i', 'w', s @ ..] => {
											match s {
												[] => true,
												['n', s @ ..] => {
													match s {
														[] => true,
														['e', 'b', 'a', 'g', 'r', 'o', 'v', '\u{1}', s @ ..] => super::t0::n6001(s, out),
														['u', '\u{1}', s @ ..] => super::t0::n6002(s, out),
														_ => false,
													}
												}
												['s', 'g', 'l', 'o', 'f', 'r', 'e', '\u{1}', s @ ..] => {
													super::push(out, 4049);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', 'e', 'm', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'h', 'f', s @ ..] => {
									super::push(out, 1704);
									s.is_empty()
								}
								['u', 'e', 'd', 'e', 'b', 's', s @ ..] => {
									super::push(out, 1705);
									s.is_empty()
								}
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
			super::push(out, 94);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 309);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 95);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 848);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 849);
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 850);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['t', 'i', 'r', s @ ..] => {
					super::push(out, 311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 97);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 312);
					match s {
						[] => true,
						['r', 't', 's', 'n', 'e', 't', 't', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 313);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 314);
					match s {
						[] => true,
						['r', 'f', '\u{1}', s @ ..] => {
							super::push(out, 852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 315);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 853);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1706);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 854);
							s.is_empty()
						}
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 99);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 316);
					match s {
						[] => true,
						['e', 'i', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 857);
							match s {
								[] => true,
								['f', 'o', 't', 's', 'r', 'e', s @ ..] => {
									super::push(out, 1707);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 858);
							s.is_empty()
						}
						['h', 'c', 's', 'u', 'ä', s @ ..] => {
							super::push(out, 859);
							s.is_empty()
						}
						['l', 'e', s @ ..] => {
							super::push(out, 860);
							s.is_empty()
						}
						['m', 'u', 'i', 'c', s @ ..] => {
							super::push(out, 861);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1708);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1709);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 863);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1710);
									s.is_empty()
								}
								['l', 'h', 'ü', 'f', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 1711);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1712);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', s @ ..] => {
							super::push(out, 864);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 865);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1713);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 317);
					match s {
						[] => true,
						['h', 't', s @ ..] => {
							match s {
								[] => true,
								['o', 'r', s @ ..] => {
									super::push(out, 1714);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 318);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1716);
									s.is_empty()
								}
								['n', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1717);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 100);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -n.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['b', 'r', 'u', '\u{1}', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				['e', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 323);
					s.is_empty()
				}
				['g', 'e', s @ ..] => {
					super::push(out, 324);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 325);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['o', 'l', s @ ..] => {
							super::push(out, 868);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 870);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 871);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 872);
							match s {
								[] => true,
								['g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2857);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 2858);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 2859);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 'p', s @ ..] => {
							super::push(out, 873);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n110(s, out),
		['h', s @ ..] => {
			super::push(out, 104);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 342);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1784);
									s.is_empty()
								}
								['l', 'h', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 1785);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1786);
									s.is_empty()
								}
								['e', 'z', 't', 'i', 'l', 'k', '\u{1}', s @ ..] => {
									super::push(out, 1787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1788);
									s.is_empty()
								}
								['l', 'l', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1789);
									s.is_empty()
								}
								['s', 'd', 'n', 'u', 'h', '\u{1}', s @ ..] => {
									super::push(out, 1790);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1791);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['b', 'u', 't', 's', '\u{1}', s @ ..] => {
													super::push(out, 4101);
													s.is_empty()
												}
												['d', 'u', 'j', '\u{1}', s @ ..] => {
													super::push(out, 4102);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 4103);
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', '\u{1}', s @ ..] => {
											super::push(out, 2961);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 's', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1792);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 932);
							s.is_empty()
						}
						['l', 'a', 't', 's', 'i', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', 'l', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['ü', 'd', '\u{1}', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 107);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'l', 'a', s @ ..] => {
							super::push(out, 935);
							s.is_empty()
						}
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1793);
									s.is_empty()
								}
								['o', 'm', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2962);
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 4104);
													s.is_empty()
												}
												['ä', s @ ..] => {
													super::push(out, 4105);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1794);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1795);
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 2963);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => super::t0::n1123(s, out),
						['i', 'e', s @ ..] => super::t0::n1124(s, out),
						['n', s @ ..] => super::t0::n1125(s, out),
						['s', s @ ..] => super::t0::n1126(s, out),
						['t', s @ ..] => super::t0::n1127(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n413(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n116(s, out),
		['y', s @ ..] => super::t0::n117(s, out),
		['ä', s @ ..] => super::t0::n118(s, out),
		['ö', s @ ..] => super::t0::n119(s, out),
		['ü', s @ ..] => super::t0::n120(s, out),
		_ => false,
	}
}

/// Words ending -p.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['h', '\u{1}', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 113);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 114);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					super::push(out, 362);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['b', s @ ..] => super::t0::n426(s, out),
				['d', s @ ..] => {
					super::push(out, 363);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						['n', 'i', 'l', '\u{1}', s @ ..] => {
							super::push(out, 952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', s @ ..] => {
					super::push(out, 364);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 953);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1833);
									s.is_empty()
								}
								['n', 'e', 'n', 'n', s @ ..] => super::t0::n2314(s, out),
								['t', s @ ..] => super::t0::n2315(s, out),
								_ => false,
							}
						}
						['l', s @ ..] => super::t0::n1152(s, out),
						['o', s @ ..] => super::t0::n1153(s, out),
						['u', s @ ..] => super::t0::n1154(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t0::n431(s, out),
				['n', s @ ..] => super::t0::n432(s, out),
				['p', s @ ..] => super::t0::n433(s, out),
				['r', s @ ..] => super::t0::n434(s, out),
				['t', s @ ..] => super::t0::n435(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n126(s, out),
		['h', s @ ..] => super::t0::n127(s, out),
		['i', s @ ..] => super::t0::n128(s, out),
		['o', s @ ..] => super::t0::n129(s, out),
		['r', s @ ..] => super::t0::n130(s, out),
		['u', s @ ..] => super::t0::n131(s, out),
		['ä', s @ ..] => super::t0::n132(s, out),
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['a', 'h', 'p', 'n', 'o', 'k', '\u{1}', s @ ..] => {
			super::push(out, 123);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 398);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', 'a', 'm', '\u{1}', s @ ..] => {
							super::push(out, 992);
							s.is_empty()
						}
						['o', 'p', 'a', 'k', '\u{1}', s @ ..] => {
							super::push(out, 993);
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
				['e', 'w', 'l', 'r', 'e', 'z', 'i', 'z', '\u{1}', s @ ..] => {
					super::push(out, 399);
					s.is_empty()
				}
				['k', 'r', 'ü', 't', '\u{1}', s @ ..] => {
					super::push(out, 400);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 401);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'a', 'k', 'i', 'r', 'f', 'a', '\u{1}', s @ ..] => {
					super::push(out, 402);
					s.is_empty()
				}
				['i', 'e', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 403);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 404);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 405);
					s.is_empty()
				}
				['l', s @ ..] => super::t0::n477(s, out),
				['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 406);
					s.is_empty()
				}
				['n', 'i', 'm', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 407);
					s.is_empty()
				}
				['t', 's', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			match s {
				[] => true,
				['m', 'b', 'u', 's', '\u{1}', s @ ..] => {
					super::push(out, 409);
					s.is_empty()
				}
				['p', 's', 'i', 'd', 'y', 'l', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 410);
					s.is_empty()
				}
				['v', 'n', s @ ..] => {
					super::push(out, 411);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 412);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1906);
									s.is_empty()
								}
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1907);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1908);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 1909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 413);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', '\u{1}', s @ ..] => {
									super::push(out, 1910);
									s.is_empty()
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 1911);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['l', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						['r', s @ ..] => super::t0::n1229(s, out),
						_ => false,
					}
				}
				['ä', 'm', 'e', 'g', s @ ..] => super::t0::n488(s, out),
				['ü', 's', '\u{1}', s @ ..] => {
					super::push(out, 414);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 126);
			s.is_empty()
		}
		['ö', s @ ..] => super::t0::n141(s, out),
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 421);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1920);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1921);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'd', 'i', s @ ..] => {
							super::push(out, 1024);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['g', 'e', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1922);
									s.is_empty()
								}
								['t', 'f', 'n', 'u', 'n', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 1923);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1025);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 422);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'ß', s @ ..] => {
									super::push(out, 1925);
									s.is_empty()
								}
								['r', 'e', 'v', 'l', s @ ..] => {
									super::push(out, 1926);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1026);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', 'l', s @ ..] => {
					super::push(out, 423);
					s.is_empty()
				}
				['l', 'ö', 'w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 424);
					s.is_empty()
				}
				['o', 'n', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 425);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 426);
					match s {
						[] => true,
						['e', 'g', 'e', 'g', 'r', 'e', 't', 't', 'e', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1028);
							s.is_empty()
						}
						['ä', 'f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1029);
							s.is_empty()
						}
						['ü', 'm', 'r', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 427);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['g', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 3093);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 3094);
											match s {
												[] => true,
												['e', 'n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 4162);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3095);
											s.is_empty()
										}
										['s', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3096);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ä', 't', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1032);
							s.is_empty()
						}
						['r', 't', 'e', 'b', s @ ..] => {
							super::push(out, 1033);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', 'e', 'r', 'i', 'd', '\u{1}', s @ ..] => {
			super::push(out, 130);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['n', 'a', s @ ..] => {
					match s {
						[] => true,
						['s', 'e', 'g', 's', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1034);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
									super::push(out, 1928);
									s.is_empty()
								}
								['r', 'e', 'v', s @ ..] => {
									match s {
										[] => true,
										['l', 'e', 'z', 'r', 'u', 'w', '\u{1}', s @ ..] => {
											super::push(out, 3097);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 3098);
											s.is_empty()
										}
										['r', 'u', '\u{1}', s @ ..] => {
											super::push(out, 3099);
											s.is_empty()
										}
										['t', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 3100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n146(s, out),
		['f', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['a', s @ ..] => super::t0::n515(s, out),
				['e', 'i', 'r', s @ ..] => {
					super::push(out, 435);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 436);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 437);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									super::push(out, 1963);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3132);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1964);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 438);
					s.is_empty()
				}
				['p', s @ ..] => super::t0::n520(s, out),
				['r', s @ ..] => {
					super::push(out, 440);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1058);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => super::t0::n522(s, out),
				_ => false,
			}
		}
		['g', s @ ..] => super::t0::n148(s, out),
		['h', s @ ..] => super::t0::n149(s, out),
		['i', s @ ..] => super::t0::n150(s, out),
		['k', s @ ..] => super::t0::n151(s, out),
		['l', s @ ..] => super::t0::n152(s, out),
		['m', s @ ..] => super::t0::n153(s, out),
		['n', s @ ..] => super::t0::n154(s, out),
		['o', s @ ..] => super::t0::n155(s, out),
		['p', s @ ..] => super::t0::n156(s, out),
		['r', s @ ..] => super::t0::n157(s, out),
		['s', s @ ..] => super::t0::n158(s, out),
		['t', s @ ..] => super::t0::n159(s, out),
		['u', s @ ..] => super::t0::n160(s, out),
		['v', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['x', s @ ..] => super::t0::n162(s, out),
		['z', s @ ..] => super::t0::n163(s, out),
		['ß', s @ ..] => super::t0::n164(s, out),
		['ä', s @ ..] => super::t0::n165(s, out),
		_ => false,
	}
}

/// Words ending -u.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2221);
									s.is_empty()
								}
								['l', s @ ..] => {
									match s {
										[] => true,
										['e', 'm', s @ ..] => {
											super::push(out, 3334);
											s.is_empty()
										}
										['h', 'a', 'f', s @ ..] => {
											super::push(out, 3335);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 2222);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1250);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1251);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 523);
					s.is_empty()
				}
				['n', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 1253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1255);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1256);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2223);
									s.is_empty()
								}
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
			super::push(out, 151);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 524);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						['d', 'n', 'a', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1258);
							s.is_empty()
						}
						['l', 'e', 'g', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1259);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2224);
									s.is_empty()
								}
								['s', 'b', s @ ..] => {
									super::push(out, 2225);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1260);
							match s {
								[] => true,
								['n', 'o', 'i', 't', 'i', 'l', 'a', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 2226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -v.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['k', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 526);
					s.is_empty()
				}
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 527);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 528);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 529);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						['l', 'u', 'v', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1263);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2227);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 2228);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1264);
							match s {
								[] => true,
								['l', 'p', 'x', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1265);
							match s {
								[] => true,
								['e', 'v', 'b', s @ ..] => {
									super::push(out, 2230);
									s.is_empty()
								}
								['u', 'k', 's', s @ ..] => {
									super::push(out, 2231);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', s @ ..] => {
									super::push(out, 2232);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['g', 'e', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n6150(s, out),
														['r', s @ ..] => super::t0::n6151(s, out),
														_ => false,
													}
												}
												['p', 'e', 'd', s @ ..] => {
													match s {
														[] => true,
														['-', s @ ..] => super::t0::n6152(s, out),
														['i', s @ ..] => super::t0::n6153(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['z', 'n', s @ ..] => {
											super::push(out, 3336);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'm', 'm', s @ ..] => {
									super::push(out, 2233);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => super::t1::n636(s, out),
				['z', 's', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 531);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -w.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', 'b', 'l', 'e', 'k', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1274);
							s.is_empty()
						}
						['n', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'g', 'l', 'e', 'k', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1276);
							s.is_empty()
						}
						['n', 'u', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'r', 't', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -x.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['f', 'o', 'r', 't', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1278);
							s.is_empty()
						}
						['p', 'm', 'o', 'k', 'r', 'e', 'p', 'y', 'h', '\u{1}', s @ ..] => {
							super::push(out, 1279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', 'n', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'd', 'o', 'h', 't', 'r', 'o', '-', 'h', 'c', 's', 'i', 'h', 'c', 'e', 'i', 'r', 'g', '\u{1}', s @ ..] => {
			super::push(out, 153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -y.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	s.is_empty()
}

/// Words ending -z.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		['n', 'e', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 156);
			match s {
				[] => true,
				['a', 'w', 'h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1280);
							s.is_empty()
						}
						['n', 'e', 'b', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1281);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1282);
							match s {
								[] => true,
								['t', 'e', 'j', '\u{1}', s @ ..] => {
									super::push(out, 2252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 533);
					match s {
						[] => true,
						['k', '\u{1}', s @ ..] => {
							super::push(out, 1283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['n', 'a', 'g', '\u{1}', s @ ..] => {
					super::push(out, 535);
					s.is_empty()
				}
				['r', 'u', 'k', '\u{1}', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['ü', 'n', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ß.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						['n', 'e', 't', 'o', 't', '\u{1}', s @ ..] => {
							super::push(out, 1285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', '\u{1}', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 539);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 159);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', '\u{1}', s @ ..] => {
							super::push(out, 1286);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 541);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 542);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2253);
									s.is_empty()
								}
								['n', 'e', 'h', s @ ..] => {
									super::push(out, 2254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', 'e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['ä', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['m', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', 'g', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						['m', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2255);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 2256);
									match s {
										[] => true,
										['a', 'r', 't', 'r', s @ ..] => {
											super::push(out, 3375);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'i', 'e', 'h', 'r', 'h', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 2257);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ü', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['r', 'e', 't', s @ ..] => {
					super::push(out, 544);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nd.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', 'w', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['i', 'e', 'l', '\u{1}', s @ ..] => {
					super::push(out, 556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n210(s, out),
		['i', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'ü', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1373);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1374);
							s.is_empty()
						}
						['f', 'f', 'i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1375);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 2374);
									s.is_empty()
								}
								['ü', s @ ..] => {
									super::push(out, 2375);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1376);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3522);
											s.is_empty()
										}
										['s', 'i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 3523);
											s.is_empty()
										}
										['ä', 'l', '\u{1}', s @ ..] => {
											super::push(out, 3524);
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
								['c', s @ ..] => {
									super::push(out, 2376);
									s.is_empty()
								}
								['n', 'e', 'r', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 2377);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2378);
									s.is_empty()
								}
								['m', 'i', s @ ..] => {
									super::push(out, 2379);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2380);
									s.is_empty()
								}
								['s', 'e', s @ ..] => {
									super::push(out, 2381);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 3525);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 189);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', 'u', 'a', 'z', 'e', s @ ..] => {
							super::push(out, 1377);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1378);
							match s {
								[] => true,
								['l', 'i', 'h', s @ ..] => {
									super::push(out, 2382);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['u', s @ ..] => {
											super::push(out, 3526);
											s.is_empty()
										}
										['ä', 'b', s @ ..] => {
											super::push(out, 3527);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2383);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2384);
									match s {
										[] => true,
										['o', s @ ..] => {
											super::push(out, 3528);
											s.is_empty()
										}
										['ö', s @ ..] => {
											super::push(out, 3529);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['f', 'ä', s @ ..] => {
							super::push(out, 1379);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 2385);
									s.is_empty()
								}
								['i', 'e', 't', 's', 'z', s @ ..] => {
									super::push(out, 2386);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 1380);
							match s {
								[] => true,
								['c', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 2387);
									s.is_empty()
								}
								['e', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 2388);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1381);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1382);
							match s {
								[] => true,
								['m', 'i', 'l', s @ ..] => {
									super::push(out, 2389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1383);
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 2390);
									match s {
										[] => true,
										['c', 'i', 'l', s @ ..] => {
											super::push(out, 3530);
											s.is_empty()
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 2391);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 2392);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 3531);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 3532);
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
								['a', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 3533);
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 4418);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 3534);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 2393);
									match s {
										[] => true,
										['f', '\u{1}', s @ ..] => {
											super::push(out, 3535);
											s.is_empty()
										}
										['n', 'r', 'e', '\u{1}', s @ ..] => {
											super::push(out, 3536);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n215(s, out),
		['ä', s @ ..] => {
			super::push(out, 192);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word klageweise.
pub(crate) fn n6184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4992);
	s.is_empty()
}

/// Words ending -rklageweise.
pub(crate) fn n6185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4993);
	s.is_empty()
}

/// Words ending -ig.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 2409);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 2410);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 3543);
											s.is_empty()
										}
										['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3544);
											s.is_empty()
										}
										['ß', 'i', 'm', '\u{1}', s @ ..] => {
											super::push(out, 3545);
											s.is_empty()
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
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 2411);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 613);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['i', 's', 'n', 'i', s @ ..] => {
							super::push(out, 1399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 615);
					s.is_empty()
				}
				['o', 'l', 'k', '\u{1}', s @ ..] => {
					super::push(out, 616);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['a', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2413);
									s.is_empty()
								}
								['l', 'e', s @ ..] => {
									super::push(out, 2414);
									s.is_empty()
								}
								['n', 'e', 'i', s @ ..] => {
									super::push(out, 2415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['a', 'r', s @ ..] => {
							super::push(out, 1400);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 1401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 1402);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 622);
					match s {
						[] => true,
						['u', 'h', 'c', 's', 't', 'i', s @ ..] => {
							super::push(out, 1403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2416);
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									super::push(out, 2417);
									s.is_empty()
								}
								['w', 'n', 'n', 'ü', 'd', '\u{1}', s @ ..] => {
									super::push(out, 2418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 1404);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1405);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1406);
							match s {
								[] => true,
								['k', 'h', 'c', 'a', 'r', s @ ..] => {
									super::push(out, 2420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ä', s @ ..] => {
							match s {
								[] => true,
								['b', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 2421);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 2422);
									s.is_empty()
								}
								['t', 's', s @ ..] => super::t1::n3067(s, out),
								['w', s @ ..] => {
									super::push(out, 2423);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 1407);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 2424);
									s.is_empty()
								}
								['f', s @ ..] => {
									super::push(out, 2425);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 2426);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3551);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 3552);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2427);
									match s {
										[] => true,
										['g', 'r', 'e', 'd', 'r', 'o', 'v', '\u{1}', s @ ..] => super::t1::n4447(s, out),
										_ => false,
									}
								}
								['s', s @ ..] => super::t1::n3073(s, out),
								['t', s @ ..] => super::t1::n3074(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => super::t1::n747(s, out),
				['u', s @ ..] => super::t1::n748(s, out),
				['ä', s @ ..] => super::t1::n749(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n255(s, out),
		['f', s @ ..] => super::t1::n256(s, out),
		['g', s @ ..] => super::t1::n257(s, out),
		['h', s @ ..] => super::t1::n258(s, out),
		['i', 'e', 'r', 'b', '\u{1}', s @ ..] => super::t1::n259(s, out),
		['k', s @ ..] => super::t1::n260(s, out),
		['l', s @ ..] => super::t1::n261(s, out),
		['m', s @ ..] => super::t1::n262(s, out),
		['n', s @ ..] => super::t1::n263(s, out),
		['p', s @ ..] => super::t1::n264(s, out),
		['r', s @ ..] => super::t1::n265(s, out),
		['s', s @ ..] => super::t1::n266(s, out),
		['t', s @ ..] => super::t1::n267(s, out),
		['u', 'e', 'r', '\u{1}', s @ ..] => super::t1::n268(s, out),
		['v', 'r', s @ ..] => super::t1::n269(s, out),
		['w', s @ ..] => super::t1::n270(s, out),
		['x', 'u', 'j', '\u{1}', s @ ..] => super::t1::n271(s, out),
		['z', s @ ..] => super::t1::n272(s, out),
		['ß', s @ ..] => super::t1::n273(s, out),
		['ö', 'b', '\u{1}', s @ ..] => super::t1::n274(s, out),
		_ => false,
	}
}

/// Words ending -ch.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 249);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1553);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2615);
									match s {
										[] => true,
										['e', 'y', 'b', 'a', 'b', '\u{1}', s @ ..] => {
											super::push(out, 3702);
											s.is_empty()
										}
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
					super::push(out, 734);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 735);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 736);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1554);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1555);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1556);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['r', 'f', 'z', 't', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t1::n286(s, out),
		['o', s @ ..] => {
			super::push(out, 252);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1572);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 1573);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1574);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1575);
							s.is_empty()
						}
						['m', 'u', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1576);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1577);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1578);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1579);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', 'i', s @ ..] => {
			super::push(out, 253);
			s.is_empty()
		}
		['s', s @ ..] => super::t1::n289(s, out),
		['u', s @ ..] => {
			super::push(out, 255);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sozial.
pub(crate) fn n3558(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'r', s @ ..] => {
			super::push(out, 4022);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 4023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kal.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1629);
					s.is_empty()
				}
				['l', 'e', 'g', s @ ..] => {
					super::push(out, 1630);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1631);
					s.is_empty()
				}
				['r', 'e', 'l', 'k', 'i', s @ ..] => {
					super::push(out, 1632);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					super::push(out, 1633);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mal.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 793);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['t', 'p', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1635);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1637);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 2806);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 4024);
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 4025);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 796);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nal.
pub(crate) fn n324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		['e', 'm', s @ ..] => {
			super::push(out, 798);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1638);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1639);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2807);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'o', 'd', s @ ..] => {
					super::push(out, 1640);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1641);
					s.is_empty()
				}
				['t', 's', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1642);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['g', 'i', 'p', s @ ..] => {
					super::push(out, 1643);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 2809);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2810);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5235);
													s.is_empty()
												}
												['h', s @ ..] => {
													super::push(out, 5236);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 5237);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 5238);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 4026);
									match s {
										[] => true,
										['s', 'o', 'p', 'o', s @ ..] => {
											super::push(out, 4816);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', 'n', s @ ..] => {
									super::push(out, 4027);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 4028);
									s.is_empty()
								}
								['r', 'o', 'p', 'o', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 4029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 1644);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dipal.
pub(crate) fn n325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	s.is_empty()
}

/// Words ending -ral.
pub(crate) fn n326(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['e', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1645);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1646);
					match s {
						[] => true,
						['i', 'l', 'o', s @ ..] => {
							super::push(out, 2811);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2812);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', 'a', 's', s @ ..] => {
			super::push(out, 803);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 804);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1649);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1651);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['v', '\u{1}', s @ ..] => {
									super::push(out, 4030);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 4031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				['u', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2813);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2814);
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
				['r', '\u{1}', s @ ..] => {
					super::push(out, 1653);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sal.
pub(crate) fn n327(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'b', s @ ..] => {
			super::push(out, 805);
			s.is_empty()
		}
		['r', 'e', 'v', 'i', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		['s', 'o', 'l', s @ ..] => {
			super::push(out, 807);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 808);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tal.
pub(crate) fn n328(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1655);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 2815);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2816);
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
				['b', s @ ..] => {
					super::push(out, 1656);
					match s {
						[] => true,
						['r', 'o', 'b', 'u', 's', '\u{1}', s @ ..] => {
							super::push(out, 2817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', s @ ..] => {
					super::push(out, 1657);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 1658);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', 'e', s @ ..] => {
			match s {
				[] => true,
				['d', 'n', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2818);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2819);
							s.is_empty()
						}
						['u', 'n', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 't', '\u{1}', s @ ..] => {
			super::push(out, 810);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 811);
			s.is_empty()
		}
		['u', 'r', s @ ..] => {
			super::push(out, 812);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ual.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['x', 'e', 's', '\u{1}', s @ ..] => {
			super::push(out, 813);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -val.
pub(crate) fn n330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	s.is_empty()
}

/// Words ending -yal.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	s.is_empty()
}

/// Words ending -el.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 284);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 814);
					match s {
						[] => true,
						['f', 'f', 'e', s @ ..] => {
							super::push(out, 1660);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2821);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 2822);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'o', s @ ..] => {
							super::push(out, 1661);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1662);
							match s {
								[] => true,
								['i', 'm', s @ ..] => {
									super::push(out, 2823);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', 'p', s @ ..] => {
							super::push(out, 1663);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'p', 'e', s @ ..] => {
									super::push(out, 2824);
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['d', 'n', 'o', 'p', '\u{1}', s @ ..] => {
											super::push(out, 4032);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 4033);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 2825);
									s.is_empty()
								}
								['u', 's', s @ ..] => {
									super::push(out, 2826);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 2827);
									s.is_empty()
								}
								['k', 'i', s @ ..] => {
									super::push(out, 2828);
									s.is_empty()
								}
								['p', 'e', 'z', 'k', 'a', 'n', 'u', s @ ..] => {
									super::push(out, 2829);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2830);
									s.is_empty()
								}
								['u', s @ ..] => {
									match s {
										[] => true,
										['k', 's', 'i', 'd', 'n', s @ ..] => {
											super::push(out, 4034);
											s.is_empty()
										}
										['m', 'r', s @ ..] => {
											super::push(out, 4035);
											s.is_empty()
										}
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
					super::push(out, 815);
					match s {
						[] => true,
						['d', 'e', 'r', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1665);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1666);
							s.is_empty()
						}
						['n', 'e', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 2831);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2832);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 2833);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1668);
							match s {
								[] => true,
								['a', 'p', 'm', 'o', 'k', 'r', s @ ..] => {
									super::push(out, 2834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 816);
					match s {
						[] => true,
						['n', 'g', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 817);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 818);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1671);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 285);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 819);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 820);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2835);
									s.is_empty()
								}
								['h', 'c', 's', 't', 'e', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 2836);
									s.is_empty()
								}
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
			super::push(out, 286);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 287);
			s.is_empty()
		}
		['g', '\u{1}', s @ ..] => {
			super::push(out, 288);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['n', 'u', 'd', 'k', s @ ..] => {
					super::push(out, 821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 292);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 822);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 823);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 824);
					s.is_empty()
				}
				['y', 'e', '\u{1}', s @ ..] => {
					super::push(out, 825);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 't', 'ü', 'l', '\u{1}', s @ ..] => {
			super::push(out, 293);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hl.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['a', 'f', 'h', s @ ..] => {
			super::push(out, 294);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -il.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['a', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 1676);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1677);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'm', 'o', s @ ..] => {
					super::push(out, 826);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 827);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1679);
							s.is_empty()
						}
						['e', 'r', s @ ..] => {
							super::push(out, 1680);
							s.is_empty()
						}
						['n', 'e', 'f', 'f', 'a', 'r', 'e', 'b', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1681);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['h', 'p', 'o', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					super::push(out, 828);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 829);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 830);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1682);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 2837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1683);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'i', s @ ..] => {
					super::push(out, 831);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', s @ ..] => {
							super::push(out, 1684);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', 'e', 'p', s @ ..] => {
			super::push(out, 296);
			s.is_empty()
		}
		['r', 'b', s @ ..] => {
			super::push(out, 297);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 298);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['k', 'e', s @ ..] => {
					super::push(out, 832);
					s.is_empty()
				}
				['n', 'a', 'k', s @ ..] => {
					super::push(out, 833);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 299);
			s.is_empty()
		}
		['z', 'e', s @ ..] => {
			super::push(out, 300);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ll.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 301);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 302);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 834);
					s.is_empty()
				}
				['g', '\u{1}', s @ ..] => {
					super::push(out, 835);
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					super::push(out, 836);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1686);
							s.is_empty()
						}
						['c', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1687);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1688);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1689);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1690);
							match s {
								[] => true,
								['e', 't', 'a', s @ ..] => {
									super::push(out, 2838);
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 4036);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4037);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['s', 's', 'e', '\u{1}', s @ ..] => {
													super::push(out, 4817);
													s.is_empty()
												}
												['t', 's', s @ ..] => {
													super::push(out, 4818);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2840);
									s.is_empty()
								}
								['i', 'f', s @ ..] => {
									match s {
										[] => true,
										['f', 'o', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4819);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 4820);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', 't', 'r', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4038);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', 't', s @ ..] => {
											super::push(out, 4039);
											s.is_empty()
										}
										['e', s @ ..] => {
											match s {
												[] => true,
												['s', s @ ..] => {
													super::push(out, 4821);
													s.is_empty()
												}
												['t', 's', s @ ..] => {
													super::push(out, 4822);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 2841);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 837);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2842);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 2844);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 2845);
									s.is_empty()
								}
								['m', 'i', s @ ..] => {
									super::push(out, 2846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'i', s @ ..] => {
							match s {
								[] => true,
								['s', 's', 'e', 'f', 'o', s @ ..] => {
									super::push(out, 2847);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 4040);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 4041);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 4042);
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 4043);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 4044);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1691);
							s.is_empty()
						}
						['u', 't', s @ ..] => {
							match s {
								[] => true,
								['k', 'u', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 2848);
									s.is_empty()
								}
								['l', 'u', 'k', '\u{1}', s @ ..] => {
									super::push(out, 2849);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', 'i', s @ ..] => {
							super::push(out, 1692);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1693);
							s.is_empty()
						}
						['s', 'n', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						['t', 'k', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4045);
											s.is_empty()
										}
										['h', s @ ..] => {
											super::push(out, 4046);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 2850);
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => super::t1::n2117(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 't', 's', s @ ..] => super::t1::n355(s, out),
		['o', s @ ..] => super::t1::n356(s, out),
		['u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 304);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ol.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 305);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 306);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ul.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['a', 'f', s @ ..] => {
			match s {
				[] => true,
				['k', 'n', 'i', s @ ..] => {
					super::push(out, 841);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 842);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ül.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	s.is_empty()
}

/// The word vorgabenwirksam.
pub(crate) fn n6001(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4823);
	s.is_empty()
}

/// The word unwirksam.
pub(crate) fn n6002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4824);
	s.is_empty()
}

/// Words ending -en.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1718);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1719);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2860);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 2861);
									match s {
										[] => true,
										['e', 't', 'n', s @ ..] => {
											super::push(out, 4050);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 1720);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 2862);
									match s {
										[] => true,
										['b', s @ ..] => {
											match s {
												[] => true,
												['e', 'g', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n6482(s, out),
														['a', 'd', '\u{1}', s @ ..] => super::t1::n6483(s, out),
														['k', 'c', 'ü', 'r', 'u', 'z', '\u{1}', s @ ..] => super::t1::n6484(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 4825);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['e', 'g', 'b', s @ ..] => {
											super::push(out, 4051);
											s.is_empty()
										}
										['h', 'c', 's', 'e', 'g', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4826);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 4827);
													match s {
														[] => true,
														['s', 'o', 'r', 'g', '\u{1}', s @ ..] => super::t1::n6485(s, out),
														_ => false,
													}
												}
												['t', 'u', s @ ..] => {
													super::push(out, 4828);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 4052);
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 4829);
													s.is_empty()
												}
												['h', s @ ..] => {
													super::push(out, 4830);
													s.is_empty()
												}
												['r', 'e', 'b', s @ ..] => {
													super::push(out, 4831);
													s.is_empty()
												}
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
							super::push(out, 1721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 876);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 2863);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 2864);
									s.is_empty()
								}
								['r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 2865);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', 'm', 'u', 'n', 'e', 'g', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1722);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['a', 'f', 'h', s @ ..] => {
							super::push(out, 1723);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 2866);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1724);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 2867);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 1725);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 2868);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 4053);
											s.is_empty()
										}
										['g', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 4832);
													s.is_empty()
												}
												['f', s @ ..] => {
													super::push(out, 4833);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 4834);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 2869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 880);
					match s {
						[] => true,
						['i', s @ ..] => super::t1::n2153(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['e', s @ ..] => super::t1::n2154(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t1::n1059(s, out),
				['n', s @ ..] => super::t1::n1060(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t1::n388(s, out),
		['g', s @ ..] => super::t1::n389(s, out),
		['h', s @ ..] => super::t1::n390(s, out),
		['i', 'n', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 332);
			s.is_empty()
		}
		['k', s @ ..] => super::t1::n392(s, out),
		['l', s @ ..] => super::t1::n393(s, out),
		['m', s @ ..] => super::t1::n394(s, out),
		['n', s @ ..] => super::t1::n395(s, out),
		['r', s @ ..] => super::t1::n396(s, out),
		['s', s @ ..] => super::t1::n397(s, out),
		['t', s @ ..] => super::t1::n398(s, out),
		['z', s @ ..] => super::t1::n399(s, out),
		['ß', s @ ..] => super::t1::n400(s, out),
		_ => false,
	}
}

/// Words ending -hern.
pub(crate) fn n1123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 936);
	match s {
		[] => true,
		['c', 'e', 'l', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1796);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eiern.
pub(crate) fn n1124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 937);
	s.is_empty()
}

/// Words ending -nern.
pub(crate) fn n1125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 938);
	s.is_empty()
}

/// Words ending -sern.
pub(crate) fn n1126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 939);
	match s {
		[] => true,
		['i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 1797);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tern.
pub(crate) fn n1127(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1798);
			match s {
				[] => true,
				['c', 'ü', 'n', 'k', 'c', 'o', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 2964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 1799);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 1800);
			match s {
				[] => true,
				['a', 'b', 'a', 'l', 'a', '\u{1}', s @ ..] => {
					super::push(out, 2965);
					s.is_empty()
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 4106);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 4107);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irn.
pub(crate) fn n413(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	s.is_empty()
}

/// Words ending -un.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['a', 'r', 'b', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 941);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 942);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', '\u{1}', s @ ..] => {
			super::push(out, 353);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yn.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'f', '\u{1}', s @ ..] => {
			super::push(out, 355);
			s.is_empty()
		}
		['g', 'o', 's', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -än.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['d', 'n', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 358);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ön.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	s.is_empty()
}

/// Words ending -ün.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 360);
			s.is_empty()
		}
		['r', 'g', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 943);
					s.is_empty()
				}
				['l', 'l', s @ ..] => {
					super::push(out, 944);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -bar.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'e', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1801);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				['l', '\u{1}', s @ ..] => {
					super::push(out, 1803);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1804);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 4108);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 4109);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2966);
							s.is_empty()
						}
						['e', 'w', 'n', s @ ..] => {
							super::push(out, 2967);
							s.is_empty()
						}
						['i', 'w', s @ ..] => {
							super::push(out, 2968);
							s.is_empty()
						}
						['u', 'w', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2969);
							s.is_empty()
						}
						['ä', s @ ..] => {
							super::push(out, 2970);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				['i', 'e', 'r', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2971);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 2972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							super::push(out, 2973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2974);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2975);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 4110);
									s.is_empty()
								}
								['r', 'd', 'h', 'c', 'r', 'u', 'd', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2976);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'w', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1809);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 1810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1811);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', s @ ..] => {
							super::push(out, 2978);
							s.is_empty()
						}
						['i', 'e', 'r', 'r', 'e', 'n', s @ ..] => {
							super::push(out, 2979);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 4112);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 4113);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4114);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 4115);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4116);
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
						['g', s @ ..] => {
							super::push(out, 2980);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 4117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'z', 'l', s @ ..] => {
							super::push(out, 2981);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 4118);
									s.is_empty()
								}
								['r', 'e', 'h', 'r', 'o', 'v', 'n', s @ ..] => {
									super::push(out, 4119);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 1812);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2982);
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4120);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 4121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 's', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1813);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 947);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', 'n', 'a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 2984);
							s.is_empty()
						}
						['i', 'p', 's', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2985);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2986);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 2987);
							match s {
								[] => true,
								['t', 'i', 'm', s @ ..] => {
									match s {
										[] => true,
										['n', 'u', 's', 'e', 'd', 'n', 'u', 'b', '\u{1}', s @ ..] => {
											super::push(out, 4884);
											s.is_empty()
										}
										['r', 'e', '\u{1}', s @ ..] => {
											super::push(out, 4885);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						['ä', s @ ..] => {
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 4122);
									match s {
										[] => true,
										['s', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4886);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 4123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ü', s @ ..] => {
							super::push(out, 2989);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1814);
					s.is_empty()
				}
				['l', 'e', 't', 's', 'r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2990);
							s.is_empty()
						}
						['e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2991);
							s.is_empty()
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
				['h', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', s @ ..] => super::t1::n3785(s, out),
						['ä', s @ ..] => super::t1::n3786(s, out),
						_ => false,
					}
				}
				['m', s @ ..] => super::t1::n2282(s, out),
				['r', 'o', 'f', s @ ..] => super::t1::n2283(s, out),
				_ => false,
			}
		}
		['n', s @ ..] => super::t1::n1141(s, out),
		['p', s @ ..] => super::t1::n1142(s, out),
		['r', s @ ..] => super::t1::n1143(s, out),
		['s', s @ ..] => super::t1::n1144(s, out),
		['t', s @ ..] => super::t1::n1145(s, out),
		['u', 'a', 'h', 'c', 's', s @ ..] => super::t1::n1146(s, out),
		['z', s @ ..] => super::t1::n1147(s, out),
		['ß', s @ ..] => super::t1::n1148(s, out),
		_ => false,
	}
}

/// Words ending -nnenklar.
pub(crate) fn n2314(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1834);
	s.is_empty()
}

/// Words ending -tklar.
pub(crate) fn n2315(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1835);
	s.is_empty()
}

/// Words ending -llar.
pub(crate) fn n1152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	s.is_empty()
}

/// Words ending -olar.
pub(crate) fn n1153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 955);
	s.is_empty()
}

/// Words ending -ular.
pub(crate) fn n1154(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1836);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['ä', s @ ..] => {
					super::push(out, 3021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mar.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	s.is_empty()
}

/// Words ending -nar.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	s.is_empty()
}

/// Words ending -par.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	s.is_empty()
}

/// Words ending -rar.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	s.is_empty()
}

/// Words ending -tar.
pub(crate) fn n435(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 956);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 957);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -er.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 370);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 958);
					s.is_empty()
				}
				['u', 'a', 's', 'z', s @ ..] => {
					super::push(out, 959);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 371);
			match s {
				[] => true,
				['e', 'i', 'b', s @ ..] => {
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 1838);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1839);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 960);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 961);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1840);
							s.is_empty()
						}
						['t', 'u', s @ ..] => {
							super::push(out, 1841);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 373);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1842);
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 1843);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1844);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 4146);
											s.is_empty()
										}
										['l', 'e', s @ ..] => {
											super::push(out, 4147);
											s.is_empty()
										}
										['n', 'e', s @ ..] => {
											match s {
												[] => true,
												['b', 'm', s @ ..] => {
													super::push(out, 4901);
													s.is_empty()
												}
												['h', s @ ..] => {
													super::push(out, 4902);
													s.is_empty()
												}
												_ => false,
											}
										}
										['s', 's', s @ ..] => {
											super::push(out, 4148);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1845);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 3022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1846);
							s.is_empty()
						}
						['o', 'l', s @ ..] => {
							super::push(out, 1847);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'i', 'd', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 969);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 971);
					s.is_empty()
				}
				['o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 972);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 973);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 1848);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['h', s @ ..] => {
									super::push(out, 3023);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3024);
									s.is_empty()
								}
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
			super::push(out, 381);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 975);
					s.is_empty()
				}
				['n', 'u', 'm', 'z', 't', 'u', 'p', '\u{1}', s @ ..] => {
					super::push(out, 976);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 977);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 978);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1850);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['i', 'f', 'k', 'c', 'o', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 3025);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1852);
							match s {
								[] => true,
								['d', 'n', 'e', 'p', 'p', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 3026);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 979);
					s.is_empty()
				}
				['u', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 981);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3027);
									s.is_empty()
								}
								['ß', s @ ..] => {
									super::push(out, 3028);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 982);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 1853);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3029);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3030);
									s.is_empty()
								}
								['d', 'n', 'ü', 's', '\u{1}', s @ ..] => {
									super::push(out, 3031);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['q', s @ ..] => {
					super::push(out, 983);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', 'e', 'l', 'c', '\u{1}', s @ ..] => {
			super::push(out, 384);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 385);
			match s {
				[] => true,
				['h', 'c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1855);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['g', 'l', 'o', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3032);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3033);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'g', 'n', 'u', 't', 'u', 'e', 'd', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 984);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hr.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', 'w', '\u{1}', s @ ..] => {
			super::push(out, 386);
			s.is_empty()
		}
		['e', 'h', '\u{1}', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 388);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ir.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	s.is_empty()
}

/// Words ending -or.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	s.is_empty()
}

/// Words ending -rr.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 389);
			s.is_empty()
		}
		['o', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 390);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					super::push(out, 985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ur.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['t', 'a', s @ ..] => {
			super::push(out, 392);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -är.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 122);
	match s {
		[] => true,
		['d', 'n', 'e', s @ ..] => {
			super::push(out, 393);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 394);
			s.is_empty()
		}
		['i', 'l', 'i', 'm', 'a', 'f', '\u{1}', s @ ..] => {
			super::push(out, 395);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 396);
			s.is_empty()
		}
		['l', 'u', s @ ..] => {
			match s {
				[] => true,
				['g', 'e', s @ ..] => {
					super::push(out, 986);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', 't', 'k', 'e', 'p', 's', '\u{1}', s @ ..] => {
							super::push(out, 1857);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1860);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 987);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 397);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						['g', 'i', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 3034);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											match s {
												[] => true,
												['e', 'r', 'k', '\u{1}', s @ ..] => {
													super::push(out, 4903);
													s.is_empty()
												}
												['l', 'f', 'n', 'i', '\u{1}', s @ ..] => {
													super::push(out, 4904);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 4905);
													s.is_empty()
												}
												_ => false,
											}
										}
										['k', 'a', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 4149);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['k', s @ ..] => {
													super::push(out, 4906);
													s.is_empty()
												}
												['l', 'o', 'v', 'e', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n6501(s, out),
														['r', '\u{1}', s @ ..] => super::t1::n6502(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1864);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', 'e', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 's', '\u{1}', s @ ..] => {
									super::push(out, 3035);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 3036);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1867);
							match s {
								[] => true,
								['o', 'i', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 3037);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 1868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'm', 'i', 'd', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -los.
pub(crate) fn n477(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', 'e', 'l', '\u{1}', s @ ..] => {
			super::push(out, 994);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 995);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 4150);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 3038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', 'e', 'i', 'r', 't', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1870);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				['i', 's', 'a', 't', 'n', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1872);
					s.is_empty()
				}
				['r', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			match s {
				[] => true,
				['a', 'l', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				['p', 'm', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['e', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1876);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', 'c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				['s', 'n', 'u', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1878);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3039);
							s.is_empty()
						}
						['r', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'e', 't', 'r', 'a', 'p', '\u{1}', s @ ..] => {
			super::push(out, 997);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['c', 'o', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['a', 'm', 'k', 'r', 'e', 'm', '\u{1}', s @ ..] => {
					super::push(out, 1880);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1881);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 3041);
							s.is_empty()
						}
						['g', 'ü', 'l', s @ ..] => {
							super::push(out, 3042);
							s.is_empty()
						}
						['i', 't', s @ ..] => {
							super::push(out, 3043);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 3044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'a', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				['i', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 1883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', 'b', 'o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 3045);
							s.is_empty()
						}
						['t', 's', 'y', 's', '\u{1}', s @ ..] => {
							super::push(out, 3046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'a', 'n', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1884);
					s.is_empty()
				}
				['m', 'i', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1885);
					s.is_empty()
				}
				['o', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 1886);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1887);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', 'n', 'a', 'h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 3047);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 3048);
							s.is_empty()
						}
						['e', 'd', 'i', '\u{1}', s @ ..] => {
							super::push(out, 3049);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 3050);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['r', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 4152);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 4153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 3051);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 4154);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 3052);
							s.is_empty()
						}
						['s', 's', 'i', 'w', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3053);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3054);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 's', '\u{1}', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				['o', 't', '\u{1}', s @ ..] => {
					super::push(out, 1890);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 3055);
							s.is_empty()
						}
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 3056);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'd', '\u{1}', s @ ..] => {
			super::push(out, 1000);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1001);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1891);
					match s {
						[] => true,
						['t', 'k', s @ ..] => {
							super::push(out, 3057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1892);
					s.is_empty()
				}
				['o', 'm', s @ ..] => {
					super::push(out, 1893);
					s.is_empty()
				}
				['u', 't', 'l', s @ ..] => {
					super::push(out, 1894);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['b', 'r', 'e', 'w', 'r', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1895);
					s.is_empty()
				}
				['d', 'n', 'a', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['m', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3058);
							s.is_empty()
						}
						['n', 'e', 'g', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3059);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'n', 'u', s @ ..] => {
					match s {
						[] => true,
						['b', 'i', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3060);
							s.is_empty()
						}
						['g', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 4155);
									s.is_empty()
								}
								['i', 't', 'f', 'ä', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4156);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 3061);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['h', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 4157);
									s.is_empty()
								}
								['n', 'i', 's', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 4158);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'h', 'ü', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 3062);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 3063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 'u', 'r', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 3064);
							s.is_empty()
						}
						['p', 's', 'r', 'e', 'd', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 3065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1896);
					s.is_empty()
				}
				['m', 'h', 'a', 'n', 's', 'u', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1897);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1898);
					match s {
						[] => true,
						['o', 'i', 't', 'o', s @ ..] => {
							super::push(out, 3066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', 'n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 3067);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 3068);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['f', 'a', 'h', 'c', 's', 'n', 'e', 'h', 'c', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						['h', 'c', s @ ..] => {
							match s {
								[] => true,
								['e', 'l', 'h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 4159);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 4160);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 3070);
							s.is_empty()
						}
						['l', 'a', 'h', 'n', 'i', '\u{1}', s @ ..] => super::t1::n3881(s, out),
						_ => false,
					}
				}
				['u', 'a', 'h', '\u{1}', s @ ..] => super::t1::n2395(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t1::n1217(s, out),
		['u', s @ ..] => super::t1::n1218(s, out),
		['v', 'i', 't', 'a', 'n', 'r', 'e', 't', 'l', 'a', '\u{1}', s @ ..] => super::t1::n1219(s, out),
		['x', 'i', 'f', 'f', 'a', '\u{1}', s @ ..] => super::t1::n1220(s, out),
		['z', s @ ..] => super::t1::n1221(s, out),
		['ß', 'u', 'r', 'g', '\u{1}', s @ ..] => super::t1::n1222(s, out),
		_ => false,
	}
}

/// Words ending -ross.
pub(crate) fn n1229(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1912);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3087);
					s.is_empty()
				}
				['n', 'e', 'h', 'c', 'ä', 'l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 3088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 1913);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gemäss.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1010);
			s.is_empty()
		}
		['h', 'c', 'a', 's', '\u{1}', s @ ..] => {
			super::push(out, 1011);
			s.is_empty()
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['g', 'n', 'u', s @ ..] => {
					match s {
						[] => true,
						['r', 'h', 'a', 'f', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3089);
							s.is_empty()
						}
						['t', 'r', 'a', 'w', 'r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 3090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'i', 'e', 'h', 'r', 'h', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1914);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ös.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['i', 'l', 'e', 'r', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 3091);
									s.is_empty()
								}
								['i', 'e', 'r', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3092);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					super::push(out, 1013);
					s.is_empty()
				}
				['z', 'i', 'd', s @ ..] => {
					super::push(out, 1014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 416);
			s.is_empty()
		}
		['l', 'u', 'd', s @ ..] => {
			super::push(out, 417);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 418);
			s.is_empty()
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', s @ ..] => {
					super::push(out, 1015);
					s.is_empty()
				}
				['e', 'v', s @ ..] => {
					super::push(out, 1016);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1915);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1916);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'i', s @ ..] => {
			super::push(out, 419);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['b', 'i', s @ ..] => {
					super::push(out, 1017);
					s.is_empty()
				}
				['o', 'm', s @ ..] => {
					super::push(out, 1018);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1019);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1020);
					s.is_empty()
				}
				['u', 'o', 'm', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1918);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1021);
					s.is_empty()
				}
				['n', 'a', s @ ..] => {
					super::push(out, 1022);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1023);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -et.
pub(crate) fn n146(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1035);
					match s {
						[] => true,
						['h', 'c', 's', 'e', 'b', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1036);
					match s {
						[] => true,
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1037);
					match s {
						[] => true,
						['e', 'l', 'k', 'e', 'g', s @ ..] => {
							super::push(out, 1931);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1038);
					match s {
						[] => true,
						['i', 'b', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3101);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3102);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1039);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1040);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1932);
							match s {
								[] => true,
								['w', 'e', 'g', 'u', s @ ..] => {
									super::push(out, 3103);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3104);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3105);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ü', 'r', 'g', 'e', 'b', s @ ..] => {
							super::push(out, 1933);
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3106);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1041);
					match s {
						[] => true,
						['h', 'ä', 'f', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3107);
									s.is_empty()
								}
								['s', 'n', s @ ..] => {
									super::push(out, 3108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 1934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 429);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 430);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['d', 'r', 'o', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1935);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 1936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'f', 'a', 'w', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1042);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1043);
					match s {
						[] => true,
						['i', 'e', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3109);
									s.is_empty()
								}
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1044);
					match s {
						[] => true,
						['c', 'i', 'e', 'z', 'e', 'g', s @ ..] => {
							super::push(out, 1937);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3111);
									s.is_empty()
								}
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
			super::push(out, 432);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1046);
					match s {
						[] => true,
						['a', 'h', 'c', 's', 't', 'r', 'i', 'w', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1938);
							s.is_empty()
						}
						['ä', 'r', 'k', 't', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 1939);
							s.is_empty()
						}
						['ü', 'l', 'k', 'r', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 1940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3112);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3113);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3114);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 3115);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 3116);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 4164);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 4165);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'e', 'l', s @ ..] => {
							super::push(out, 1941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', 'r', 'a', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3118);
									match s {
										[] => true,
										['g', s @ ..] => {
											match s {
												[] => true,
												['b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 4907);
													s.is_empty()
												}
												['s', 'u', 'a', '\u{1}', s @ ..] => {
													super::push(out, 4908);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 3119);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1945);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1048);
					s.is_empty()
				}
				['r', s @ ..] => super::t1::n1281(s, out),
				['s', s @ ..] => super::t1::n1282(s, out),
				['t', s @ ..] => super::t1::n1283(s, out),
				['u', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ö', s @ ..] => super::t1::n1285(s, out),
				['ü', 'h', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1052);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -aft.
pub(crate) fn n515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['h', s @ ..] => {
			match s {
				[] => true,
				['d', 'l', 'i', 'b', 'n', s @ ..] => {
					super::push(out, 1958);
					s.is_empty()
				}
				['k', 'c', 'ü', s @ ..] => {
					super::push(out, 1959);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', 't', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 4168);
									s.is_empty()
								}
								['f', 'i', 'e', 'w', 'z', 'n', s @ ..] => {
									super::push(out, 4169);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 4170);
									match s {
										[] => true,
										['n', 'o', '\u{1}', s @ ..] => {
											super::push(out, 4909);
											s.is_empty()
										}
										['r', 'i', 'z', '\u{1}', s @ ..] => {
											super::push(out, 4910);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 3126);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'm', s @ ..] => {
					super::push(out, 1960);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['b', 'o', s @ ..] => {
									super::push(out, 4171);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 4172);
									match s {
										[] => true,
										['l', 'i', 'r', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4911);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'e', 'h', s @ ..] => {
									super::push(out, 4173);
									s.is_empty()
								}
								['z', 't', s @ ..] => {
									super::push(out, 4174);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'o', s @ ..] => {
							super::push(out, 3127);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'l', 'h', 'e', 'f', 's', s @ ..] => {
							super::push(out, 3128);
							s.is_empty()
						}
						['h', 'a', 'w', s @ ..] => {
							super::push(out, 3129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 't', 'a', 't', 's', 'n', s @ ..] => {
					super::push(out, 1961);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 1962);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 1053);
			match s {
				[] => true,
				['t', 's', 'e', 'b', 'r', 'o', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3130);
							s.is_empty()
						}
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 3131);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pft.
pub(crate) fn n520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 3133);
							s.is_empty()
						}
						['r', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 3134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1965);
					s.is_empty()
				}
				['u', 't', 's', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1966);
					s.is_empty()
				}
				['ä', s @ ..] => {
					super::push(out, 1967);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', 'r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 1968);
					s.is_empty()
				}
				['k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1969);
					s.is_empty()
				}
				['z', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1970);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 1056);
			s.is_empty()
		}
		['ö', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['h', 'c', 's', 'r', s @ ..] => {
					super::push(out, 1971);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1972);
					match s {
						[] => true,
						['k', 'e', 'g', 'u', 'z', '\u{1}', s @ ..] => {
							super::push(out, 3135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uft.
pub(crate) fn n522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['a', 'k', 'e', s @ ..] => {
			super::push(out, 1060);
			s.is_empty()
		}
		['ä', 'h', 'e', 'g', '\u{1}', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -gt.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['g', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1062);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1063);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1064);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3136);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3137);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1066);
					s.is_empty()
				}
				['w', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1067);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1974);
							match s {
								[] => true,
								['g', 't', s @ ..] => {
									super::push(out, 3138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 1975);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1070);
					s.is_empty()
				}
				['w', 'e', 'b', 'f', s @ ..] => {
					super::push(out, 1071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1977);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3139);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 1978);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1979);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['u', 'r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						['ä', 'f', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1075);
					match s {
						[] => true,
						['i', 'e', 't', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1076);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['f', 'ä', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 1983);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 1984);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'e', 'b', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4912);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 4913);
													s.is_empty()
												}
												['s', s @ ..] => {
													match s {
														[] => true,
														['g', s @ ..] => super::t1::n6503(s, out),
														['h', s @ ..] => super::t1::n6504(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 4175);
											s.is_empty()
										}
										['ä', s @ ..] => {
											super::push(out, 4176);
											s.is_empty()
										}
										['ü', 'r', s @ ..] => {
											super::push(out, 4177);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3140);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3141);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'g', 'h', s @ ..] => {
									super::push(out, 3142);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3143);
									match s {
										[] => true,
										['ü', 'g', 'r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 4178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'ä', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 1985);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 1986);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ß', 'ä', 'm', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['e', 'b', s @ ..] => {
							super::push(out, 1987);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 446);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 447);
			match s {
				[] => true,
				['e', 'z', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1989);
							s.is_empty()
						}
						['r', 'e', 'b', s @ ..] => {
							super::push(out, 1990);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['r', 'p', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1083);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 1084);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1085);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1991);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ht.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1087);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3144);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 4179);
											s.is_empty()
										}
										['g', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4914);
													s.is_empty()
												}
												['h', 'c', 'r', 'u', 'd', '\u{1}', s @ ..] => {
													super::push(out, 4915);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 4916);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['h', s @ ..] => {
									super::push(out, 3145);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3146);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 1993);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1994);
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4180);
											s.is_empty()
										}
										['n', 'i', s @ ..] => {
											super::push(out, 4181);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'b', 'e', 'g', 'n', s @ ..] => {
							super::push(out, 1995);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 3147);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3148);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1996);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1088);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['h', 'c', 's', 't', 't', 'o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 3149);
									s.is_empty()
								}
								['i', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 3150);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['a', 'a', 'w', '\u{1}', s @ ..] => {
											super::push(out, 4182);
											s.is_empty()
										}
										['b', 'a', 't', 's', 'ß', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 4183);
											s.is_empty()
										}
										['d', s @ ..] => {
											match s {
												[] => true,
												['g', 'a', 'j', '\u{1}', s @ ..] => {
													super::push(out, 4917);
													s.is_empty()
												}
												['n', s @ ..] => {
													super::push(out, 4918);
													s.is_empty()
												}
												_ => false,
											}
										}
										['h', s @ ..] => {
											super::push(out, 4184);
											s.is_empty()
										}
										['l', 'e', 'g', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 4185);
											s.is_empty()
										}
										['m', 'r', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 4186);
											s.is_empty()
										}
										['n', 'i', 'm', 'r', 'e', 't', '\u{1}', s @ ..] => {
											super::push(out, 4187);
											s.is_empty()
										}
										['r', 'e', 'd', 'n', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4188);
											s.is_empty()
										}
										['s', 'b', 'a', 't', 's', 'ß', 'a', 'm', '\u{1}', s @ ..] => {
											super::push(out, 4189);
											s.is_empty()
										}
										['t', 's', 'i', 'r', 'f', '\u{1}', s @ ..] => {
											super::push(out, 4190);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'a', 'a', 'w', '\u{1}', s @ ..] => {
									super::push(out, 3151);
									s.is_empty()
								}
								['k', 'n', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 3152);
									s.is_empty()
								}
								['l', 'e', 'g', 'e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 3153);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 1998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t1::n1327(s, out),
				['o', 'k', 'e', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2003);
							s.is_empty()
						}
						['s', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => super::t1::n1329(s, out),
				['u', s @ ..] => super::t1::n1330(s, out),
				['ä', s @ ..] => super::t1::n1331(s, out),
				_ => false,
			}
		}
		['e', 'r', 'd', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
			super::push(out, 451);
			s.is_empty()
		}
		['i', 'e', 'w', 'e', 'g', 'n', 'i', 'e', '\u{1}', s @ ..] => {
			super::push(out, 452);
			s.is_empty()
		}
		['o', s @ ..] => super::t1::n534(s, out),
		['s', s @ ..] => super::t1::n535(s, out),
		['u', s @ ..] => super::t1::n536(s, out),
		['ä', s @ ..] => super::t1::n537(s, out),
		['ö', 'h', 'r', 'e', '\u{1}', s @ ..] => super::t1::n538(s, out),
		['ü', 'r', 'b', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => super::t1::n539(s, out),
		_ => false,
	}
}

/// Words ending -it.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1095);
					match s {
						[] => true,
						['e', 'n', 'e', 'b', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1096);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1097);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3174);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3175);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2017);
							s.is_empty()
						}
						['e', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3176);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 4206);
											s.is_empty()
										}
										['s', 'i', s @ ..] => {
											super::push(out, 4207);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'l', 'a', 'w', s @ ..] => {
									super::push(out, 3177);
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', 'e', 'b', s @ ..] => {
							match s {
								[] => true,
								['n', 'n', 'i', 's', '\u{1}', s @ ..] => {
									super::push(out, 3178);
									s.is_empty()
								}
								['ß', 'a', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 3179);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2018);
							s.is_empty()
						}
						['l', 'e', 'm', 'm', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 458);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 459);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -kt.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 460);
			s.is_empty()
		}
		['c', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['h', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2020);
							s.is_empty()
						}
						['k', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2021);
							s.is_empty()
						}
						['m', 'h', 'c', 's', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2022);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3180);
									s.is_empty()
								}
								['k', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3181);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3182);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'e', 'b', 'l', 'e', 'k', 's', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2023);
							s.is_empty()
						}
						['r', 't', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['b', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4919);
													s.is_empty()
												}
												['e', 'e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => {
													super::push(out, 4920);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', '\u{1}', s @ ..] => {
											super::push(out, 4208);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3183);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3184);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2025);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2026);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3185);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3186);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3187);
									match s {
										[] => true,
										['s', 'e', 'g', 's', 'u', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2027);
							s.is_empty()
						}
						['w', 'e', 'g', 'f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['l', 'k', 'e', 'g', 't', 's', 'i', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 2029);
							s.is_empty()
						}
						['n', 'k', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						['r', 't', 's', 'e', 'g', 't', 's', 'b', 'l', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['z', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1105);
					match s {
						[] => true,
						['f', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2033);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 2034);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['l', 'g', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3188);
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 4210);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 's', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3189);
									s.is_empty()
								}
								['ß', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 2036);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 3191);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 3192);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3193);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3194);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2037);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 2038);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 461);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['i', 'd', 'n', s @ ..] => {
							super::push(out, 2039);
							s.is_empty()
						}
						['r', 'o', 'k', 't', s @ ..] => {
							super::push(out, 2040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 462);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['a', 'k', s @ ..] => {
					super::push(out, 1107);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1108);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['m', 'h', 'c', 's', 'e', 'g', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						['t', 's', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2042);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', 'r', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['a', 'p', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1111);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lt.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1112);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1113);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						['s', 's', s @ ..] => {
							super::push(out, 2044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 467);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2045);
							match s {
								[] => true,
								['h', 'e', 'g', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4211);
											s.is_empty()
										}
										['n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 4212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'u', 'w', 'h', 'c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['i', 'd', 'n', 'a', 'k', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 2047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2048);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 2049);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3195);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3196);
									match s {
										[] => true,
										['e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4213);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 2050);
							s.is_empty()
						}
						['ü', 'l', 'k', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 1117);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 1118);
					match s {
						[] => true,
						['p', 's', 'r', 'e', 'b', 'ü', '\u{1}', s @ ..] => {
							super::push(out, 2052);
							s.is_empty()
						}
						['z', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 2054);
							match s {
								[] => true,
								['i', 'w', s @ ..] => {
									match s {
										[] => true,
										['r', 'e', 'v', '\u{1}', s @ ..] => {
											super::push(out, 4214);
											s.is_empty()
										}
										['t', 'n', 'e', s @ ..] => {
											match s {
												[] => true,
												['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
													super::push(out, 4921);
													s.is_empty()
												}
												['r', 'e', 't', 'n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 4922);
													s.is_empty()
												}
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
							match s {
								[] => true,
								['e', 'r', s @ ..] => {
									super::push(out, 3197);
									match s {
										[] => true,
										['p', 's', 'e', 'g', '\u{1}', s @ ..] => {
											super::push(out, 4215);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', 'f', 'e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3198);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1119);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 4216);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 4217);
											match s {
												[] => true,
												['e', 'g', '\u{1}', s @ ..] => {
													super::push(out, 4923);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', 'h', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 4218);
											s.is_empty()
										}
										['e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
											super::push(out, 4219);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ö', 'r', 'd', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'p', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 2056);
							s.is_empty()
						}
						['ö', s @ ..] => {
							super::push(out, 2057);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 's', s @ ..] => {
					super::push(out, 1120);
					match s {
						[] => true,
						['e', 'f', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2058);
							s.is_empty()
						}
						['ü', 'l', 'h', 'c', 's', s @ ..] => {
							match s {
								[] => true,
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3199);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3200);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['s', 'ä', s @ ..] => {
							super::push(out, 2059);
							s.is_empty()
						}
						['t', 'i', 'm', 'e', s @ ..] => {
							super::push(out, 2060);
							match s {
								[] => true,
								['b', 'r', 'e', 'd', 'n', 'i', 'm', '\u{1}', s @ ..] => {
									super::push(out, 3201);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['n', 'u', 'r', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2061);
							s.is_empty()
						}
						['t', 'u', 'h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2062);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['z', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3202);
									s.is_empty()
								}
								['r', 'e', '\u{1}', s @ ..] => {
									super::push(out, 3203);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 469);
			match s {
				[] => true,
				['e', s @ ..] => super::t1::n1373(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t1::n554(s, out),
		['o', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['h', 'e', s @ ..] => {
					super::push(out, 1131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => super::t1::n557(s, out),
		_ => false,
	}
}

/// Words ending -mt.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 474);
			match s {
				[] => true,
				['s', 'n', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['a', 'r', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				['ä', 'l', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1136);
					s.is_empty()
				}
				['ü', s @ ..] => {
					super::push(out, 1137);
					match s {
						[] => true,
						['r', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 476);
			s.is_empty()
		}
		['l', 'a', 'u', 'q', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 477);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 478);
			match s {
				[] => true,
				['a', 'd', 'r', 'e', 'v', 't', 't', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1138);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['l', 'k', 'e', s @ ..] => {
							super::push(out, 2076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 't', 's', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3214);
									s.is_empty()
								}
								['t', 's', 'b', 'l', 'e', 's', '\u{1}', s @ ..] => {
									super::push(out, 3215);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['o', 'f', s @ ..] => {
					super::push(out, 1141);
					match s {
						[] => true,
						['e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3216);
									s.is_empty()
								}
								['l', 'h', 'o', 'w', '\u{1}', s @ ..] => {
									super::push(out, 3217);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					match s {
						[] => true,
						['h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2078);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 'ä', 'r', s @ ..] => {
			match s {
				[] => true,
				['e', 'g', 'f', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 479);
			s.is_empty()
		}
		['ö', 'r', 't', 's', 'r', 'e', 'b', 'ü', 'ß', 'i', 'e', 'w', 'h', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 480);
			s.is_empty()
		}
		['ü', 'l', 'b', 'r', 'e', 'v', 'n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 481);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nt.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['d', 'r', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['g', 'e', 'l', 'e', 'h', s @ ..] => {
					super::push(out, 1145);
					s.is_empty()
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2082);
							s.is_empty()
						}
						['i', 'f', 'i', 'n', 'g', 'i', 's', 'n', s @ ..] => {
							super::push(out, 2083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['p', 'e', 'g', 'n', s @ ..] => {
							super::push(out, 2084);
							s.is_empty()
						}
						['u', 'b', s @ ..] => {
							super::push(out, 2085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				['r', 'e', 'l', 'o', 't', 'z', s @ ..] => {
					super::push(out, 1147);
					s.is_empty()
				}
				['s', 's', 'e', 'r', 'e', 't', 'n', 'i', 'h', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'm', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						['s', 'i', s @ ..] => {
							super::push(out, 2088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['v', 'e', 'l', 'e', 'r', s @ ..] => {
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 2089);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 2090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'o', 'r', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', 'r', 'e', 'h', 'c', 'i', 'e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 4221);
											s.is_empty()
										}
										['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
											super::push(out, 4222);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', 'z', 's', 'n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2091);
							s.is_empty()
						}
						['u', 'l', 'k', 'n', 'o', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', 'l', 'l', 'e', 't', 'n', 'i', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2093);
							s.is_empty()
						}
						['r', 'e', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 3219);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3220);
									s.is_empty()
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
						['a', 'v', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['b', '\u{1}', s @ ..] => {
											super::push(out, 4223);
											s.is_empty()
										}
										['u', 'q', 'ä', '\u{1}', s @ ..] => {
											super::push(out, 4224);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 3221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 3222);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 3224);
									s.is_empty()
								}
								['t', 's', 'b', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3225);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 2099);
							s.is_empty()
						}
						['r', 'u', 'k', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 2100);
							s.is_empty()
						}
						['ä', 'h', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3226);
									s.is_empty()
								}
								['o', 'k', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', 'b', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 2101);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 2102);
							s.is_empty()
						}
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['i', 'n', 'm', 'o', '\u{1}', s @ ..] => {
									super::push(out, 3228);
									s.is_empty()
								}
								['m', 'e', 'd', 'i', '\u{1}', s @ ..] => {
									super::push(out, 3229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', s @ ..] => super::t1::n5281(s, out),
										['n', s @ ..] => {
											super::push(out, 4225);
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 3230);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => super::t1::n1415(s, out),
				['z', s @ ..] => super::t1::n1416(s, out),
				_ => false,
			}
		}
		['h', s @ ..] => super::t1::n570(s, out),
		['i', s @ ..] => super::t1::n571(s, out),
		['n', s @ ..] => super::t1::n572(s, out),
		['o', 't', 'e', 'b', 'n', 'u', '\u{1}', s @ ..] => super::t1::n573(s, out),
		['r', s @ ..] => super::t1::n574(s, out),
		['u', s @ ..] => super::t1::n575(s, out),
		['ö', 'r', 'k', 'e', 'g', 'e', 'e', 'n', 'h', 'c', 's', '\u{1}', s @ ..] => super::t1::n576(s, out),
		_ => false,
	}
}

/// Words ending -ot.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['r', 'u', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 1171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pt.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['p', 'i', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['k', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rt.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t1::n582(s, out),
		['h', s @ ..] => {
			super::push(out, 495);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 2158);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2159);
							match s {
								[] => true,
								['e', 'g', '\u{1}', s @ ..] => {
									super::push(out, 3303);
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
						['e', s @ ..] => {
							super::push(out, 2160);
							match s {
								[] => true,
								['g', '\u{1}', s @ ..] => {
									super::push(out, 3304);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 2161);
							s.is_empty()
						}
						['k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2162);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2163);
							s.is_empty()
						}
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2164);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2165);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'e', s @ ..] => {
							super::push(out, 2166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['b', 'r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2167);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 1196);
					match s {
						[] => true,
						['n', 'r', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2169);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					super::push(out, 1197);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2171);
							match s {
								[] => true,
								['e', 'g', 'n', 'a', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3306);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['c', 'i', 'l', 'p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1198);
					s.is_empty()
				}
				['s', 's', 'e', 'r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 1199);
					s.is_empty()
				}
				['v', 'i', 't', 'l', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 1200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1201);
					match s {
						[] => true,
						['p', 's', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3307);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 3308);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2172);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ä', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['l', 'k', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 'f', 'u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2173);
							s.is_empty()
						}
						['r', 'e', '\u{1}', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ö', s @ ..] => {
			super::push(out, 498);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['e', 'g', 's', 's', s @ ..] => {
							super::push(out, 2175);
							s.is_empty()
						}
						['r', 'e', 'z', 's', s @ ..] => {
							super::push(out, 2176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -st.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 144);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1203);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 1204);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', 'e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1206);
					s.is_empty()
				}
				['r', 'k', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1207);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 500);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1208);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['b', 'e', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2177);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 2178);
							s.is_empty()
						}
						['f', 'f', 'i', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						['g', 'n', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 2180);
							s.is_empty()
						}
						['h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2181);
							s.is_empty()
						}
						['l', 'e', 'g', 'a', 'n', '\u{1}', s @ ..] => {
							super::push(out, 2182);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'w', 'h', 'c', 'o', 'h', '\u{1}', s @ ..] => {
											super::push(out, 4296);
											s.is_empty()
										}
										['u', 't', 's', '\u{1}', s @ ..] => {
											super::push(out, 4297);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', 'b', '\u{1}', s @ ..] => {
									super::push(out, 3309);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['n', 'i', 'h', 'c', 's', 'a', 'm', 'l', 'ü', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 3310);
									s.is_empty()
								}
								['p', 'o', 'r', 't', '\u{1}', s @ ..] => {
									super::push(out, 3311);
									s.is_empty()
								}
								['s', 'l', 'e', 'f', '\u{1}', s @ ..] => {
									super::push(out, 3312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['m', 'm', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 3313);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 3314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'g', 'l', 'l', 'o', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3315);
									s.is_empty()
								}
								['g', 'n', 'u', 'r', 'e', 't', 't', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 3316);
									s.is_empty()
								}
								['n', 'o', 'i', 's', 'o', 'r', 'r', 'o', 'k', '\u{1}', s @ ..] => {
									super::push(out, 3317);
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 4298);
											s.is_empty()
										}
										['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
											super::push(out, 4299);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 't', 'i', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 2183);
							s.is_empty()
						}
						['ß', s @ ..] => {
							match s {
								[] => true,
								['i', 'e', 'l', 'h', 'c', 's', 'r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3318);
									s.is_empty()
								}
								['u', 'h', 'c', 's', '\u{1}', s @ ..] => {
									super::push(out, 3319);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2184);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['i', 'z', 'n', 'i', 'e', '\u{1}', s @ ..] => {
					super::push(out, 1211);
					s.is_empty()
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 2186);
							s.is_empty()
						}
						['ü', s @ ..] => {
							super::push(out, 2187);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 2188);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2189);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['n', 'r', 'e', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1212);
					s.is_empty()
				}
				['r', 'e', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1213);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 1214);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1215);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1216);
					s.is_empty()
				}
				['r', 't', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1217);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'i', 'w', 'h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 505);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 506);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 3320);
									s.is_empty()
								}
								['m', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3321);
									s.is_empty()
								}
								['r', 'e', 'v', '\u{1}', s @ ..] => {
									super::push(out, 3322);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 2191);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 2192);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 2193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['r', 't', 's', s @ ..] => {
							super::push(out, 2194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 2195);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 2196);
							s.is_empty()
						}
						['w', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3323);
									match s {
										[] => true,
										['n', 'u', s @ ..] => {
											super::push(out, 4300);
											s.is_empty()
										}
										['r', 'e', s @ ..] => {
											super::push(out, 4301);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['e', s @ ..] => {
													super::push(out, 4958);
													s.is_empty()
												}
												['t', 'ä', s @ ..] => {
													super::push(out, 4959);
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 's', s @ ..] => {
											super::push(out, 4302);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 3324);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t1::n598(s, out),
		['ö', 'l', 'e', 'g', s @ ..] => super::t1::n599(s, out),
		['ü', 'w', '\u{1}', s @ ..] => super::t1::n600(s, out),
		_ => false,
	}
}

/// Words ending -tt.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3325);
									s.is_empty()
								}
								['l', 'a', 'a', '\u{1}', s @ ..] => {
									super::push(out, 3326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 2197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2198);
							s.is_empty()
						}
						['n', 'e', 'd', 'i', 'e', 's', '\u{1}', s @ ..] => {
							super::push(out, 2199);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 1222);
					s.is_empty()
				}
				['w', 'h', 'c', 's', '\u{1}', s @ ..] => {
					super::push(out, 1223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 1224);
					s.is_empty()
				}
				['k', 'o', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1225);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 1226);
					match s {
						[] => true,
						['o', 'i', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2200);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2201);
							s.is_empty()
						}
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 2202);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'd', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 's', '\u{1}', s @ ..] => {
			super::push(out, 509);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['g', 'i', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1228);
					s.is_empty()
				}
				['l', 'f', '\u{1}', s @ ..] => {
					super::push(out, 1229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1230);
					s.is_empty()
				}
				['p', 'a', 'k', '\u{1}', s @ ..] => {
					super::push(out, 1231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ü', 'l', '\u{1}', s @ ..] => {
			super::push(out, 510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ut.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 1232);
					match s {
						[] => true,
						['e', 'g', 'n', s @ ..] => {
							super::push(out, 2203);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1233);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2204);
							s.is_empty()
						}
						['n', 'i', 'e', 'l', 'k', '\u{1}', s @ ..] => {
							super::push(out, 2205);
							s.is_empty()
						}
						['r', 'o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 2206);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 't', s @ ..] => {
					super::push(out, 1234);
					s.is_empty()
				}
				['s', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1236);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 3327);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 3328);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'r', 'e', 'z', '\u{1}', s @ ..] => {
							super::push(out, 2207);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1237);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1238);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'a', '\u{1}', s @ ..] => {
			super::push(out, 512);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 513);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 514);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xt.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['e', 'h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 515);
			s.is_empty()
		}
		['i', 'l', 'f', 'r', 'e', 'v', '\u{1}', s @ ..] => {
			super::push(out, 516);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zt.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 517);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 2208);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 2209);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1239);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'e', 't', s @ ..] => {
			super::push(out, 518);
			match s {
				[] => true,
				['s', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1240);
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
						['r', 'e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2210);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 2211);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'g', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1241);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['ü', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', 'e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 2212);
							s.is_empty()
						}
						['k', 'e', 'g', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 2213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1242);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', 'g', s @ ..] => {
									match s {
										[] => true,
										['f', s @ ..] => {
											super::push(out, 4303);
											s.is_empty()
										}
										['n', 'e', 'g', s @ ..] => {
											super::push(out, 4304);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 't', s @ ..] => {
									super::push(out, 3329);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['h', 'r', 'e', 'b', s @ ..] => {
							super::push(out, 2214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 2215);
							s.is_empty()
						}
						['n', 'e', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 3330);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 4305);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', 'b', s @ ..] => {
									super::push(out, 3331);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ä', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['e', 'g', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 2216);
							s.is_empty()
						}
						['h', 'c', 's', 'e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 2217);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ü', s @ ..] => {
					match s {
						[] => true,
						['h', 'c', 's', 'e', 'g', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3332);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 3333);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 2218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 520);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ßt.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['f', 'e', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1246);
					s.is_empty()
				}
				['h', 'r', 'e', 'v', '\u{1}', s @ ..] => {
					super::push(out, 1247);
					s.is_empty()
				}
				['p', 'e', 'g', 'n', 'a', '\u{1}', s @ ..] => {
					super::push(out, 1248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 1249);
					s.is_empty()
				}
				['w', 'e', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 2219);
							s.is_empty()
						}
						['g', '\u{1}', s @ ..] => {
							super::push(out, 2220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ät.
pub(crate) fn n165(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 521);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 522);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word egressiv.
pub(crate) fn n6150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4960);
	s.is_empty()
}

/// Words ending -regressiv.
pub(crate) fn n6151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4961);
	s.is_empty()
}

/// Words ending --depressiv.
pub(crate) fn n6152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4962);
	s.is_empty()
}

/// Words ending -idepressiv.
pub(crate) fn n6153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4963);
	s.is_empty()
}

