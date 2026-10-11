use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['a', 'l', 'l', 'u', 'n', '\u{1}', s @ ..] => {
			super::push(out, 1);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['l', 'l', 'i', 'm', 'i', 'u', 'q', 's', 'e', 's', '\u{1}', s @ ..] => {
					super::push(out, 6);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['t', 'n', 'e', 'g', 'n', s @ ..] => {
					match s {
						[] => true,
						['i', 'u', 'q', '\u{1}', s @ ..] => {
							super::push(out, 8);
							s.is_empty()
						}
						['o', 'n', '\u{1}', s @ ..] => {
							super::push(out, 9);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', 'i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
			super::push(out, 4);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['e', 'r', 't', '\u{1}', s @ ..] => {
					super::push(out, 7);
					s.is_empty()
				}
				['u', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 10);
							match s {
								[] => true,
								['p', 'u', s @ ..] => {
									match s {
										[] => true,
										['c', 'e', 'g', 'i', 'v', '\u{1}', s @ ..] => {
											super::push(out, 22);
											s.is_empty()
										}
										['n', 'o', 'n', '\u{1}', s @ ..] => {
											super::push(out, 23);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 11);
							match s {
								[] => true,
								['i', s @ ..] => super::t0::n18(s, out),
								_ => false,
							}
						}
						['n', s @ ..] => super::t0::n14(s, out),
						['t', s @ ..] => {
							super::push(out, 13);
							match s {
								[] => true,
								['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 20);
									s.is_empty()
								}
								['r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 21);
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', 'a', 't', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 14);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -imus.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					match s {
						[] => true,
						['n', 'u', '\u{1}', s @ ..] => {
							super::push(out, 35);
							s.is_empty()
						}
						['s', 'u', s @ ..] => {
							match s {
								[] => true,
								['n', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 45);
									s.is_empty()
								}
								['t', s @ ..] => {
									match s {
										[] => true,
										['n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 50);
											s.is_empty()
										}
										['r', 'a', 'u', 'q', '\u{1}', s @ ..] => {
											super::push(out, 51);
											s.is_empty()
										}
										_ => false,
									}
								}
								['v', 'a', 't', 'c', 'o', '\u{1}', s @ ..] => {
									super::push(out, 46);
									s.is_empty()
								}
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
			super::push(out, 25);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 36);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['r', 't', 'e', 'd', s @ ..] => {
											match s {
												[] => true,
												['n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 57);
													s.is_empty()
												}
												['o', 'u', 'd', '\u{1}', s @ ..] => {
													super::push(out, 58);
													s.is_empty()
												}
												_ => false,
											}
										}
										['v', s @ ..] => {
											match s {
												[] => true,
												['e', 'd', s @ ..] => {
													match s {
														[] => true,
														['n', 'u', '\u{1}', s @ ..] => super::t0::n86(s, out),
														['o', 'u', 'd', '\u{1}', s @ ..] => super::t0::n87(s, out),
														_ => false,
													}
												}
												['t', 'e', 'n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 59);
													s.is_empty()
												}
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
							super::push(out, 37);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['n', s @ ..] => {
											super::push(out, 52);
											match s {
												[] => true,
												['o', 'n', 'e', 'd', 'o', 'u', 'd', '\u{1}', s @ ..] => {
													super::push(out, 60);
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'd', 'a', 'u', 'q', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
											super::push(out, 53);
											s.is_empty()
										}
										['u', s @ ..] => {
											super::push(out, 54);
											match s {
												[] => true,
												['q', 'n', 'i', 'u', 'q', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n88(s, out),
														['e', 'd', s @ ..] => super::t0::n89(s, out),
														_ => false,
													}
												}
												['t', 'p', 'e', 's', 'e', 'd', s @ ..] => {
													match s {
														[] => true,
														['n', 'u', '\u{1}', s @ ..] => super::t0::n90(s, out),
														['o', 'u', 'd', '\u{1}', s @ ..] => super::t0::n91(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['x', 'e', 's', 'e', s @ ..] => {
											super::push(out, 55);
											match s {
												[] => true,
												['d', 'n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 61);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 47);
									match s {
										[] => true,
										['t', 'c', 'o', 'e', 'd', 'o', 'u', 'd', '\u{1}', s @ ..] => {
											super::push(out, 56);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 38);
							match s {
								[] => true,
								['l', 'i', 'm', 'e', 'd', 'n', 'u', '\u{1}', s @ ..] => {
									super::push(out, 48);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'i', 'l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
							super::push(out, 39);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 40);
							match s {
								[] => true,
								['i', 'n', 'i', 'f', 'n', 'i', '\u{1}', s @ ..] => {
									super::push(out, 49);
									s.is_empty()
								}
								['n', 'e', s @ ..] => {
									match s {
										[] => true,
										['c', 'e', 'd', s @ ..] => {
											match s {
												[] => true,
												['n', 'u', '\u{1}', s @ ..] => {
													super::push(out, 62);
													s.is_empty()
												}
												['o', 'u', 'd', '\u{1}', s @ ..] => {
													super::push(out, 63);
													s.is_empty()
												}
												_ => false,
											}
										}
										['g', 'n', s @ ..] => {
											match s {
												[] => true,
												['i', 'u', 'q', '\u{1}', s @ ..] => {
													super::push(out, 64);
													s.is_empty()
												}
												['o', 'n', '\u{1}', s @ ..] => {
													super::push(out, 65);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 30);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nus.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 15);
			match s {
				[] => true,
				['c', 'i', s @ ..] => {
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							super::push(out, 31);
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							super::push(out, 32);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'n', 'u', '\u{1}', s @ ..] => {
					super::push(out, 26);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 27);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 33);
							match s {
								[] => true,
								['n', 'o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 41);
									s.is_empty()
								}
								['u', 'q', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 42);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', 'u', 'q', '\u{1}', s @ ..] => {
									super::push(out, 43);
									s.is_empty()
								}
								['o', 'n', '\u{1}', s @ ..] => {
									super::push(out, 44);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 't', 'c', 'o', '\u{1}', s @ ..] => {
							super::push(out, 34);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'l', 'i', 'm', '\u{1}', s @ ..] => {
					super::push(out, 28);
					s.is_empty()
				}
				['t', 'n', 'e', 'g', 'n', 'i', 'u', 'q', '\u{1}', s @ ..] => {
					super::push(out, 29);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 16);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 17);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 18);
			s.is_empty()
		}
		['u', '\u{1}', s @ ..] => {
			super::push(out, 19);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word undevicesimus.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	s.is_empty()
}

/// The word duodevicesimus.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	s.is_empty()
}

/// The word quinquagesimus.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 68);
	s.is_empty()
}

/// Words ending -dequinquagesimus.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['n', 'u', '\u{1}', s @ ..] => {
			super::push(out, 71);
			s.is_empty()
		}
		['o', 'u', 'd', '\u{1}', s @ ..] => {
			super::push(out, 72);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word undeseptuagesimus.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 69);
	s.is_empty()
}

/// The word duodeseptuagesimus.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	s.is_empty()
}

