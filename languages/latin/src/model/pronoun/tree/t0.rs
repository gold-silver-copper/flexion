use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 11);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 31);
							s.is_empty()
						}
						['\u{12}', s @ ..] => {
							super::push(out, 32);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 33);
							s.is_empty()
						}
						['\u{12}', s @ ..] => {
							super::push(out, 34);
							match s {
								[] => true,
								['u', 'q', 'c', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 56);
									s.is_empty()
								}
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
			super::push(out, 4);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 14);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 16);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 17);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 35);
							s.is_empty()
						}
						['\u{12}', s @ ..] => {
							super::push(out, 36);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 57);
									s.is_empty()
								}
								['p', s @ ..] => {
									super::push(out, 58);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 18);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 19);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 37);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							super::push(out, 38);
							match s {
								[] => true,
								['q', s @ ..] => {
									match s {
										[] => true,
										['m', s @ ..] => {
											super::push(out, 82);
											match s {
												[] => true,
												['u', 'c', s @ ..] => {
													match s {
														[] => true,
														['i', s @ ..] => super::t0::n121(s, out),
														['r', s @ ..] => super::t0::n122(s, out),
														['s', s @ ..] => super::t0::n123(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 83);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 84);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 85);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 5);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 20);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 21);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 22);
					match s {
						[] => true,
						['\u{11}', s @ ..] => {
							super::push(out, 39);
							match s {
								[] => true,
								['q', s @ ..] => {
									super::push(out, 59);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 60);
									s.is_empty()
								}
								_ => false,
							}
						}
						['\u{12}', s @ ..] => {
							super::push(out, 40);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							super::push(out, 41);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 23);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 42);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 61);
									match s {
										[] => true,
										['i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 86);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 62);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 63);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 64);
									match s {
										[] => true,
										['q', 's', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 87);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['\u{13}', s @ ..] => {
							super::push(out, 43);
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 65);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 88);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 89);
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 66);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 24);
					match s {
						[] => true,
						['\u{12}', s @ ..] => {
							super::push(out, 44);
							s.is_empty()
						}
						['\u{13}', s @ ..] => {
							super::push(out, 45);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 25);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 26);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8);
			match s {
				[] => true,
				['e', '\u{12}', 't', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 46);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 67);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 47);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 48);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => super::t0::n9(s, out),
		['t', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['e', s @ ..] => super::t0::n33(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -icumque (3 syllables).
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	s.is_empty()
}

/// Words ending -rcumque (3 syllables).
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	s.is_empty()
}

/// Words ending -scumque (3 syllables).
pub(crate) fn n123(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 116);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 117);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -s.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['\u{13}', 'r', 'u', 'l', 'p', s @ ..] => {
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 68);
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 69);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 28);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 49);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 70);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 50);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 71);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 72);
							match s {
								[] => true,
								['q', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 97);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 98);
											match s {
												[] => true,
												['n', '\u{1}', s @ ..] => {
													super::push(out, 114);
													s.is_empty()
												}
												_ => false,
											}
										}
										['i', s @ ..] => {
											super::push(out, 99);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 100);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 101);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 73);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 90);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 91);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['\u{13}', s @ ..] => {
					super::push(out, 51);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 74);
							match s {
								[] => true,
								['q', s @ ..] => {
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 102);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 103);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 75);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 29);
			match s {
				[] => true,
				['\u{11}', s @ ..] => {
					super::push(out, 52);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 76);
							match s {
								[] => true,
								['m', '\u{1}', s @ ..] => {
									super::push(out, 92);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 77);
							s.is_empty()
						}
						_ => false,
					}
				}
				['\u{12}', s @ ..] => {
					super::push(out, 53);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 78);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 79);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -et.
pub(crate) fn n33(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['\u{12}', s @ ..] => {
			super::push(out, 54);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 93);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 104);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 105);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 94);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 80);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 106);
									match s {
										[] => true,
										['i', 'u', 'q', 'c', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 115);
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 107);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 81);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 95);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 109);
									s.is_empty()
								}
								['t', s @ ..] => {
									super::push(out, 110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 96);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

