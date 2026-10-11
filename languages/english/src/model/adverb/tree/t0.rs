use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 18);
							s.is_empty()
						}
						['o', 'o', 'g', '\u{1}', s @ ..] => {
							super::push(out, 19);
							s.is_empty()
						}
						['r', 'a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 20);
							s.is_empty()
						}
						['u', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 21);
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', 'e', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 4);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', 'b', '\u{1}', s @ ..] => {
							super::push(out, 22);
							s.is_empty()
						}
						['n', 'o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 23);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					match s {
						[] => true,
						['c', 'u', 'm', '\u{1}', s @ ..] => {
							super::push(out, 24);
							s.is_empty()
						}
						['g', 'i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 25);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 26);
							match s {
								[] => true,
								['i', 'w', 's', '\u{1}', s @ ..] => {
									super::push(out, 64);
									s.is_empty()
								}
								['o', 'o', 'm', 's', '\u{1}', s @ ..] => {
									super::push(out, 65);
									s.is_empty()
								}
								['r', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 66);
									s.is_empty()
								}
								['u', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 67);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['k', 'c', s @ ..] => {
					match s {
						[] => true,
						['a', 'b', '\u{1}', s @ ..] => {
							super::push(out, 27);
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['h', 't', '\u{1}', s @ ..] => {
									super::push(out, 68);
									s.is_empty()
								}
								['u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 69);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 5);
					match s {
						[] => true,
						['a', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 28);
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', 'm', 's', '\u{1}', s @ ..] => {
									super::push(out, 70);
									s.is_empty()
								}
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 71);
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									super::push(out, 72);
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
						['a', 'e', 'l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 29);
							s.is_empty()
						}
						['i', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 30);
							s.is_empty()
						}
						['o', 'o', 's', '\u{1}', s @ ..] => {
							super::push(out, 31);
							s.is_empty()
						}
						['w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 32);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['e', 'e', 'd', '\u{1}', s @ ..] => {
							super::push(out, 33);
							s.is_empty()
						}
						['r', 'a', 'h', 's', '\u{1}', s @ ..] => {
							super::push(out, 34);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['e', 'n', '\u{1}', s @ ..] => {
							super::push(out, 35);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 36);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 73);
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									super::push(out, 74);
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							match s {
								[] => true,
								['c', 'i', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
									super::push(out, 75);
									s.is_empty()
								}
								['g', 'i', s @ ..] => {
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											super::push(out, 103);
											s.is_empty()
										}
										['t', '\u{1}', s @ ..] => {
											super::push(out, 104);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'a', 'r', 't', 's', '\u{1}', s @ ..] => {
							super::push(out, 37);
							s.is_empty()
						}
						['o', 'h', '\u{1}', s @ ..] => {
							super::push(out, 38);
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											super::push(out, 105);
											s.is_empty()
										}
										['f', '\u{1}', s @ ..] => {
											super::push(out, 106);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', 'w', '\u{1}', s @ ..] => {
									super::push(out, 76);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['w', 'o', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 39);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 40);
							s.is_empty()
						}
						_ => false,
					}
				}
				['y', 'a', s @ ..] => {
					match s {
						[] => true,
						['t', 's', '\u{1}', s @ ..] => {
							super::push(out, 41);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 42);
							s.is_empty()
						}
						['w', s @ ..] => {
							match s {
								[] => true,
								['k', '\u{1}', s @ ..] => {
									super::push(out, 77);
									s.is_empty()
								}
								['q', '\u{1}', s @ ..] => {
									super::push(out, 78);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => super::t0::n2(s, out),
		['\u{13}', s @ ..] => super::t0::n3(s, out),
		_ => false,
	}
}

/// Words ending - (2 syllables).
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['a', 'e', s @ ..] => {
					super::push(out, 43);
					match s {
						[] => true,
						['r', 'h', 't', s @ ..] => {
							match s {
								[] => true,
								['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 107);
									s.is_empty()
								}
								['p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'e', 'i', 'f', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 79);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 80);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', 'w', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 81);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 82);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['c', 'i', 'n', '\u{1}', s @ ..] => {
					super::push(out, 44);
					s.is_empty()
				}
				['d', 'i', 'w', '\u{1}', s @ ..] => {
					super::push(out, 45);
					s.is_empty()
				}
				['h', 't', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', '\u{1}', s @ ..] => {
							super::push(out, 83);
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							super::push(out, 84);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 't', 't', 'i', 'l', '\u{1}', s @ ..] => {
					super::push(out, 46);
					s.is_empty()
				}
				['n', 'i', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 47);
					s.is_empty()
				}
				['r', 'a', 's', '\u{1}', s @ ..] => {
					super::push(out, 48);
					s.is_empty()
				}
				['s', 'o', 'l', 'c', '\u{1}', s @ ..] => {
					super::push(out, 49);
					s.is_empty()
				}
				['t', 'a', 'l', '\u{1}', s @ ..] => {
					super::push(out, 50);
					s.is_empty()
				}
				_ => false,
			}
		}
		['k', 'e', 'e', 'r', 'c', 'p', 'u', '\u{1}', s @ ..] => {
			super::push(out, 8);
			s.is_empty()
		}
		['l', 'l', 'i', s @ ..] => {
			super::push(out, 9);
			match s {
				[] => true,
				['h', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 85);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 86);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['a', 'e', 'r', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 87);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 88);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 11);
			match s {
				[] => true,
				['w', 'o', 't', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 89);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 90);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			match s {
				[] => true,
				['d', 'r', 'a', 'w', 'r', 'o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 51);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['i', 'a', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 109);
									s.is_empty()
								}
								['p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 110);
									s.is_empty()
								}
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
			super::push(out, 12);
			match s {
				[] => true,
				['e', 'e', 'r', 't', 's', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 91);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 92);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'u', 'o', 'c', s @ ..] => {
					match s {
						[] => true,
						['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
							super::push(out, 93);
							s.is_empty()
						}
						['p', 'u', '\u{1}', s @ ..] => {
							super::push(out, 94);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['a', 'e', 'h', 't', s @ ..] => {
							match s {
								[] => true,
								['r', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 111);
									s.is_empty()
								}
								['u', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 112);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'w', 'h', 't', s @ ..] => {
							match s {
								[] => true,
								['r', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 113);
									s.is_empty()
								}
								['u', 'o', 's', '\u{1}', s @ ..] => {
									super::push(out, 114);
									s.is_empty()
								}
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
			super::push(out, 13);
			match s {
				[] => true,
				['l', s @ ..] => super::t0::n79(s, out),
				['n', 'n', 'u', 'f', '\u{1}', s @ ..] => {
					super::push(out, 53);
					s.is_empty()
				}
				['s', 'a', 'e', '\u{1}', s @ ..] => {
					super::push(out, 54);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending - (3 syllables).
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 14);
			match s {
				[] => true,
				['d', 'a', 'r', 'g', 'p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 55);
					s.is_empty()
				}
				['g', 'n', 'a', 'r', 'n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
					super::push(out, 56);
					s.is_empty()
				}
				['p', 'o', s @ ..] => {
					super::push(out, 57);
					match s {
						[] => true,
						['l', 's', s @ ..] => {
							match s {
								[] => true,
								['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 139);
									s.is_empty()
								}
								['p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 140);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', 'o', 'h', 's', 'p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 58);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['i', 'a', 't', 'n', 'u', 'o', 'm', 'p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 59);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 16);
			match s {
				[] => true,
				['e', 'v', 'i', 'r', 'p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 60);
					s.is_empty()
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 17);
			match s {
				[] => true,
				['e', 'l', 'l', 'a', 'v', 'p', 'u', '\u{1}', s @ ..] => {
					super::push(out, 61);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 62);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 98);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 141);
									match s {
										[] => true,
										['e', 'i', 'r', 'f', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 99);
							match s {
								[] => true,
								['c', s @ ..] => {
									match s {
										[] => true,
										['i', 'n', '\u{1}', s @ ..] => {
											super::push(out, 184);
											s.is_empty()
										}
										['r', 'e', 'i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 185);
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'i', 'w', '\u{1}', s @ ..] => {
									super::push(out, 142);
									s.is_empty()
								}
								['g', 'r', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 143);
									s.is_empty()
								}
								['m', 'i', 't', 'e', 'b', '\u{1}', s @ ..] => {
									super::push(out, 144);
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', 'f', '\u{1}', s @ ..] => {
											super::push(out, 186);
											s.is_empty()
										}
										['o', 'l', 'a', '\u{1}', s @ ..] => {
											super::push(out, 187);
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', '\u{1}', s @ ..] => {
											super::push(out, 188);
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											super::push(out, 189);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['i', 'w', '\u{1}', s @ ..] => {
											super::push(out, 190);
											s.is_empty()
										}
										['o', 'l', 'c', '\u{1}', s @ ..] => {
											super::push(out, 191);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'a', 'l', '\u{1}', s @ ..] => {
									super::push(out, 145);
									s.is_empty()
								}
								['v', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'b', '\u{1}', s @ ..] => {
											super::push(out, 192);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 193);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 100);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 146);
									match s {
										[] => true,
										['a', 'e', 'r', '\u{1}', s @ ..] => {
											super::push(out, 194);
											s.is_empty()
										}
										['e', 'e', 'p', 's', '\u{1}', s @ ..] => {
											super::push(out, 195);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 147);
									match s {
										[] => true,
										['a', 'e', '\u{1}', s @ ..] => {
											super::push(out, 196);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 101);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 148);
									match s {
										[] => true,
										['r', s @ ..] => {
											super::push(out, 197);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'i', 'a', 'g', 'n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 63);
					match s {
						[] => true,
						['t', 'n', 'u', 'o', 'c', s @ ..] => {
							match s {
								[] => true,
								['n', 'w', 'o', 'd', '\u{1}', s @ ..] => {
									super::push(out, 149);
									s.is_empty()
								}
								['p', 'u', '\u{1}', s @ ..] => {
									super::push(out, 150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ly (2 syllables).
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 151);
							s.is_empty()
						}
						['l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 152);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 153);
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							super::push(out, 154);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'e', 'n', '\u{1}', s @ ..] => {
					super::push(out, 115);
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 155);
							s.is_empty()
						}
						['o', 'b', '\u{1}', s @ ..] => {
							super::push(out, 156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['e', 'b', '\u{1}', s @ ..] => {
							super::push(out, 157);
							s.is_empty()
						}
						['i', 'k', '\u{1}', s @ ..] => {
							super::push(out, 158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'o', 'g', '\u{1}', s @ ..] => {
					super::push(out, 116);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', 'h', '\u{1}', s @ ..] => {
							super::push(out, 159);
							s.is_empty()
						}
						['o', 'l', '\u{1}', s @ ..] => {
							super::push(out, 160);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'o', 'r', 'p', '\u{1}', s @ ..] => {
					super::push(out, 117);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['e', 'r', s @ ..] => {
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							super::push(out, 161);
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							super::push(out, 162);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 118);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', 'n', 'o', 'r', 't', 's', '\u{1}', s @ ..] => {
			super::push(out, 95);
			s.is_empty()
		}
		['h', s @ ..] => {
			match s {
				[] => true,
				['c', 'i', 'r', '\u{1}', s @ ..] => {
					super::push(out, 119);
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['i', 'h', '\u{1}', s @ ..] => {
							super::push(out, 163);
							s.is_empty()
						}
						['u', 'o', 'r', '\u{1}', s @ ..] => {
							super::push(out, 164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'r', 'a', 'h', '\u{1}', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['o', 'l', '\u{1}', s @ ..] => {
									super::push(out, 198);
									s.is_empty()
								}
								['r', 'b', '\u{1}', s @ ..] => {
									super::push(out, 199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'o', s @ ..] => {
							match s {
								[] => true,
								['m', 's', '\u{1}', s @ ..] => {
									super::push(out, 200);
									s.is_empty()
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 201);
									s.is_empty()
								}
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
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					match s {
						[] => true,
						['h', 't', '\u{1}', s @ ..] => {
							super::push(out, 165);
							s.is_empty()
						}
						['u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 166);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', 'r', 'f', '\u{1}', s @ ..] => {
					super::push(out, 121);
					s.is_empty()
				}
				['r', 'a', 'd', '\u{1}', s @ ..] => {
					super::push(out, 122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			match s {
				[] => true,
				['e', 's', '\u{1}', s @ ..] => {
					super::push(out, 123);
					s.is_empty()
				}
				['i', 's', '\u{1}', s @ ..] => {
					super::push(out, 124);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'r', s @ ..] => {
			match s {
				[] => true,
				['a', 'w', '\u{1}', s @ ..] => {
					super::push(out, 125);
					s.is_empty()
				}
				['i', 'f', '\u{1}', s @ ..] => {
					super::push(out, 126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', s @ ..] => {
					match s {
						[] => true,
						['l', 'c', '\u{1}', s @ ..] => {
							super::push(out, 167);
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							super::push(out, 168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'e', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 127);
					s.is_empty()
				}
				['i', 'a', s @ ..] => {
					match s {
						[] => true,
						['g', '\u{1}', s @ ..] => {
							super::push(out, 169);
							s.is_empty()
						}
						['l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 170);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 'r', 'a', 'h', 's', '\u{1}', s @ ..] => {
			super::push(out, 96);
			s.is_empty()
		}
		['r', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 172);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', 'a', '\u{1}', s @ ..] => {
					super::push(out, 129);
					s.is_empty()
				}
				['u', 's', '\u{1}', s @ ..] => {
					super::push(out, 130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			match s {
				[] => true,
				['a', 'e', s @ ..] => {
					super::push(out, 131);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 174);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
							super::push(out, 175);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', 'i', 'r', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 132);
					s.is_empty()
				}
				['e', 'e', 'w', 's', '\u{1}', s @ ..] => {
					super::push(out, 133);
					s.is_empty()
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['i', 'w', 's', '\u{1}', s @ ..] => {
							super::push(out, 176);
							s.is_empty()
						}
						['o', 's', '\u{1}', s @ ..] => {
							super::push(out, 177);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', 'g', s @ ..] => {
					super::push(out, 134);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 204);
											s.is_empty()
										}
										['p', '\u{1}', s @ ..] => {
											super::push(out, 205);
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'k', '\u{1}', s @ ..] => {
									super::push(out, 202);
									s.is_empty()
								}
								['r', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 206);
											s.is_empty()
										}
										['b', '\u{1}', s @ ..] => {
											super::push(out, 207);
											s.is_empty()
										}
										['p', 's', '\u{1}', s @ ..] => {
											super::push(out, 208);
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', '\u{1}', s @ ..] => {
									super::push(out, 203);
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
						['a', 'l', 'p', '\u{1}', s @ ..] => {
							super::push(out, 178);
							s.is_empty()
						}
						['e', 'g', '\u{1}', s @ ..] => {
							super::push(out, 179);
							s.is_empty()
						}
						['i', 'a', 'f', '\u{1}', s @ ..] => {
							super::push(out, 180);
							s.is_empty()
						}
						['o', 'w', '\u{1}', s @ ..] => {
							super::push(out, 181);
							s.is_empty()
						}
						['u', 'a', '\u{1}', s @ ..] => {
							super::push(out, 182);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'a', 'e', 'h', '\u{1}', s @ ..] => {
					super::push(out, 135);
					s.is_empty()
				}
				['s', 'u', 'j', '\u{1}', s @ ..] => {
					super::push(out, 136);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', 'r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 137);
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					super::push(out, 138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['w', 'o', 'l', 's', '\u{1}', s @ ..] => {
			super::push(out, 97);
			s.is_empty()
		}
		_ => false,
	}
}

