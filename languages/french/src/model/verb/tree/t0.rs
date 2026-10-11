use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['e', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 3);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 8);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 9);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 53);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 216);
											match s {
												[] => true,
												['c', s @ ..] => {
													super::push(out, 392);
													s.is_empty()
												}
												['f', s @ ..] => {
													super::push(out, 393);
													s.is_empty()
												}
												['p', s @ ..] => {
													super::push(out, 394);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 395);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n583(s, out),
														['p', s @ ..] => super::t0::n584(s, out),
														_ => false,
													}
												}
												['t', s @ ..] => {
													super::push(out, 396);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 217);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 218);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => super::t0::n54(s, out),
								['u', s @ ..] => super::t0::n55(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t0::n10(s, out),
						['o', s @ ..] => {
							super::push(out, 11);
							s.is_empty()
						}
						['p', s @ ..] => super::t0::n12(s, out),
						['t', s @ ..] => super::t0::n13(s, out),
						['u', s @ ..] => super::t0::n14(s, out),
						['v', s @ ..] => super::t0::n15(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => super::t0::n2(s, out),
		_ => false,
	}
}

/// The word rendre.
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 539);
	s.is_empty()
}

/// Words ending -prendre.
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 540);
	s.is_empty()
}

/// Words ending -rdre.
pub(crate) fn n54(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 54);
	match s {
		[] => true,
		['e', 'p', 'é', '\u{1}', s @ ..] => {
			super::push(out, 219);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -udre.
pub(crate) fn n55(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 55);
	match s {
		[] => true,
		['o', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 398);
					match s {
						[] => true,
						['r', 'e', 'v', '\u{1}', s @ ..] => {
							super::push(out, 541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 399);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 542);
							s.is_empty()
						}
						['é', s @ ..] => {
							super::push(out, 543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ire.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 220);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 221);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 222);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 223);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 57);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 224);
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 225);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 226);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 227);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 59);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 228);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 229);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 60);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 230);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 62);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 232);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 233);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 234);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 235);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 63);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 236);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 237);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 238);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 239);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 400);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 544);
									s.is_empty()
								}
								['é', 'd', 'o', s @ ..] => {
									super::push(out, 545);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pre.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	s.is_empty()
}

/// Words ending -tre.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 64);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 65);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 240);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 401);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 546);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 547);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 548);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 402);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 403);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 241);
					match s {
						[] => true,
						['r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 404);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 66);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 242);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 243);
					match s {
						[] => true,
						['m', 'e', 'r', 't', s @ ..] => {
							super::push(out, 405);
							match s {
								[] => true,
								['n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 549);
									s.is_empty()
								}
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
			super::push(out, 68);
			s.is_empty()
		}
		['ê', s @ ..] => {
			super::push(out, 69);
			s.is_empty()
		}
		['î', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 244);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 406);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 550);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 551);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 552);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 407);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 408);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 245);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 409);
							s.is_empty()
						}
						['r', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 553);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 554);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ure.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['l', 'c', s @ ..] => {
			match s {
				[] => true,
				['n', 'o', s @ ..] => {
					super::push(out, 246);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 247);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vre.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 248);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 410);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 249);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -r.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['e', s @ ..] => super::t0::n4(s, out),
		['i', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 36);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 37);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 38);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 39);
					match s {
						[] => true,
						['a', 's', s @ ..] => {
							super::push(out, 179);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 40);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 41);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 180);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 181);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 509);
											match s {
												[] => true,
												['f', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n691(s, out),
														['é', s @ ..] => super::t0::n692(s, out),
														_ => false,
													}
												}
												['j', s @ ..] => {
													super::push(out, 614);
													s.is_empty()
												}
												['s', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n693(s, out),
														['s', s @ ..] => super::t0::n694(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['e', s @ ..] => {
											super::push(out, 510);
											match s {
												[] => true,
												['i', 'v', '\u{1}', s @ ..] => {
													super::push(out, 615);
													s.is_empty()
												}
												['u', s @ ..] => {
													match s {
														[] => true,
														['c', s @ ..] => super::t0::n695(s, out),
														['g', 'r', 'o', 'n', 'e', '\u{1}', s @ ..] => super::t0::n696(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 511);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 182);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 183);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 42);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 184);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 356);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => super::t0::n43(s, out),
				['o', s @ ..] => super::t0::n44(s, out),
				['p', s @ ..] => {
					super::push(out, 45);
					s.is_empty()
				}
				['r', s @ ..] => super::t0::n46(s, out),
				['s', s @ ..] => super::t0::n47(s, out),
				['t', s @ ..] => super::t0::n48(s, out),
				['u', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 211);
							s.is_empty()
						}
						['o', s @ ..] => super::t0::n216(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t0::n50(s, out),
				_ => false,
			}
		}
		['ë', s @ ..] => {
			super::push(out, 6);
			s.is_empty()
		}
		['ï', s @ ..] => super::t0::n7(s, out),
		_ => false,
	}
}

/// Words ending -er.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['r', 'u', s @ ..] => {
					super::push(out, 71);
					match s {
						[] => true,
						['o', 'b', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 250);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 72);
					match s {
						[] => true,
						['m', 'a', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 251);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 252);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 73);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 74);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							super::push(out, 253);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 75);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 18);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 76);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 254);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 255);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 77);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 78);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 256);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 413);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', 'i', s @ ..] => {
							super::push(out, 257);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									super::push(out, 414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 79);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 80);
					match s {
						[] => true,
						['m', 'é', s @ ..] => {
							super::push(out, 258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 81);
					match s {
						[] => true,
						['e', 'm', 'é', s @ ..] => {
							super::push(out, 259);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									super::push(out, 415);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 82);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 83);
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 19);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 84);
					s.is_empty()
				}
				['f', 'a', s @ ..] => {
					super::push(out, 85);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 260);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 20);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 86);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 87);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 88);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 261);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									super::push(out, 416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 89);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 90);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 91);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 92);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 93);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 21);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 94);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 262);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 417);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 555);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 556);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 263);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 264);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 265);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 266);
							match s {
								[] => true,
								['a', 'g', '\u{1}', s @ ..] => {
									super::push(out, 418);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 267);
							match s {
								[] => true,
								['o', 'c', s @ ..] => {
									super::push(out, 419);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 557);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['é', s @ ..] => {
							super::push(out, 268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 22);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 269);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 270);
							s.is_empty()
						}
						['é', 'm', s @ ..] => {
							super::push(out, 271);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 420);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'é', s @ ..] => {
					super::push(out, 95);
					match s {
						[] => true,
						['g', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 96);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 273);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 274);
							match s {
								[] => true,
								['é', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 558);
											s.is_empty()
										}
										['r', '\u{1}', s @ ..] => {
											super::push(out, 559);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', 'o', 'r', 'p', 'p', s @ ..] => {
							super::push(out, 275);
							match s {
								[] => true,
								['a', 'é', 'r', '\u{1}', s @ ..] => {
									super::push(out, 421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 97);
					match s {
						[] => true,
						['a', 't', 'x', s @ ..] => {
							super::push(out, 276);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 23);
			s.is_empty()
		}
		['l', s @ ..] => super::t0::n24(s, out),
		['m', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 114);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 115);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 116);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 117);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 118);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 119);
					match s {
						[] => true,
						['l', 'p', s @ ..] => {
							super::push(out, 292);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 455);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 293);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 456);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'u', s @ ..] => {
							super::push(out, 294);
							match s {
								[] => true,
								['o', 'c', 'c', 'a', 's', s @ ..] => {
									super::push(out, 457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n26(s, out),
		['p', s @ ..] => super::t0::n27(s, out),
		['r', s @ ..] => super::t0::n28(s, out),
		['s', s @ ..] => super::t0::n29(s, out),
		['t', s @ ..] => super::t0::n30(s, out),
		['u', s @ ..] => super::t0::n31(s, out),
		['v', s @ ..] => super::t0::n32(s, out),
		['x', s @ ..] => {
			super::push(out, 33);
			s.is_empty()
		}
		['y', s @ ..] => super::t0::n34(s, out),
		['é', s @ ..] => super::t0::n35(s, out),
		_ => false,
	}
}

/// The word faillir.
pub(crate) fn n691(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 638);
	s.is_empty()
}

/// Words ending -éfaillir.
pub(crate) fn n692(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 639);
	s.is_empty()
}

/// The word saillir.
pub(crate) fn n693(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 640);
	s.is_empty()
}

/// Words ending -ssaillir.
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 641);
	s.is_empty()
}

/// Words ending -cueillir.
pub(crate) fn n695(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 642);
	s.is_empty()
}

/// The word enorgueillir.
pub(crate) fn n696(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 643);
	s.is_empty()
}

/// Words ending -nir.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					super::push(out, 358);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							super::push(out, 512);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 513);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 617);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 186);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 187);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 188);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 189);
			s.is_empty()
		}
		['ü', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['é', 'r', '\u{1}', s @ ..] => {
					super::push(out, 359);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oir.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 515);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 516);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 517);
							match s {
								[] => true,
								['e', 'm', '\u{1}', s @ ..] => {
									super::push(out, 618);
									s.is_empty()
								}
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
			super::push(out, 192);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 360);
					match s {
						[] => true,
						['v', 'é', s @ ..] => {
							super::push(out, 518);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 362);
					match s {
						[] => true,
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 194);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 195);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 363);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 521);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 619);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 620);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 523);
							s.is_empty()
						}
						['t', 'n', 'e', 'm', 'a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 366);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 367);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 525);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 526);
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 621);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 644);
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 622);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 368);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rir.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 196);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 198);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 199);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 200);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['a', 's', '\u{1}', s @ ..] => {
					super::push(out, 369);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 370);
					s.is_empty()
				}
				['h', 'a', '\u{1}', s @ ..] => {
					super::push(out, 371);
					s.is_empty()
				}
				['m', '\u{1}', s @ ..] => {
					super::push(out, 372);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 373);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 527);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 202);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 529);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 203);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					super::push(out, 374);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 375);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 376);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 377);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 531);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 532);
							match s {
								[] => true,
								['n', 'e', s @ ..] => {
									super::push(out, 623);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sir.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 378);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', '\u{1}', s @ ..] => {
					super::push(out, 379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 380);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tir.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 381);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 382);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 534);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 535);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 536);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 537);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 383);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 624);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 625);
									s.is_empty()
								}
								['é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 626);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 384);
					s.is_empty()
				}
				['o', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 538);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 627);
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 628);
									s.is_empty()
								}
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
			super::push(out, 207);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 208);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 209);
			s.is_empty()
		}
		['ê', s @ ..] => {
			super::push(out, 210);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 385);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 386);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ouir.
pub(crate) fn n216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vir.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 214);
			match s {
				[] => true,
				['e', 's', 's', 'a', s @ ..] => {
					super::push(out, 388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'a', 'h', 'c', '\u{1}', s @ ..] => {
			super::push(out, 215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ïr.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 51);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 389);
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 390);
							s.is_empty()
						}
						['é', 'r', '\u{1}', s @ ..] => {
							super::push(out, 391);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ler.
pub(crate) fn n24(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 24);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 98);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 99);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 100);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['a', 'g', '\u{1}', s @ ..] => {
							super::push(out, 422);
							s.is_empty()
						}
						['r', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 423);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['i', 'f', '\u{1}', s @ ..] => {
							super::push(out, 424);
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', 'c', '\u{1}', s @ ..] => {
									super::push(out, 560);
									s.is_empty()
								}
								['i', 't', 'é', '\u{1}', s @ ..] => {
									super::push(out, 561);
									s.is_empty()
								}
								['o', 'm', 'a', '\u{1}', s @ ..] => {
									super::push(out, 562);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', 'h', '\u{1}', s @ ..] => {
									super::push(out, 563);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['m', '\u{1}', s @ ..] => {
											super::push(out, 629);
											s.is_empty()
										}
										['s', s @ ..] => {
											match s {
												[] => true,
												['n', 'e', 's', 'é', 'd', '\u{1}', s @ ..] => {
													super::push(out, 645);
													s.is_empty()
												}
												['é', 'd', '\u{1}', s @ ..] => {
													super::push(out, 646);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'p', 'é', 'd', '\u{1}', s @ ..] => {
							super::push(out, 425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'o', 'm', '\u{1}', s @ ..] => {
					super::push(out, 277);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 278);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', 's', 's', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 426);
							s.is_empty()
						}
						['m', 'o', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 427);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['j', '\u{1}', s @ ..] => {
									super::push(out, 564);
									s.is_empty()
								}
								['r', 'g', '\u{1}', s @ ..] => {
									super::push(out, 565);
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
						['g', 'a', '\u{1}', s @ ..] => {
							super::push(out, 428);
							s.is_empty()
						}
						['n', 'a', '\u{1}', s @ ..] => {
							super::push(out, 429);
							s.is_empty()
						}
						['è', 'r', 'c', '\u{1}', s @ ..] => {
							super::push(out, 430);
							s.is_empty()
						}
						['é', 'r', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 566);
									s.is_empty()
								}
								['é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 567);
									s.is_empty()
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
						['p', s @ ..] => {
							super::push(out, 431);
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							super::push(out, 432);
							s.is_empty()
						}
						['é', '\u{1}', s @ ..] => {
							super::push(out, 433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'r', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 279);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['s', 'o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 434);
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 568);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 630);
											s.is_empty()
										}
										['m', 'e', '\u{1}', s @ ..] => {
											super::push(out, 631);
											s.is_empty()
										}
										['é', 'd', '\u{1}', s @ ..] => {
											super::push(out, 632);
											s.is_empty()
										}
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
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 435);
							s.is_empty()
						}
						['n', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 436);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 437);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 569);
									s.is_empty()
								}
								['o', 'b', '\u{1}', s @ ..] => {
									super::push(out, 570);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', 'd', '\u{1}', s @ ..] => {
							super::push(out, 438);
							s.is_empty()
						}
						['ô', 'c', '\u{1}', s @ ..] => {
							super::push(out, 439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'q', 'a', 'r', 'c', '\u{1}', s @ ..] => {
					super::push(out, 280);
					s.is_empty()
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['a', 't', '\u{1}', s @ ..] => {
							super::push(out, 440);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 441);
							s.is_empty()
						}
						['r', 'e', 'c', 'é', 'd', '\u{1}', s @ ..] => {
							super::push(out, 442);
							s.is_empty()
						}
						['u', 'o', 'n', 'e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 443);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 102);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 281);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 283);
					s.is_empty()
				}
				['o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 105);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 285);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 444);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 286);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 445);
							match s {
								[] => true,
								['e', 'r', '\u{1}', s @ ..] => {
									super::push(out, 571);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 446);
							match s {
								[] => true,
								['e', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 572);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 's', 'i', s @ ..] => {
							super::push(out, 447);
							match s {
								[] => true,
								['u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 573);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 287);
					match s {
						[] => true,
						['a', 'm', s @ ..] => super::t0::n483(s, out),
						['s', s @ ..] => super::t0::n484(s, out),
						['v', s @ ..] => super::t0::n485(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n109(s, out),
		['p', s @ ..] => super::t0::n110(s, out),
		['r', s @ ..] => super::t0::n111(s, out),
		['s', s @ ..] => super::t0::n112(s, out),
		['u', s @ ..] => super::t0::n113(s, out),
		['y', s @ ..] => super::t0::n114(s, out),
		['é', s @ ..] => super::t0::n115(s, out),
		['ê', s @ ..] => super::t0::n116(s, out),
		_ => false,
	}
}

/// Words ending -ner.
pub(crate) fn n26(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 26);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 295);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 122);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 123);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['a', 'l', s @ ..] => {
							super::push(out, 458);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'l', 's', s @ ..] => {
							super::push(out, 459);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 580);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 124);
			match s {
				[] => true,
				['h', 'c', 'a', 'r', s @ ..] => {
					super::push(out, 298);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					super::push(out, 299);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 461);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 300);
					match s {
						[] => true,
						['q', 'o', s @ ..] => {
							super::push(out, 462);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 463);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 125);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', s @ ..] => {
							super::push(out, 465);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 466);
							match s {
								[] => true,
								['c', 'i', s @ ..] => {
									super::push(out, 582);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'l', 'a', s @ ..] => {
							super::push(out, 467);
							match s {
								[] => true,
								['b', 'é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 583);
									s.is_empty()
								}
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
			super::push(out, 126);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 302);
					match s {
						[] => true,
						['u', 'o', 'p', 'é', '\u{1}', s @ ..] => {
							super::push(out, 468);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 303);
					match s {
						[] => true,
						['c', 'n', 'i', 'é', s @ ..] => {
							super::push(out, 469);
							s.is_empty()
						}
						['h', 'c', 'a', s @ ..] => {
							super::push(out, 470);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 't', 's', s @ ..] => {
					super::push(out, 304);
					match s {
						[] => true,
						['o', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 128);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -per.
pub(crate) fn n27(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 27);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['i', 'p', s @ ..] => {
					super::push(out, 305);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 472);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rer.
pub(crate) fn n28(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 28);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 130);
			match s {
				[] => true,
				['p', 'm', s @ ..] => {
					super::push(out, 306);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 131);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 308);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 132);
			match s {
				[] => true,
				['é', s @ ..] => {
					super::push(out, 309);
					match s {
						[] => true,
						['x', 'e', '\u{1}', s @ ..] => {
							super::push(out, 474);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 133);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 134);
			s.is_empty()
		}
		['g', s @ ..] => {
			super::push(out, 135);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 310);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 311);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 136);
			match s {
				[] => true,
				['o', 'f', 'f', s @ ..] => {
					super::push(out, 312);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 137);
			match s {
				[] => true,
				['l', 'p', s @ ..] => {
					super::push(out, 313);
					match s {
						[] => true,
						['é', '\u{1}', s @ ..] => {
							super::push(out, 476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 138);
			match s {
				[] => true,
				['a', 'm', s @ ..] => {
					super::push(out, 314);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 139);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 315);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 317);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 140);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 318);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 479);
							s.is_empty()
						}
						['t', 'i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 480);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 322);
					match s {
						[] => true,
						['i', 'f', 'n', 'e', '\u{1}', s @ ..] => {
							super::push(out, 481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ser.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 143);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 144);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 323);
					match s {
						[] => true,
						['b', 'e', s @ ..] => {
							super::push(out, 482);
							match s {
								[] => true,
								['r', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 584);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 324);
					match s {
						[] => true,
						['r', 'c', 'e', s @ ..] => {
							super::push(out, 483);
							match s {
								[] => true,
								['r', 't', 'n', 'e', '\u{1}', s @ ..] => {
									super::push(out, 585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 325);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 484);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 586);
									s.is_empty()
								}
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
			super::push(out, 145);
			match s {
				[] => true,
				['p', 's', 'e', s @ ..] => {
					super::push(out, 326);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 486);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 587);
									match s {
										[] => true,
										['i', 'u', 'o', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 633);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 487);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 327);
					match s {
						[] => true,
						['a', 'e', s @ ..] => {
							super::push(out, 488);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									super::push(out, 588);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ter.
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 148);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 149);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 328);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 329);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'c', 'é', 'd', '\u{1}', s @ ..] => {
									super::push(out, 589);
									s.is_empty()
								}
								['u', 'o', 'm', '\u{1}', s @ ..] => {
									super::push(out, 590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					super::push(out, 330);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['f', 'f', 'u', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 489);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 591);
									s.is_empty()
								}
								['i', s @ ..] => {
									match s {
										[] => true,
										['a', 'c', '\u{1}', s @ ..] => {
											super::push(out, 634);
											s.is_empty()
										}
										['b', '\u{1}', s @ ..] => {
											super::push(out, 635);
											s.is_empty()
										}
										['u', s @ ..] => {
											match s {
												[] => true,
												['e', 'f', '\u{1}', s @ ..] => {
													super::push(out, 647);
													s.is_empty()
												}
												['g', 'i', 'a', '\u{1}', s @ ..] => {
													super::push(out, 648);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 636);
											s.is_empty()
										}
										['é', 'd', '\u{1}', s @ ..] => {
											super::push(out, 637);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', 'v', '\u{1}', s @ ..] => {
							super::push(out, 490);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'r', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 331);
					s.is_empty()
				}
				['r', 'u', 'c', '\u{1}', s @ ..] => {
					super::push(out, 332);
					s.is_empty()
				}
				['s', 's', 'u', 'o', 'p', 'é', '\u{1}', s @ ..] => {
					super::push(out, 333);
					s.is_empty()
				}
				['u', 'q', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 592);
									s.is_empty()
								}
								['c', '\u{1}', s @ ..] => {
									super::push(out, 593);
									s.is_empty()
								}
								['p', 'm', 'e', '\u{1}', s @ ..] => {
									super::push(out, 594);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', 'j', '\u{1}', s @ ..] => {
									super::push(out, 595);
									s.is_empty()
								}
								['e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 596);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 597);
									s.is_empty()
								}
								['h', s @ ..] => {
									super::push(out, 598);
									s.is_empty()
								}
								['l', 'c', '\u{1}', s @ ..] => {
									super::push(out, 599);
									s.is_empty()
								}
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 600);
									s.is_empty()
								}
								['t', 'é', '\u{1}', s @ ..] => {
									super::push(out, 601);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 491);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									super::push(out, 602);
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									super::push(out, 603);
									s.is_empty()
								}
								['r', 'r', 'e', 'p', '\u{1}', s @ ..] => {
									super::push(out, 604);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', s @ ..] => {
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 605);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 606);
									s.is_empty()
								}
								_ => false,
							}
						}
						['é', 'b', '\u{1}', s @ ..] => {
							super::push(out, 492);
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', 'b', '\u{1}', s @ ..] => {
							super::push(out, 493);
							s.is_empty()
						}
						['i', 'r', '\u{1}', s @ ..] => {
							super::push(out, 494);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 151);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', 'a', s @ ..] => {
							super::push(out, 495);
							match s {
								[] => true,
								['l', '\u{1}', s @ ..] => {
									super::push(out, 607);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 496);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 608);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 497);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 609);
									s.is_empty()
								}
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
			super::push(out, 154);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 334);
					match s {
						[] => true,
						['g', 'i', 'e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 335);
					match s {
						[] => true,
						['v', s @ ..] => {
							super::push(out, 499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', 'r', 'c', 'a', s @ ..] => {
					super::push(out, 336);
					match s {
						[] => true,
						['r', '\u{1}', s @ ..] => {
							super::push(out, 500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 157);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 158);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		['â', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 337);
					s.is_empty()
				}
				_ => false,
			}
		}
		['é', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['ê', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['û', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uer.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 338);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 339);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 340);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 341);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 342);
					match s {
						[] => true,
						['a', 't', s @ ..] => {
							super::push(out, 503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 343);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 344);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 165);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		['q', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['a', 'm', '\u{1}', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 346);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 504);
							match s {
								[] => true,
								['t', 'n', s @ ..] => {
									super::push(out, 610);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 168);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 349);
					s.is_empty()
				}
				['i', 't', 's', 'o', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ver.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 169);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 171);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 172);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['e', 'r', '\u{1}', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 174);
			match s {
				[] => true,
				['a', 's', 'n', s @ ..] => {
					super::push(out, 352);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							super::push(out, 505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yer.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 175);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 176);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 353);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 506);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 507);
							match s {
								[] => true,
								['s', 'e', 'd', '\u{1}', s @ ..] => {
									super::push(out, 611);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 354);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 508);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 612);
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 613);
									s.is_empty()
								}
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
			super::push(out, 178);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -éer.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	s.is_empty()
}

/// Words ending -mailler.
pub(crate) fn n483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 448);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 574);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -siller.
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 575);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -viller.
pub(crate) fn n485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 450);
	match s {
		[] => true,
		['e', 'u', 'q', 'o', 'r', 'c', 'e', 'r', '\u{1}', s @ ..] => {
			super::push(out, 576);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oler.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['v', s @ ..] => {
			super::push(out, 288);
			match s {
				[] => true,
				['n', 'e', s @ ..] => {
					super::push(out, 451);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pler.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	s.is_empty()
}

/// Words ending -rler.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	s.is_empty()
}

/// Words ending -sler.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['u', 'r', 'b', '\u{1}', s @ ..] => {
			super::push(out, 289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uler.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 290);
			match s {
				[] => true,
				['b', 'i', 'g', s @ ..] => {
					super::push(out, 452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', 'a', 'c', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 578);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'r', s @ ..] => {
					super::push(out, 454);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yler.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	s.is_empty()
}

/// Words ending -éler.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	s.is_empty()
}

/// Words ending -êler.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 291);
			s.is_empty()
		}
		_ => false,
	}
}

