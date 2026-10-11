use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['\u{11}', s @ ..] => {
			super::push(out, 1);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 4);
					match s {
						[] => true,
						['u', s @ ..] => {
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 46);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 47);
									s.is_empty()
								}
								['o', 'j', s @ ..] => {
									super::push(out, 48);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 5);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 7);
					match s {
						[] => true,
						['p', s @ ..] => {
							super::push(out, 22);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 49);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['é', s @ ..] => {
					super::push(out, 8);
					s.is_empty()
				}
				['û', s @ ..] => {
					super::push(out, 9);
					s.is_empty()
				}
				_ => false,
			}
		}
		['\u{12}', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 11);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 23);
							match s {
								[] => true,
								['n', 'o', s @ ..] => {
									super::push(out, 50);
									match s {
										[] => true,
										['b', s @ ..] => {
											super::push(out, 72);
											s.is_empty()
										}
										['l', s @ ..] => {
											super::push(out, 73);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 24);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									super::push(out, 51);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 25);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 52);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 53);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 26);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 27);
							match s {
								[] => true,
								['l', s @ ..] => {
									super::push(out, 54);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 55);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14);
					s.is_empty()
				}
				['é', s @ ..] => {
					super::push(out, 15);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 28);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 56);
									s.is_empty()
								}
								['l', s @ ..] => {
									super::push(out, 57);
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 29);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 58);
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 74);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 30);
							s.is_empty()
						}
						['k', s @ ..] => {
							super::push(out, 31);
							match s {
								[] => true,
								['r', s @ ..] => {
									super::push(out, 59);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'u', s @ ..] => {
							super::push(out, 32);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 60);
									match s {
										[] => true,
										['i', 'm', s @ ..] => {
											super::push(out, 75);
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 61);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 33);
							s.is_empty()
						}
						['r', 'r', s @ ..] => {
							super::push(out, 34);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									super::push(out, 62);
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 's', 'a', s @ ..] => {
							super::push(out, 35);
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 63);
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 76);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 77);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => {
							super::push(out, 36);
							match s {
								[] => true,
								['s', 'e', '\u{1}', s @ ..] => {
									super::push(out, 64);
									s.is_empty()
								}
								['t', 'a', s @ ..] => {
									super::push(out, 65);
									match s {
										[] => true,
										['h', s @ ..] => {
											super::push(out, 78);
											s.is_empty()
										}
										_ => false,
									}
								}
								['é', s @ ..] => {
									super::push(out, 66);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 79);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 37);
							s.is_empty()
						}
						_ => false,
					}
				}
				['û', s @ ..] => {
					super::push(out, 16);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 38);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 39);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['\u{13}', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 17);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 18);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 19);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 20);
					match s {
						[] => true,
						['n', s @ ..] => super::t0::n41(s, out),
						_ => false,
					}
				}
				['é', s @ ..] => super::t0::n21(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nu (3 syllables).
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['t', 't', s @ ..] => {
					super::push(out, 80);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 81);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -é (3 syllables).
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 41);
			match s {
				[] => true,
				['é', s @ ..] => {
					super::push(out, 67);
					match s {
						[] => true,
						['c', 'c', s @ ..] => {
							super::push(out, 82);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 88);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 89);
									s.is_empty()
								}
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
			super::push(out, 42);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 68);
					match s {
						[] => true,
						['o', 'n', s @ ..] => {
							super::push(out, 83);
							match s {
								[] => true,
								['g', s @ ..] => {
									super::push(out, 90);
									s.is_empty()
								}
								['â', s @ ..] => {
									super::push(out, 91);
									s.is_empty()
								}
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
			super::push(out, 44);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 69);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 70);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 84);
							match s {
								[] => true,
								['-', s @ ..] => {
									super::push(out, 92);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 93);
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 85);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 71);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 86);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 87);
							match s {
								[] => true,
								['s', s @ ..] => {
									super::push(out, 94);
									s.is_empty()
								}
								['x', s @ ..] => {
									super::push(out, 95);
									match s {
										[] => true,
										['e', s @ ..] => {
											match s {
												[] => true,
												['o', s @ ..] => {
													super::push(out, 96);
													s.is_empty()
												}
												['é', s @ ..] => {
													super::push(out, 97);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

