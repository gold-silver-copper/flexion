use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['d', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 12);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 144);
									match s {
										[] => true,
										['e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 378);
											s.is_empty()
										}
										['u', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 379);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 145);
									match s {
										[] => true,
										['ę', 'h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 380);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 146);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 147);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											super::push(out, 381);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 382);
											s.is_empty()
										}
										['w', s @ ..] => {
											super::push(out, 383);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', 'ł', 'd', '\u{1}', s @ ..] => {
							super::push(out, 38);
							s.is_empty()
						}
						['ę', s @ ..] => {
							super::push(out, 39);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 384);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 385);
											s.is_empty()
										}
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
					super::push(out, 13);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 14);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 40);
							match s {
								[] => true,
								['y', s @ ..] => {
									match s {
										[] => true,
										['h', 'c', '\u{1}', s @ ..] => {
											super::push(out, 386);
											s.is_empty()
										}
										['z', 's', '\u{1}', s @ ..] => {
											super::push(out, 387);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 41);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 148);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 388);
											match s {
												[] => true,
												['r', 'u', s @ ..] => {
													super::push(out, 778);
													match s {
														[] => true,
														['b', s @ ..] => super::t0::n1495(s, out),
														['f', s @ ..] => super::t0::n1496(s, out),
														_ => false,
													}
												}
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
							super::push(out, 42);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 149);
									s.is_empty()
								}
								['o', 'ł', 's', 'ł', s @ ..] => {
									super::push(out, 150);
									s.is_empty()
								}
								['y', 'z', 'r', 'b', 'e', s @ ..] => {
									super::push(out, 151);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => super::t0::n44(s, out),
						['i', s @ ..] => super::t0::n45(s, out),
						['k', s @ ..] => super::t0::n46(s, out),
						['l', s @ ..] => super::t0::n47(s, out),
						['n', s @ ..] => super::t0::n48(s, out),
						['o', s @ ..] => super::t0::n49(s, out),
						['s', s @ ..] => super::t0::n50(s, out),
						['t', s @ ..] => super::t0::n51(s, out),
						['z', s @ ..] => super::t0::n52(s, out),
						['ż', s @ ..] => super::t0::n53(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n15(s, out),
				['m', s @ ..] => super::t0::n16(s, out),
				['n', s @ ..] => super::t0::n17(s, out),
				['p', s @ ..] => super::t0::n18(s, out),
				['s', s @ ..] => super::t0::n19(s, out),
				['w', s @ ..] => super::t0::n20(s, out),
				['z', s @ ..] => super::t0::n21(s, out),
				_ => false,
			}
		}
		['j', s @ ..] => super::t0::n3(s, out),
		['k', s @ ..] => super::t0::n4(s, out),
		['n', s @ ..] => super::t0::n5(s, out),
		['t', s @ ..] => super::t0::n6(s, out),
		['w', s @ ..] => super::t0::n7(s, out),
		['y', s @ ..] => super::t0::n8(s, out),
		_ => false,
	}
}

/// Words ending -buriacki.
pub(crate) fn n1495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1235);
	s.is_empty()
}

/// Words ending -furiacki.
pub(crate) fn n1496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1236);
	s.is_empty()
}

/// Words ending -eki.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['l', 'a', 'd', '\u{1}', s @ ..] => {
			super::push(out, 152);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -iki.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	s.is_empty()
}

/// Words ending -kki.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['l', 'e', s @ ..] => {
					super::push(out, 389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 154);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lki.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['e', 'i', 'w', '\u{1}', s @ ..] => {
			super::push(out, 155);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nki.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	s.is_empty()
}

/// Words ending -oki.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 156);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 157);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 158);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 159);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ski.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['l', 'b', '\u{1}', s @ ..] => {
					super::push(out, 390);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			super::push(out, 161);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 162);
			s.is_empty()
		}
		['w', s @ ..] => {
			super::push(out, 163);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 164);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 392);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 393);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 165);
			match s {
				[] => true,
				['m', '\u{1}', s @ ..] => {
					super::push(out, 395);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ń', s @ ..] => {
			super::push(out, 166);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tki.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 167);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 168);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 169);
			match s {
				[] => true,
				['r', 'k', '\u{1}', s @ ..] => {
					super::push(out, 396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zki.
pub(crate) fn n52(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['ą', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['w', '\u{1}', s @ ..] => {
					super::push(out, 397);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żki.
pub(crate) fn n53(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 52);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 172);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 173);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -li.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	s.is_empty()
}

/// Words ending -mi.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	s.is_empty()
}

/// Words ending -ni.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 175);
					match s {
						[] => true,
						['i', 'w', 'o', 'p', 'd', 'o', '\u{1}', s @ ..] => {
							super::push(out, 398);
							s.is_empty()
						}
						['l', 'ś', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 399);
							s.is_empty()
						}
						['r', 'ś', 'o', 'p', 'z', s @ ..] => {
							super::push(out, 400);
							s.is_empty()
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 779);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 780);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 176);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 177);
					match s {
						[] => true,
						['t', 's', 'o', 'd', s @ ..] => {
							super::push(out, 401);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 781);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 178);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pi.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['u', 'ł', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['g', '\u{1}', s @ ..] => {
					super::push(out, 179);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -si.
pub(crate) fn n19(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 19);
	match s {
		[] => true,
		['u', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['l', 'a', s @ ..] => {
					super::push(out, 180);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -wi.
pub(crate) fn n20(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 20);
	s.is_empty()
}

/// Words ending -zi.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	s.is_empty()
}

/// Words ending -j.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	s.is_empty()
}

/// Words ending -k.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	s.is_empty()
}

/// Words ending -n.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 22);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 23);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -t.
pub(crate) fn n6(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6);
	s.is_empty()
}

/// Words ending -w.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	s.is_empty()
}

/// Words ending -y.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 58);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 181);
							s.is_empty()
						}
						['r', 'g', 'y', s @ ..] => {
							super::push(out, 182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ę', 'z', 'z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 59);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 25);
			match s {
				[] => true,
				['ą', s @ ..] => {
					super::push(out, 60);
					match s {
						[] => true,
						['j', s @ ..] => {
							super::push(out, 183);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 184);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 402);
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 783);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 784);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 26);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 185);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 186);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 61);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 187);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'w', s @ ..] => {
					super::push(out, 62);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 189);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 190);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 63);
					match s {
						[] => true,
						['a', 'r', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 64);
					match s {
						[] => true,
						['ł', 'b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 403);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 404);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 192);
							match s {
								[] => true,
								['l', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 405);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 406);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 407);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 408);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 409);
									s.is_empty()
								}
								['ł', 'g', '\u{1}', s @ ..] => {
									super::push(out, 410);
									s.is_empty()
								}
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
			super::push(out, 28);
			match s {
				[] => true,
				['o', 'r', 'h', s @ ..] => {
					super::push(out, 66);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							super::push(out, 193);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => super::t0::n29(s, out),
		['p', s @ ..] => {
			super::push(out, 30);
			match s {
				[] => true,
				['ę', s @ ..] => {
					super::push(out, 91);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 284);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 285);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 31);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 92);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 286);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1104);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 1105);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 93);
					match s {
						[] => true,
						['o', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 678);
									s.is_empty()
								}
								['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 679);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 94);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 680);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ą', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 682);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 683);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 95);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 96);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 287);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 97);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 289);
							s.is_empty()
						}
						['k', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 684);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 98);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['o', 'e', s @ ..] => {
									super::push(out, 686);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 687);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 688);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 99);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t0::n32(s, out),
		['t', s @ ..] => super::t0::n33(s, out),
		['w', s @ ..] => super::t0::n34(s, out),
		['z', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 122);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 123);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 124);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ł', s @ ..] => super::t0::n36(s, out),
		['ż', s @ ..] => {
			super::push(out, 37);
			match s {
				[] => true,
				['e', s @ ..] => super::t0::n145(s, out),
				['u', s @ ..] => {
					super::push(out, 142);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => super::t0::n440(s, out),
						_ => false,
					}
				}
				['y', 'h', 'c', '\u{1}', s @ ..] => super::t0::n147(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ny.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 194);
					match s {
						[] => true,
						['ą', s @ ..] => {
							super::push(out, 411);
							match s {
								[] => true,
								['ż', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 195);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 412);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 786);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 196);
					match s {
						[] => true,
						['a', 'ł', s @ ..] => {
							super::push(out, 413);
							match s {
								[] => true,
								['p', 'o', '\u{1}', s @ ..] => {
									super::push(out, 787);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'z', 's', 'y', s @ ..] => {
							super::push(out, 414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 197);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 198);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 199);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 415);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 416);
							match s {
								[] => true,
								['b', 'r', 'u', s @ ..] => {
									super::push(out, 788);
									s.is_empty()
								}
								['k', s @ ..] => {
									super::push(out, 789);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 790);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 1237);
											match s {
												[] => true,
												['f', s @ ..] => {
													super::push(out, 1526);
													match s {
														[] => true,
														['a', 'r', s @ ..] => super::t0::n1959(s, out),
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
									super::push(out, 791);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 792);
									match s {
										[] => true,
										['i', 'l', 'a', 't', s @ ..] => {
											super::push(out, 1238);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 1527);
													s.is_empty()
												}
												['t', s @ ..] => {
													super::push(out, 1528);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', 'r', s @ ..] => {
											super::push(out, 1239);
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 1529);
													s.is_empty()
												}
												['o', s @ ..] => {
													super::push(out, 1530);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ż', s @ ..] => {
									super::push(out, 793);
									match s {
										[] => true,
										['a', 'g', 'n', 'a', 'a', 'z', '\u{1}', s @ ..] => {
											super::push(out, 1240);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 417);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 200);
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 201);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 418);
							s.is_empty()
						}
						['w', 'd', s @ ..] => {
							super::push(out, 419);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'n', 'a', s @ ..] => {
							super::push(out, 420);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 795);
									s.is_empty()
								}
								['h', 'c', 'o', 'p', '\u{1}', s @ ..] => {
									super::push(out, 796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 797);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['g', s @ ..] => {
											super::push(out, 1241);
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 1242);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ż', s @ ..] => {
							super::push(out, 421);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 202);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 798);
									s.is_empty()
								}
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1243);
											s.is_empty()
										}
										['o', 'd', 'w', 'a', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1244);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 799);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'd', s @ ..] => {
							super::push(out, 422);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', 'z', s @ ..] => {
									super::push(out, 800);
									s.is_empty()
								}
								['o', 'p', 's', '\u{1}', s @ ..] => {
									super::push(out, 801);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 203);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 423);
							s.is_empty()
						}
						['l', s @ ..] => super::t0::n492(s, out),
						_ => false,
					}
				}
				['y', s @ ..] => super::t0::n214(s, out),
				['ę', s @ ..] => super::t0::n215(s, out),
				_ => false,
			}
		}
		['c', s @ ..] => super::t0::n73(s, out),
		['d', s @ ..] => super::t0::n74(s, out),
		['f', s @ ..] => super::t0::n75(s, out),
		['h', s @ ..] => super::t0::n76(s, out),
		['i', s @ ..] => super::t0::n77(s, out),
		['j', s @ ..] => super::t0::n78(s, out),
		['k', s @ ..] => super::t0::n79(s, out),
		['l', s @ ..] => super::t0::n80(s, out),
		['m', s @ ..] => super::t0::n81(s, out),
		['n', s @ ..] => super::t0::n82(s, out),
		['o', s @ ..] => super::t0::n83(s, out),
		['p', s @ ..] => super::t0::n84(s, out),
		['r', s @ ..] => super::t0::n85(s, out),
		['s', s @ ..] => super::t0::n86(s, out),
		['t', s @ ..] => super::t0::n87(s, out),
		['w', s @ ..] => super::t0::n88(s, out),
		['y', s @ ..] => super::t0::n89(s, out),
		['z', s @ ..] => super::t0::n90(s, out),
		['ł', s @ ..] => super::t0::n91(s, out),
		['ś', s @ ..] => super::t0::n92(s, out),
		['ź', s @ ..] => super::t0::n93(s, out),
		['ż', s @ ..] => super::t0::n94(s, out),
		_ => false,
	}
}

/// Words ending -sy.
pub(crate) fn n32(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 32);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 101);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 291);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 292);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 293);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1106);
									s.is_empty()
								}
								['g', s @ ..] => {
									super::push(out, 1107);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 1108);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1109);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'u', 'k', '\u{1}', s @ ..] => {
							super::push(out, 689);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 690);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 294);
					match s {
						[] => true,
						['w', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1110);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1111);
									s.is_empty()
								}
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
			super::push(out, 103);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 104);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ty.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 33);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 106);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 295);
					match s {
						[] => true,
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 691);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 296);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1112);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 1449);
											s.is_empty()
										}
										_ => false,
									}
								}
								['k', s @ ..] => {
									super::push(out, 1113);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1114);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 1450);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1115);
									match s {
										[] => true,
										['a', 'm', s @ ..] => {
											super::push(out, 1451);
											match s {
												[] => true,
												['h', 'c', 'a', 'ł', '\u{1}', s @ ..] => {
													super::push(out, 1628);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 107);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 297);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 298);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 299);
					s.is_empty()
				}
				['l', 'o', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 692);
							s.is_empty()
						}
						['p', 's', 'o', 'p', '\u{1}', s @ ..] => {
							super::push(out, 693);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 300);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 301);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 694);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 695);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 302);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 303);
					s.is_empty()
				}
				['w', 'o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 696);
							s.is_empty()
						}
						['d', 'a', s @ ..] => {
							super::push(out, 697);
							s.is_empty()
						}
						['k', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1116);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 1117);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 698);
							s.is_empty()
						}
						['n', 'a', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 699);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 700);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 701);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 702);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 108);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 109);
			match s {
				[] => true,
				['a', 'w', s @ ..] => {
					super::push(out, 304);
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 703);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1452);
											s.is_empty()
										}
										['ł', s @ ..] => {
											super::push(out, 1453);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 110);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 305);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 704);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 705);
							match s {
								[] => true,
								['a', 'w', 'd', 'e', 'j', 'a', s @ ..] => {
									super::push(out, 1118);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1119);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1120);
									s.is_empty()
								}
								['ą', 'r', 'a', 'z', '\u{1}', s @ ..] => {
									super::push(out, 1121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1122);
									match s {
										[] => true,
										['ł', 'b', '\u{1}', s @ ..] => {
											super::push(out, 1454);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ś', s @ ..] => {
									super::push(out, 1123);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1124);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1125);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 706);
							s.is_empty()
						}
						['n', s @ ..] => super::t0::n859(s, out),
						['o', s @ ..] => {
							super::push(out, 707);
							s.is_empty()
						}
						['s', 'ę', s @ ..] => super::t0::n861(s, out),
						['w', s @ ..] => super::t0::n862(s, out),
						['z', s @ ..] => super::t0::n863(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 307);
					s.is_empty()
				}
				['u', s @ ..] => super::t0::n357(s, out),
				['y', s @ ..] => super::t0::n358(s, out),
				['ę', s @ ..] => super::t0::n359(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n115(s, out),
		['y', s @ ..] => super::t0::n116(s, out),
		['ą', s @ ..] => super::t0::n117(s, out),
		['ę', s @ ..] => super::t0::n118(s, out),
		['ł', s @ ..] => super::t0::n119(s, out),
		_ => false,
	}
}

/// Words ending -wy.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 314);
					match s {
						[] => true,
						['u', 'l', s @ ..] => {
							super::push(out, 721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 315);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 722);
							match s {
								[] => true,
								['i', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1465);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1466);
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 1630);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 1631);
													s.is_empty()
												}
												_ => false,
											}
										}
										['y', s @ ..] => {
											super::push(out, 1467);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 723);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1137);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1138);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 316);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 317);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 724);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'ż', s @ ..] => {
					super::push(out, 318);
					s.is_empty()
				}
				['z', 'ł', s @ ..] => {
					super::push(out, 319);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 320);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 117);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 726);
							s.is_empty()
						}
						['z', 'c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1139);
									s.is_empty()
								}
								['u', 'e', s @ ..] => {
									super::push(out, 1140);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ś', s @ ..] => {
							match s {
								[] => true,
								['a', 'ł', 'w', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1468);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1469);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'r', 't', 'z', s @ ..] => {
									super::push(out, 1141);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1142);
									s.is_empty()
								}
								['o', 't', 'i', 'l', 'e', s @ ..] => {
									super::push(out, 1143);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => super::t0::n373(s, out),
				['n', s @ ..] => {
					super::push(out, 321);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 322);
					s.is_empty()
				}
				['z', s @ ..] => super::t0::n376(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t0::n122(s, out),
		['t', s @ ..] => super::t0::n123(s, out),
		['y', s @ ..] => super::t0::n124(s, out),
		['ź', s @ ..] => {
			super::push(out, 121);
			match s {
				[] => true,
				['e', 'z', 'r', 't', 'e', s @ ..] => {
					super::push(out, 337);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ły.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 126);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 338);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 751);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1205);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1206);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 752);
							match s {
								[] => true,
								['u', s @ ..] => {
									super::push(out, 1207);
									match s {
										[] => true,
										['z', 'o', 'r', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 1644);
													s.is_empty()
												}
												['y', s @ ..] => {
													super::push(out, 1645);
													s.is_empty()
												}
												['z', s @ ..] => {
													super::push(out, 1646);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['z', s @ ..] => {
									super::push(out, 1208);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 1209);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1521);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1522);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 753);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1210);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 754);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 339);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 340);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 755);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 341);
					s.is_empty()
				}
				['t', 's', '\u{1}', s @ ..] => {
					super::push(out, 342);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 343);
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 756);
							s.is_empty()
						}
						['r', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1211);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1212);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 344);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 757);
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 758);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1213);
									s.is_empty()
								}
								['j', 'o', s @ ..] => {
									super::push(out, 1214);
									match s {
										[] => true,
										['d', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1647);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1648);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', 'g', s @ ..] => {
									super::push(out, 1215);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 1523);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 759);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 127);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 345);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 346);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 128);
			match s {
				[] => true,
				['a', 'j', s @ ..] => {
					super::push(out, 347);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 348);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 129);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['i', 'b', 'e', s @ ..] => {
							super::push(out, 760);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['d', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1216);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1217);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1218);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 349);
					match s {
						[] => true,
						['c', 'ś', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1220);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1221);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', s @ ..] => {
					super::push(out, 350);
					s.is_empty()
				}
				['ą', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 762);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 763);
							match s {
								[] => true,
								['k', 'o', 'ł', s @ ..] => {
									super::push(out, 1222);
									s.is_empty()
								}
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
			super::push(out, 130);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['s', 'o', s @ ..] => {
							super::push(out, 764);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 765);
							match s {
								[] => true,
								['r', 'e', 'i', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1223);
									s.is_empty()
								}
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
			super::push(out, 131);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 766);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 351);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => super::t0::n136(s, out),
		['o', s @ ..] => {
			super::push(out, 133);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 361);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 362);
					match s {
						[] => true,
						['e', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1230);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => super::t0::n138(s, out),
		['r', s @ ..] => {
			super::push(out, 135);
			s.is_empty()
		}
		['s', s @ ..] => super::t0::n140(s, out),
		['t', s @ ..] => super::t0::n141(s, out),
		['u', s @ ..] => super::t0::n142(s, out),
		['y', s @ ..] => super::t0::n143(s, out),
		['z', s @ ..] => super::t0::n144(s, out),
		_ => false,
	}
}

/// Words ending -eży.
pub(crate) fn n145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['i', 'w', 'ś', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 776);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 777);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word duży.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 377);
	s.is_empty()
}

/// The word chyży.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 143);
	s.is_empty()
}

/// Words ending -rafinowany.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 1691);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 1692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lubny.
pub(crate) fn n492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1531);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ybny.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	s.is_empty()
}

/// Words ending -ębny.
pub(crate) fn n215(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	s.is_empty()
}

/// Words ending -cny.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 206);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 425);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 426);
					s.is_empty()
				}
				['z', 'e', 'b', '\u{1}', s @ ..] => {
					super::push(out, 427);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 804);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 805);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 428);
					s.is_empty()
				}
				['w', 'o', '\u{1}', s @ ..] => {
					super::push(out, 429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dny.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['n', 's', '\u{1}', s @ ..] => {
					super::push(out, 430);
					s.is_empty()
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['k', 'z', 's', '\u{1}', s @ ..] => {
							super::push(out, 806);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 807);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'z', '\u{1}', s @ ..] => {
							super::push(out, 808);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 809);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 810);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 811);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', '\u{1}', s @ ..] => {
											super::push(out, 1533);
											s.is_empty()
										}
										['z', '\u{1}', s @ ..] => {
											super::push(out, 1534);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1535);
											s.is_empty()
										}
										['e', 'i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 1536);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
									super::push(out, 1245);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 1537);
											s.is_empty()
										}
										['p', s @ ..] => {
											super::push(out, 1538);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 1246);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1247);
									s.is_empty()
								}
								['y', s @ ..] => {
									match s {
										[] => true,
										['w', '\u{1}', s @ ..] => {
											super::push(out, 1539);
											s.is_empty()
										}
										['z', 'r', 'p', 'z', s @ ..] => {
											super::push(out, 1540);
											s.is_empty()
										}
										_ => false,
									}
								}
								['z', 'o', 'r', '\u{1}', s @ ..] => {
									super::push(out, 1248);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', s @ ..] => {
							super::push(out, 812);
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
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 813);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 815);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 816);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					super::push(out, 431);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 432);
					s.is_empty()
				}
				['l', 'o', 's', 'e', s @ ..] => {
					super::push(out, 433);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 434);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 435);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 817);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 818);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 819);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1249);
									s.is_empty()
								}
								['p', 'e', s @ ..] => {
									super::push(out, 1250);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['r', 'a', s @ ..] => {
									match s {
										[] => true,
										['i', 'w', 'e', s @ ..] => {
											super::push(out, 1541);
											s.is_empty()
										}
										['k', s @ ..] => {
											super::push(out, 1542);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', 'e', s @ ..] => {
									super::push(out, 1251);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1252);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'e', s @ ..] => {
							super::push(out, 820);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 436);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 437);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 821);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 438);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1253);
									s.is_empty()
								}
								['n', 'ż', s @ ..] => {
									super::push(out, 1254);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 1255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 439);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 440);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 823);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 824);
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1256);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1257);
									s.is_empty()
								}
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
			super::push(out, 208);
			match s {
				[] => true,
				['c', s @ ..] => super::t0::n516(s, out),
				['k', s @ ..] => {
					super::push(out, 441);
					s.is_empty()
				}
				['l', s @ ..] => super::t0::n518(s, out),
				['m', s @ ..] => super::t0::n519(s, out),
				['n', s @ ..] => super::t0::n520(s, out),
				['r', s @ ..] => super::t0::n521(s, out),
				['ł', s @ ..] => super::t0::n522(s, out),
				_ => false,
			}
		}
		['y', s @ ..] => super::t0::n224(s, out),
		['z', s @ ..] => super::t0::n225(s, out),
		['ą', s @ ..] => super::t0::n226(s, out),
		['ę', s @ ..] => super::t0::n227(s, out),
		_ => false,
	}
}

/// Words ending -fny.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 213);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 214);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', 's', '\u{1}', s @ ..] => {
			super::push(out, 216);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -hny.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['c', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 450);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 451);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 452);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iny.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	s.is_empty()
}

/// Words ending -jny.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 841);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 842);
							match s {
								[] => true,
								['w', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1543);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1544);
											s.is_empty()
										}
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
					super::push(out, 454);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 455);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 456);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['c', 'y', 'w', 'z', '\u{1}', s @ ..] => {
							super::push(out, 843);
							s.is_empty()
						}
						['d', 'o', 'r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 844);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 218);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 458);
					s.is_empty()
				}
				['k', 'o', 'p', 's', 'e', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				['n', 'z', '\u{1}', s @ ..] => {
					super::push(out, 460);
					s.is_empty()
				}
				['p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 845);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 846);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 463);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 464);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 847);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 848);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1270);
									match s {
										[] => true,
										['r', 't', s @ ..] => {
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													super::push(out, 1649);
													s.is_empty()
												}
												['s', 'b', 'a', '\u{1}', s @ ..] => {
													super::push(out, 1650);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 1271);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 849);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1272);
									match s {
										[] => true,
										['r', s @ ..] => {
											match s {
												[] => true,
												['a', s @ ..] => {
													super::push(out, 1651);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1652);
													match s {
														[] => true,
														['l', 'o', 't', 'e', s @ ..] => super::t0::n2007(s, out),
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
									super::push(out, 1273);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 466);
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 850);
							match s {
								[] => true,
								['c', 'e', 'r', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1545);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1546);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kny.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['ę', 'i', 'p', '\u{1}', s @ ..] => {
			super::push(out, 222);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lny.
pub(crate) fn n80(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 223);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 467);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 851);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 852);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'r', s @ ..] => {
					super::push(out, 468);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 469);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 853);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 854);
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							super::push(out, 855);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 470);
					s.is_empty()
				}
				['h', 'c', 'a', s @ ..] => {
					super::push(out, 471);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', 'e', s @ ..] => {
							super::push(out, 856);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 857);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 858);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1274);
									s.is_empty()
								}
								['o', 'j', '\u{1}', s @ ..] => {
									super::push(out, 1275);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1276);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 859);
							match s {
								[] => true,
								['d', 'e', 'i', 'w', 'o', 'p', 'd', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1277);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					match s {
						[] => true,
						['c', 'e', s @ ..] => {
							super::push(out, 860);
							match s {
								[] => true,
								['p', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1547);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1548);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 861);
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1549);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1550);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'y', 'd', s @ ..] => {
					super::push(out, 472);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 473);
					match s {
						[] => true,
						['r', 'o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 862);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 863);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1278);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 1279);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 474);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 864);
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									super::push(out, 1280);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', 'g', s @ ..] => {
							super::push(out, 865);
							match s {
								[] => true,
								['y', 'r', 'o', '\u{1}', s @ ..] => {
									super::push(out, 1281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'a', s @ ..] => {
					super::push(out, 475);
					match s {
						[] => true,
						['z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1282);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 1283);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 476);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1284);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1285);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1286);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1551);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1287);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'm', s @ ..] => {
							super::push(out, 867);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1288);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 1289);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1290);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1291);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1292);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1293);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 't', s @ ..] => {
							match s {
								[] => true,
								['a', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1294);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1295);
									match s {
										[] => true,
										['u', 'k', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1653);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1654);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 477);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 868);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1296);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 869);
							s.is_empty()
						}
						['u', s @ ..] => super::t0::n1062(s, out),
						_ => false,
					}
				}
				['u', 't', s @ ..] => super::t0::n566(s, out),
				['w', s @ ..] => {
					super::push(out, 479);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => super::t0::n240(s, out),
		['i', s @ ..] => super::t0::n241(s, out),
		['o', s @ ..] => super::t0::n242(s, out),
		['u', s @ ..] => super::t0::n243(s, out),
		['y', s @ ..] => super::t0::n244(s, out),
		['ó', s @ ..] => super::t0::n245(s, out),
		['ś', s @ ..] => super::t0::n246(s, out),
		_ => false,
	}
}

/// Words ending -mny.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1314);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 895);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1316);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 1317);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 896);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 897);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1318);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 1319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'f', '\u{1}', s @ ..] => {
							super::push(out, 898);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', 'c', 'k', s @ ..] => {
					super::push(out, 502);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 231);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['g', 'o', '\u{1}', s @ ..] => {
							super::push(out, 899);
							s.is_empty()
						}
						['k', 's', '\u{1}', s @ ..] => {
							super::push(out, 900);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 901);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 503);
					match s {
						[] => true,
						['y', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['o', 'k', s @ ..] => {
							super::push(out, 903);
							s.is_empty()
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 904);
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
				['d', s @ ..] => {
					super::push(out, 504);
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					super::push(out, 505);
					s.is_empty()
				}
				['z', s @ ..] => {
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 905);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 507);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 508);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nny.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 232);
			match s {
				[] => true,
				['r', 'a', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 509);
					s.is_empty()
				}
				['ł', 'h', 'c', 'a', s @ ..] => {
					super::push(out, 510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 233);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 907);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 909);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 910);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1554);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 1555);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 1320);
									match s {
										[] => true,
										['o', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1655);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1656);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['e', 'z', 'r', 'b', s @ ..] => {
									super::push(out, 1321);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1556);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'r', s @ ..] => {
									super::push(out, 1322);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											super::push(out, 1557);
											s.is_empty()
										}
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
					super::push(out, 512);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 911);
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
				['c', s @ ..] => {
					super::push(out, 514);
					match s {
						[] => true,
						['ś', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'z', s @ ..] => {
					super::push(out, 515);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 234);
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 517);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
							super::push(out, 913);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1323);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1324);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', s @ ..] => {
							match s {
								[] => true,
								['o', 'p', s @ ..] => {
									super::push(out, 1325);
									s.is_empty()
								}
								['u', '\u{1}', s @ ..] => {
									super::push(out, 1326);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 518);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 914);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					match s {
						[] => true,
						['h', s @ ..] => {
							super::push(out, 916);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 917);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1558);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1559);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', 't', 'r', 'o', 'f', '\u{1}', s @ ..] => {
			super::push(out, 235);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				['z', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 919);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 920);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['p', 'ł', s @ ..] => {
							super::push(out, 922);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ony.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 237);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 521);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1327);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 1328);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1560);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 239);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 523);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 1329);
									s.is_empty()
								}
								['m', s @ ..] => {
									super::push(out, 1330);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1331);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1561);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1562);
											s.is_empty()
										}
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
					super::push(out, 524);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['r', 'e', 'z', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 925);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 926);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 927);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1332);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', s @ ..] => {
							super::push(out, 928);
							match s {
								[] => true,
								['s', 'a', s @ ..] => {
									super::push(out, 1333);
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 1563);
											s.is_empty()
										}
										['ł', s @ ..] => {
											super::push(out, 1564);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 243);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 929);
							s.is_empty()
						}
						['n', 'u', s @ ..] => {
							super::push(out, 930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['ł', 's', 'a', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1334);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1335);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pny.
pub(crate) fn n84(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 80);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 245);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 246);
			s.is_empty()
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 529);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 530);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 531);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 931);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 932);
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
				['d', s @ ..] => {
					super::push(out, 532);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			match s {
				[] => true,
				['s', 'o', s @ ..] => {
					super::push(out, 534);
					s.is_empty()
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 933);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 934);
							s.is_empty()
						}
						['o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 935);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 936);
							s.is_empty()
						}
						['y', 'z', s @ ..] => {
							super::push(out, 937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rny.
pub(crate) fn n85(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 81);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 247);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 535);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1336);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1337);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 938);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 939);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 940);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 536);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 537);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 538);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 539);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 540);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 1338);
									match s {
										[] => true,
										['e', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1657);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1658);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 1339);
									match s {
										[] => true,
										['o', 'p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1659);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1660);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 541);
					match s {
						[] => true,
						['y', s @ ..] => {
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
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 943);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 944);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 542);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 543);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 544);
					match s {
						[] => true,
						['a', 'm', 'a', 'l', 'ś', '\u{1}', s @ ..] => {
							super::push(out, 945);
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1340);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1341);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'ą', s @ ..] => {
			super::push(out, 248);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 249);
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 545);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1342);
									s.is_empty()
								}
								['ę', s @ ..] => {
									super::push(out, 1343);
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 946);
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1344);
									s.is_empty()
								}
								['o', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1565);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 1566);
											s.is_empty()
										}
										_ => false,
									}
								}
								['y', s @ ..] => {
									super::push(out, 1345);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1346);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 1347);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 947);
							s.is_empty()
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 948);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'e', 'f', '\u{1}', s @ ..] => {
					super::push(out, 546);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 547);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 949);
							s.is_empty()
						}
						['s', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 950);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 951);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 548);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 952);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 953);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 1348);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 549);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['y', s @ ..] => {
							super::push(out, 954);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 955);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 550);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 956);
							s.is_empty()
						}
						['z', 'd', s @ ..] => {
							super::push(out, 957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					super::push(out, 551);
					s.is_empty()
				}
				['l', 'a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 552);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 553);
					match s {
						[] => true,
						['d', 'o', s @ ..] => super::t0::n1165(s, out),
						['o', s @ ..] => super::t0::n1166(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 554);
					s.is_empty()
				}
				['w', 't', s @ ..] => super::t0::n663(s, out),
				['z', s @ ..] => super::t0::n664(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t0::n276(s, out),
		['ó', s @ ..] => super::t0::n277(s, out),
		_ => false,
	}
}

/// Words ending -sny.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 253);
			match s {
				[] => true,
				['i', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 966);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 967);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 968);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 969);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 563);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 564);
					s.is_empty()
				}
				['z', 'c', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1355);
									s.is_empty()
								}
								['p', '\u{1}', s @ ..] => {
									super::push(out, 1356);
									s.is_empty()
								}
								['w', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 1357);
									s.is_empty()
								}
								_ => false,
							}
						}
						['w', '\u{1}', s @ ..] => {
							super::push(out, 970);
							s.is_empty()
						}
						['ł', s @ ..] => {
							super::push(out, 971);
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
				['d', 'a', 'r', '\u{1}', s @ ..] => {
					super::push(out, 565);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 566);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 254);
			match s {
				[] => true,
				['z', 'c', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 568);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tny.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 569);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 972);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['w', s @ ..] => {
									super::push(out, 1358);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1359);
									s.is_empty()
								}
								_ => false,
							}
						}
						['z', 'e', s @ ..] => {
							super::push(out, 973);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', 'i', 'l', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 570);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 571);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['t', s @ ..] => {
							super::push(out, 974);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'k', s @ ..] => {
					super::push(out, 572);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['h', 'c', 'a', 'l', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 574);
					s.is_empty()
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['w', 'ś', s @ ..] => {
							super::push(out, 976);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 575);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 576);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 577);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 978);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 255);
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 979);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 980);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 981);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1360);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 1361);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 982);
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 580);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 581);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 582);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 256);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['k', s @ ..] => {
							super::push(out, 983);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 984);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'g', s @ ..] => {
					super::push(out, 586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 587);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 588);
					s.is_empty()
				}
				['h', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 589);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 590);
					s.is_empty()
				}
				['k', 's', 'y', 'ł', 'b', '\u{1}', s @ ..] => {
					super::push(out, 591);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 985);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 986);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 987);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['a', 'r', 'g', '\u{1}', s @ ..] => {
							super::push(out, 988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 593);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 989);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									super::push(out, 1362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['k', s @ ..] => {
							super::push(out, 990);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 991);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1363);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1364);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 1365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', 'i', '\u{1}', s @ ..] => {
					super::push(out, 595);
					s.is_empty()
				}
				['w', 'y', 'ż', '\u{1}', s @ ..] => {
					super::push(out, 596);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 257);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 258);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 992);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 599);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['z', 'r', 'o', 'k', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1366);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1367);
									s.is_empty()
								}
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
			super::push(out, 259);
			match s {
				[] => true,
				['b', 'e', s @ ..] => {
					super::push(out, 601);
					s.is_empty()
				}
				['h', 'c', '\u{1}', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['e', 's', '\u{1}', s @ ..] => {
							super::push(out, 994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 604);
					s.is_empty()
				}
				['m', 'a', 'ł', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 605);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 606);
					s.is_empty()
				}
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 995);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 996);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 607);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['p', 's', 'o', 'n', 'ł', 'e', 'p', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'h', 'c', 'u', '\u{1}', s @ ..] => {
					super::push(out, 609);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1368);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1369);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ż', s @ ..] => {
					super::push(out, 611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', 'ł', 'k', '\u{1}', s @ ..] => {
			super::push(out, 260);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 261);
			s.is_empty()
		}
		['ę', s @ ..] => super::t0::n293(s, out),
		['ł', 'a', 't', 'z', 's', 'k', '\u{1}', s @ ..] => {
			super::push(out, 263);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wny.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 84);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1005);
							s.is_empty()
						}
						['a', 'z', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1372);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1373);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1006);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1007);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1008);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['j', '\u{1}', s @ ..] => {
					super::push(out, 617);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 1567);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 1568);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1569);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1570);
											s.is_empty()
										}
										_ => false,
									}
								}
								['w', '\u{1}', s @ ..] => {
									super::push(out, 1374);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 1375);
									s.is_empty()
								}
								['y', 'w', '\u{1}', s @ ..] => {
									super::push(out, 1376);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							super::push(out, 1010);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1011);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ł', 's', '\u{1}', s @ ..] => {
					super::push(out, 618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1012);
							s.is_empty()
						}
						['w', 'z', s @ ..] => {
							super::push(out, 1013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 619);
					s.is_empty()
				}
				['p', '\u{1}', s @ ..] => {
					super::push(out, 620);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 621);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				['z', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1014);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1015);
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
				['l', s @ ..] => {
					super::push(out, 623);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1377);
									s.is_empty()
								}
								['y', s @ ..] => {
									super::push(out, 1378);
									s.is_empty()
								}
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
			super::push(out, 264);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 625);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 1016);
							s.is_empty()
						}
						['u', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1379);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1380);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['h', 'c', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1017);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 1018);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1020);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 1021);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1022);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 626);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1024);
							s.is_empty()
						}
						['y', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1025);
							s.is_empty()
						}
						['z', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1027);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1028);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1381);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 1382);
									s.is_empty()
								}
								['z', s @ ..] => {
									super::push(out, 1383);
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 1029);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['a', 'r', 'a', 'b', 'm', 'a', '\u{1}', s @ ..] => {
							super::push(out, 1031);
							s.is_empty()
						}
						['e', s @ ..] => super::t0::n1252(s, out),
						['n', 'e', 's', 'n', s @ ..] => super::t0::n1253(s, out),
						['o', s @ ..] => super::t0::n1254(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => super::t0::n760(s, out),
				['z', s @ ..] => super::t0::n761(s, out),
				['ł', s @ ..] => super::t0::n762(s, out),
				['ż', 'a', 't', 'n', 'a', 'w', 'a', '\u{1}', s @ ..] => super::t0::n763(s, out),
				_ => false,
			}
		}
		['r', 'a', 'b', s @ ..] => super::t0::n299(s, out),
		['y', s @ ..] => super::t0::n300(s, out),
		['ó', s @ ..] => super::t0::n301(s, out),
		_ => false,
	}
}

/// Words ending -yny.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 85);
	s.is_empty()
}

/// Words ending -zny.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 266);
			match s {
				[] => true,
				['j', s @ ..] => {
					super::push(out, 639);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 641);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 1050);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1051);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 1052);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1053);
							match s {
								[] => true,
								['z', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1584);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 1585);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'k', s @ ..] => {
							super::push(out, 1054);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 642);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1056);
							match s {
								[] => true,
								['r', 'e', 's', 'z', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 1399);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 1400);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1401);
									s.is_empty()
								}
								['w', s @ ..] => {
									super::push(out, 1402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['j', s @ ..] => {
							super::push(out, 1057);
							s.is_empty()
						}
						['l', 'a', s @ ..] => {
							super::push(out, 1058);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
									super::push(out, 1403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'ł', 's', '\u{1}', s @ ..] => {
							super::push(out, 1059);
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['a', 't', 's', s @ ..] => super::t0::n1684(s, out),
								['e', 'z', 's', 'w', '\u{1}', s @ ..] => super::t0::n1685(s, out),
								['o', s @ ..] => super::t0::n1686(s, out),
								['u', 'k', 's', '\u{1}', s @ ..] => super::t0::n1687(s, out),
								['y', s @ ..] => super::t0::n1688(s, out),
								['ą', s @ ..] => super::t0::n1689(s, out),
								_ => false,
							}
						}
						['z', 'r', s @ ..] => super::t0::n1287(s, out),
						['ł', s @ ..] => super::t0::n1288(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t0::n777(s, out),
				['o', s @ ..] => super::t0::n778(s, out),
				['u', s @ ..] => super::t0::n779(s, out),
				['y', s @ ..] => super::t0::n780(s, out),
				['ę', s @ ..] => super::t0::n781(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t0::n304(s, out),
		['r', s @ ..] => super::t0::n305(s, out),
		['s', s @ ..] => super::t0::n306(s, out),
		['y', s @ ..] => super::t0::n307(s, out),
		_ => false,
	}
}

/// Words ending -łny.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 271);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -śny.
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 656);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 657);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 658);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['l', 'b', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 1075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1076);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1077);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1443);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
								_ => false,
							}
						}
						['z', '\u{1}', s @ ..] => {
							super::push(out, 1078);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', 's', '\u{1}', s @ ..] => {
					super::push(out, 661);
					s.is_empty()
				}
				['ł', 'g', '\u{1}', s @ ..] => {
					super::push(out, 662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 275);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 663);
					s.is_empty()
				}
				['ł', 'a', 'c', '\u{1}', s @ ..] => {
					super::push(out, 664);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 276);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -źny.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1079);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1080);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1081);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1082);
							s.is_empty()
						}
						['y', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1444);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1445);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', 'g', '\u{1}', s @ ..] => {
					super::push(out, 665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 277);
			s.is_empty()
		}
		['o', 'r', s @ ..] => {
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1083);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1084);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1085);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1086);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 278);
			s.is_empty()
		}
		['ó', s @ ..] => {
			super::push(out, 279);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -żny.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 280);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1088);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 1089);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1090);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 1446);
									s.is_empty()
								}
								['p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1626);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1627);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1091);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1092);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['b', 'o', 'p', s @ ..] => {
							super::push(out, 1094);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 667);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 668);
					match s {
						[] => true,
						['a', 'z', 'e', s @ ..] => {
							super::push(out, 1096);
							s.is_empty()
						}
						['l', 'e', 'i', 'w', '\u{1}', s @ ..] => {
							super::push(out, 1097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1447);
									s.is_empty()
								}
								['z', '\u{1}', s @ ..] => {
									super::push(out, 1448);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'o', '\u{1}', s @ ..] => {
							super::push(out, 1098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'ł', s @ ..] => {
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 671);
					s.is_empty()
				}
				['s', 'u', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ó', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1099);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1100);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 283);
			match s {
				[] => true,
				['i', 's', 'k', '\u{1}', s @ ..] => {
					super::push(out, 673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 674);
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1102);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 1103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 675);
					s.is_empty()
				}
				['t', 'o', 'p', '\u{1}', s @ ..] => {
					super::push(out, 676);
					s.is_empty()
				}
				['ł', 'o', s @ ..] => {
					super::push(out, 677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nisty.
pub(crate) fn n859(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['e', 'i', 'c', s @ ..] => {
			super::push(out, 1127);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1128);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 1129);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęsisty.
pub(crate) fn n861(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 708);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1130);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1131);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -wisty.
pub(crate) fn n862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 709);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1455);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1456);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 1132);
			match s {
				[] => true,
				['z', 'c', 'e', 'z', 'r', 'e', s @ ..] => {
					super::push(out, 1457);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -zisty.
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 1458);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 1134);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -usty.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	match s {
		[] => true,
		['ł', 't', '\u{1}', s @ ..] => {
			super::push(out, 710);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ysty.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['z', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1686);
											s.is_empty()
										}
										['ź', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 1687);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 1629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'a', 'i', s @ ..] => {
							super::push(out, 1462);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						['j', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 1464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', 'm', s @ ..] => {
			super::push(out, 711);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęsty.
pub(crate) fn n359(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['g', 'e', s @ ..] => {
			super::push(out, 712);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 713);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uty.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 714);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 715);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yty.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['r', 'k', 's', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ąty.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	s.is_empty()
}

/// Words ending -ęty.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 716);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 717);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 718);
					match s {
						[] => true,
						['w', 'a', s @ ..] => {
							super::push(out, 1136);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -łty.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['ó', 'ż', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 719);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 720);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -liwy.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			match s {
				[] => true,
				['e', 'j', 'o', 't', 'ę', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 1144);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['n', '\u{1}', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1632);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1633);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1634);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1635);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1636);
									s.is_empty()
								}
								['ą', 'i', 'w', 'ś', '\u{1}', s @ ..] => {
									super::push(out, 1637);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['j', 'z', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1472);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1146);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1147);
					match s {
						[] => true,
						['h', 'c', '\u{1}', s @ ..] => {
							super::push(out, 1474);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1475);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1148);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1149);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1150);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['ę', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1477);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1478);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1152);
					s.is_empty()
				}
				['ą', 'i', 'c', s @ ..] => {
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 1479);
							s.is_empty()
						}
						['ś', s @ ..] => {
							super::push(out, 1480);
							s.is_empty()
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
				['a', s @ ..] => {
					super::push(out, 1153);
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['k', s @ ..] => {
									super::push(out, 1638);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 1639);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'r', 'e', s @ ..] => {
							super::push(out, 1482);
							s.is_empty()
						}
						['y', 'r', 'e', 'i', 'n', '\u{1}', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1484);
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
					super::push(out, 1154);
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1485);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'n', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1487);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 1155);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				['r', 'u', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1157);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1489);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1490);
							s.is_empty()
						}
						['y', s @ ..] => {
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 1640);
									s.is_empty()
								}
								['ł', s @ ..] => {
									super::push(out, 1641);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1159);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1160);
					s.is_empty()
				}
				['ą', s @ ..] => {
					super::push(out, 1161);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 1162);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1163);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'a', 'ł', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1164);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1166);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1167);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1168);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 1169);
					s.is_empty()
				}
				['t', 'ą', 'w', 'e', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1171);
					s.is_empty()
				}
				['ę', 't', 's', 'u', 'e', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1174);
					s.is_empty()
				}
				['o', 'g', 'd', s @ ..] => {
					super::push(out, 1175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 727);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1176);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1491);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 1492);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1493);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1494);
							s.is_empty()
						}
						['p', 'o', 'ł', 'k', '\u{1}', s @ ..] => {
							super::push(out, 1495);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1496);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 1497);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1498);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 1177);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1178);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1179);
					s.is_empty()
				}
				['ó', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				['ę', s @ ..] => {
					super::push(out, 1181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['o', 'm', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['r', 'a', 'b', '\u{1}', s @ ..] => {
					super::push(out, 1183);
					s.is_empty()
				}
				['ó', 'm', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 1184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 728);
			s.is_empty()
		}
		['ś', s @ ..] => super::t0::n895(s, out),
		['ź', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['a', 'r', 'b', s @ ..] => {
					super::push(out, 1189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => super::t0::n897(s, out),
		_ => false,
	}
}

/// Words ending -ziwy.
pub(crate) fn n376(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 323);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 1504);
							s.is_empty()
						}
						['r', 'u', '\u{1}', s @ ..] => {
							super::push(out, 1505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 1191);
					s.is_empty()
				}
				['ę', 's', '\u{1}', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -owy.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 324);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['c', 'ś', 'o', 't', s @ ..] => {
					super::push(out, 730);
					match s {
						[] => true,
						['r', 'a', 'w', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1506);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 731);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1193);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1508);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 1509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 733);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 1194);
							match s {
								[] => true,
								['z', s @ ..] => {
									super::push(out, 1510);
									match s {
										[] => true,
										['u', 't', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1689);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1690);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', '\u{1}', s @ ..] => {
			super::push(out, 327);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 734);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 735);
					match s {
						[] => true,
						['z', s @ ..] => {
							super::push(out, 1195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1511);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1512);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 736);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 737);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 738);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 1196);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1513);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1514);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 1197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 329);
			s.is_empty()
		}
		['t', 'o', s @ ..] => {
			super::push(out, 330);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 740);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1198);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1199);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 741);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 1200);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1515);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1516);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 1201);
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 742);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 743);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['ó', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1517);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -twy.
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['a', 'ł', 'e', s @ ..] => {
			super::push(out, 333);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 334);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 335);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 336);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ywy.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 745);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1202);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 1203);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1204);
							s.is_empty()
						}
						['r', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1519);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1520);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['s', 'r', 'a', 'p', '\u{1}', s @ ..] => {
					super::push(out, 747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 748);
					s.is_empty()
				}
				['l', 'e', 'b', 'o', '\u{1}', s @ ..] => {
					super::push(out, 749);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 750);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -kły.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 353);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1524);
									s.is_empty()
								}
								['ś', s @ ..] => {
									super::push(out, 1525);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'w', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1226);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'n', '\u{1}', s @ ..] => {
			super::push(out, 355);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 356);
			s.is_empty()
		}
		['u', 'm', s @ ..] => {
			super::push(out, 357);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 358);
			match s {
				[] => true,
				['w', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1227);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ę', s @ ..] => {
			super::push(out, 359);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 360);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pły.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 768);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 769);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 363);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sły.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 364);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['ś', 'e', s @ ..] => {
							super::push(out, 1232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['w', s @ ..] => {
					super::push(out, 771);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 365);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 772);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1233);
							s.is_empty()
						}
						['o', 'd', s @ ..] => {
							super::push(out, 1234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tły.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 366);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 367);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uły.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['z', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 774);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yły.
pub(crate) fn n143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['ż', s @ ..] => {
			super::push(out, 368);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zły.
pub(crate) fn n144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 369);
			s.is_empty()
		}
		['a', 'l', 'z', 'o', 'r', '\u{1}', s @ ..] => {
			super::push(out, 370);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 371);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 372);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 373);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 374);
			s.is_empty()
		}
		['ę', s @ ..] => {
			super::push(out, 375);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 376);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cudny.
pub(crate) fn n516(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 825);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ludny.
pub(crate) fn n518(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 827);
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 829);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mudny.
pub(crate) fn n519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	s.is_empty()
}

/// Words ending -nudny.
pub(crate) fn n520(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['y', s @ ..] => {
			super::push(out, 832);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rudny.
pub(crate) fn n521(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 834);
			s.is_empty()
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 1260);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 835);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łudny.
pub(crate) fn n522(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 836);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 837);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ydny.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	s.is_empty()
}

/// Words ending -zdny.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	s.is_empty()
}

/// Words ending -ądny.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 211);
	match s {
		[] => true,
		['s', s @ ..] => {
			super::push(out, 443);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 839);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 840);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 1261);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ż', s @ ..] => {
			super::push(out, 444);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ędny.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 212);
	match s {
		[] => true,
		['b', 'z', 'e', s @ ..] => {
			super::push(out, 445);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 446);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1263);
							s.is_empty()
						}
						['z', 'w', 'z', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 447);
			s.is_empty()
		}
		['z', s @ ..] => {
			match s {
				[] => true,
				['c', 'z', 's', 'o', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1267);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', 'b', '\u{1}', s @ ..] => {
			super::push(out, 448);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -etolerancyjny.
pub(crate) fn n2007(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1693);
	s.is_empty()
}

/// Words ending -utalny.
pub(crate) fn n1062(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 870);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 1297);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1298);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tualny.
pub(crate) fn n566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 478);
	match s {
		[] => true,
		['k', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					super::push(out, 1299);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1300);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1301);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -elny.
pub(crate) fn n240(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	match s {
		[] => true,
		['c', '\u{1}', s @ ..] => {
			super::push(out, 481);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 482);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 872);
					s.is_empty()
				}
				['z', 'd', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1302);
							s.is_empty()
						}
						['o', 'm', 'a', 's', '\u{1}', s @ ..] => {
							super::push(out, 1303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 874);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 875);
					s.is_empty()
				}
				['e', 'z', 'r', '\u{1}', s @ ..] => {
					super::push(out, 876);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 877);
					s.is_empty()
				}
				['r', 'e', 'i', 'm', 'ś', '\u{1}', s @ ..] => {
					super::push(out, 878);
					s.is_empty()
				}
				['y', 'z', 'c', '\u{1}', s @ ..] => {
					super::push(out, 879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'c', 'z', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 880);
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1304);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 1305);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ilny.
pub(crate) fn n241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 484);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							super::push(out, 1306);
							s.is_empty()
						}
						['t', 's', s @ ..] => {
							super::push(out, 1307);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1552);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1553);
									s.is_empty()
								}
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
			super::push(out, 485);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 486);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 487);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['z', s @ ..] => {
					super::push(out, 881);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'a', s @ ..] => {
			super::push(out, 489);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -olny.
pub(crate) fn n242(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 883);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', s @ ..] => {
			super::push(out, 490);
			s.is_empty()
		}
		['m', 'a', s @ ..] => {
			super::push(out, 491);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 492);
			s.is_empty()
		}
		['w', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 884);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					super::push(out, 885);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'o', 'm', '\u{1}', s @ ..] => {
			super::push(out, 493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ulny.
pub(crate) fn n243(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 227);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 494);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 495);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 496);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ylny.
pub(crate) fn n244(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 228);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 497);
			match s {
				[] => true,
				['c', 'y', 'z', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1309);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', '\u{1}', s @ ..] => {
			super::push(out, 498);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['n', 'a', 'k', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['r', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 1311);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ólny.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 229);
	match s {
		[] => true,
		['g', s @ ..] => {
			match s {
				[] => true,
				['e', 'z', 'c', 'z', 's', '\u{1}', s @ ..] => {
					super::push(out, 888);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 889);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 500);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ślny.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 230);
	match s {
		[] => true,
		['y', 'm', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 1312);
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							super::push(out, 1313);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 891);
					s.is_empty()
				}
				['y', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				['z', '\u{1}', s @ ..] => {
					super::push(out, 893);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -odporny.
pub(crate) fn n1165(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1350);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1351);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oporny.
pub(crate) fn n1166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 958);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 1353);
			s.is_empty()
		}
		['z', 'e', 'b', '\u{1}', s @ ..] => {
			super::push(out, 1354);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tworny.
pub(crate) fn n663(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 555);
	match s {
		[] => true,
		['s', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 959);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zorny.
pub(crate) fn n664(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 960);
			s.is_empty()
		}
		['e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
			super::push(out, 961);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 962);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -urny.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 556);
			s.is_empty()
		}
		['j', s @ ..] => {
			super::push(out, 557);
			s.is_empty()
		}
		['k', s @ ..] => {
			super::push(out, 558);
			s.is_empty()
		}
		['m', 'h', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 963);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 964);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 965);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 559);
			s.is_empty()
		}
		['t', 'l', 'u', 'k', '\u{1}', s @ ..] => {
			super::push(out, 560);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 561);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -órny.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	s.is_empty()
}

/// Words ending -ętny.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 262);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 612);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['c', 'e', 'z', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 998);
					s.is_empty()
				}
				['m', 'a', 'n', '\u{1}', s @ ..] => {
					super::push(out, 999);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['j', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1001);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1002);
					s.is_empty()
				}
				['o', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1370);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 613);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 614);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['k', s @ ..] => {
					super::push(out, 1003);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', 'r', s @ ..] => {
			super::push(out, 615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -esowny.
pub(crate) fn n1252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1032);
	s.is_empty()
}

/// Words ending -nsensowny.
pub(crate) fn n1253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1033);
	s.is_empty()
}

/// Words ending -osowny.
pub(crate) fn n1254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1034);
	s.is_empty()
}

/// Words ending -towny.
pub(crate) fn n760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 629);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 1035);
			s.is_empty()
		}
		['k', s @ ..] => {
			match s {
				[] => true,
				['a', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1571);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'f', 'e', s @ ..] => {
					super::push(out, 1384);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1385);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 1386);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 1387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1036);
			s.is_empty()
		}
		['z', 's', s @ ..] => {
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				['o', 'k', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1573);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 1574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ł', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -zowny.
pub(crate) fn n761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 630);
	s.is_empty()
}

/// Words ending -łowny.
pub(crate) fn n762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 631);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1389);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1390);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word awantażowny.
pub(crate) fn n763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 632);
	s.is_empty()
}

/// Words ending -barwny.
pub(crate) fn n299(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 633);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 634);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ywny.
pub(crate) fn n300(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 265);
	match s {
		[] => true,
		['s', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1038);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1391);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1392);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', 'g', s @ ..] => {
					super::push(out, 1039);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1393);
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
					super::push(out, 1040);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 1394);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 1575);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 1041);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 1395);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 1396);
							s.is_empty()
						}
						_ => false,
					}
				}
				['k', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1397);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1577);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1578);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1579);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 1580);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 1661);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 1581);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 1398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'z', s @ ..] => {
					super::push(out, 1042);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1582);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1583);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 635);
			s.is_empty()
		}
		['ż', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1044);
					s.is_empty()
				}
				['o', 'p', s @ ..] => {
					super::push(out, 1045);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ówny.
pub(crate) fn n301(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1046);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1047);
					s.is_empty()
				}
				['y', 'w', '\u{1}', s @ ..] => {
					super::push(out, 1048);
					s.is_empty()
				}
				['z', 'o', 'r', '\u{1}', s @ ..] => {
					super::push(out, 1049);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', '\u{1}', s @ ..] => {
			super::push(out, 637);
			s.is_empty()
		}
		['ł', 'g', '\u{1}', s @ ..] => {
			super::push(out, 638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -stateczny.
pub(crate) fn n1684(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1586);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 1587);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word wszeteczny.
pub(crate) fn n1685(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1404);
	s.is_empty()
}

/// Words ending -oteczny.
pub(crate) fn n1686(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1405);
	s.is_empty()
}

/// The word skuteczny.
pub(crate) fn n1687(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1406);
	s.is_empty()
}

/// Words ending -yteczny.
pub(crate) fn n1688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1407);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 1588);
			s.is_empty()
		}
		['ż', s @ ..] => {
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 1662);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1694);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ąteczny.
pub(crate) fn n1689(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1408);
	s.is_empty()
}

/// Words ending -rzeczny.
pub(crate) fn n1287(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['g', s @ ..] => {
			super::push(out, 1409);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1589);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 1590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 1410);
			match s {
				[] => true,
				['d', 'e', s @ ..] => {
					super::push(out, 1591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 1411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -łeczny.
pub(crate) fn n1288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1060);
	s.is_empty()
}

/// Words ending -iczny.
pub(crate) fn n777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 643);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		['g', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 1592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['l', 'o', 'k', s @ ..] => {
							super::push(out, 1593);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 1663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['e', 'n', s @ ..] => {
							super::push(out, 1594);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 1664);
									s.is_empty()
								}
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
			super::push(out, 1062);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1415);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 1416);
					s.is_empty()
				}
				['e', 'i', s @ ..] => {
					super::push(out, 1417);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1418);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 1419);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 1596);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1665);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 1666);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 1597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ś', s @ ..] => {
					super::push(out, 1421);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			match s {
				[] => true,
				['a', 'n', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['y', 'd', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1667);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 1423);
					s.is_empty()
				}
				['o', 'k', s @ ..] => {
					super::push(out, 1424);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1425);
					s.is_empty()
				}
				['e', 'i', s @ ..] => {
					super::push(out, 1426);
					s.is_empty()
				}
				['o', 'k', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1598);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -oczny.
pub(crate) fn n778(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', 'm', s @ ..] => {
			super::push(out, 1063);
			s.is_empty()
		}
		['ł', s @ ..] => {
			super::push(out, 1064);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 1428);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uczny.
pub(crate) fn n779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 644);
	match s {
		[] => true,
		['h', s @ ..] => {
			super::push(out, 1065);
			s.is_empty()
		}
		['t', 'z', s @ ..] => {
			super::push(out, 1066);
			s.is_empty()
		}
		['z', s @ ..] => {
			super::push(out, 1067);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yczny.
pub(crate) fn n780(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 645);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 1068);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 1069);
			match s {
				[] => true,
				['t', 'n', s @ ..] => {
					super::push(out, 1429);
					match s {
						[] => true,
						['e', 'c', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 1669);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 1670);
									s.is_empty()
								}
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
			super::push(out, 1070);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 1430);
					s.is_empty()
				}
				['k', s @ ..] => {
					super::push(out, 1431);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['m', 'e', s @ ..] => {
							super::push(out, 1600);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									super::push(out, 1671);
									match s {
										[] => true,
										['y', 's', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 1703);
													s.is_empty()
												}
												['e', s @ ..] => {
													super::push(out, 1704);
													s.is_empty()
												}
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
							super::push(out, 1601);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 1672);
									s.is_empty()
								}
								['m', 'y', 's', '\u{1}', s @ ..] => {
									super::push(out, 1673);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 1674);
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 's', 'e', s @ ..] => {
							super::push(out, 1602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 1603);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 1675);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 1676);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 1604);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1696);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1697);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 's', s @ ..] => {
							super::push(out, 1605);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 1677);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'a', 'r', s @ ..] => {
					super::push(out, 1432);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 1433);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 1678);
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 1698);
											s.is_empty()
										}
										['o', 'r', '\u{1}', s @ ..] => {
											super::push(out, 1699);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', 't', s @ ..] => {
							super::push(out, 1606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 1434);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 1608);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						['z', s @ ..] => {
							super::push(out, 1611);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 1435);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 1436);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 1612);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 1679);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 1680);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 1700);
											s.is_empty()
										}
										['f', s @ ..] => {
											super::push(out, 1701);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 1681);
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', 'a', 'e', s @ ..] => {
									super::push(out, 1682);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											super::push(out, 1702);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 1683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['y', s @ ..] => {
							super::push(out, 1613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 1437);
					s.is_empty()
				}
				_ => false,
			}
		}
		['z', s @ ..] => {
			super::push(out, 1071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ęczny.
pub(crate) fn n781(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 646);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 1438);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1439);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['o', 'p', s @ ..] => {
					super::push(out, 1440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1614);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1615);
							s.is_empty()
						}
						_ => false,
					}
				}
				['z', s @ ..] => {
					super::push(out, 1441);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1616);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 1617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dzny.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	s.is_empty()
}

/// Words ending -rzny.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['p', s @ ..] => {
			super::push(out, 647);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 648);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -szny.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 649);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 650);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 1442);
							s.is_empty()
						}
						['m', 'ś', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1618);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									super::push(out, 1620);
									s.is_empty()
								}
								['ś', '\u{1}', s @ ..] => {
									super::push(out, 1621);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['z', 'r', 'g', '\u{1}', s @ ..] => {
					super::push(out, 1072);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 651);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 652);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 653);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 1073);
					s.is_empty()
				}
				['ł', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1622);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 1623);
									s.is_empty()
								}
								['o', 'p', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1684);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 1685);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 654);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -yzny.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	s.is_empty()
}

/// Words ending -śliwy.
pub(crate) fn n895(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 1185);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 1186);
			s.is_empty()
		}
		['o', 'ł', 'z', 'e', s @ ..] => {
			super::push(out, 1187);
			s.is_empty()
		}
		['ą', s @ ..] => {
			super::push(out, 1188);
			s.is_empty()
		}
		['ę', 'z', 'c', 'z', 's', 'e', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 1499);
					s.is_empty()
				}
				['z', s @ ..] => {
					super::push(out, 1500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -żliwy.
pub(crate) fn n897(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						['w', s @ ..] => {
							super::push(out, 1643);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1688);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ł', s @ ..] => {
					super::push(out, 1501);
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			match s {
				[] => true,
				['m', 'e', s @ ..] => {
					super::push(out, 1502);
					s.is_empty()
				}
				['w', s @ ..] => {
					super::push(out, 1503);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ą', s @ ..] => {
			super::push(out, 1190);
			s.is_empty()
		}
		_ => false,
	}
}

