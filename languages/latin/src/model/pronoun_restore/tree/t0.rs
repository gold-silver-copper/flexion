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
			s.is_empty()
		}
		['d', s @ ..] => {
			super::push(out, 3);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 4);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 14);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 26);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 27);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 28);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 16);
					match s {
						[] => true,
						['q', s @ ..] => {
							match s {
								[] => true,
								['m', s @ ..] => {
									super::push(out, 41);
									match s {
										[] => true,
										['u', 'c', s @ ..] => {
											match s {
												[] => true,
												['i', s @ ..] => {
													super::push(out, 67);
													s.is_empty()
												}
												['r', s @ ..] => {
													super::push(out, 68);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 69);
													match s {
														[] => true,
														['i', s @ ..] => super::t0::n81(s, out),
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
									super::push(out, 42);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 43);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 44);
									match s {
										[] => true,
										['i', 'u', 'q', 's', s @ ..] => {
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 17);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 18);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 19);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 29);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 30);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 31);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 45);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 46);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 32);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 20);
					match s {
						[] => true,
						['d', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 21);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 22);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 8);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 23);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 24);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 33);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 34);
							match s {
								[] => true,
								['q', s @ ..] => {
									match s {
										[] => true,
										['c', s @ ..] => {
											super::push(out, 57);
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 58);
											s.is_empty()
										}
										['i', s @ ..] => {
											super::push(out, 59);
											match s {
												[] => true,
												['l', s @ ..] => {
													super::push(out, 70);
													s.is_empty()
												}
												['s', s @ ..] => {
													super::push(out, 71);
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 60);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 61);
											s.is_empty()
										}
										['s', s @ ..] => {
											super::push(out, 62);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 35);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 49);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 50);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 51);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 25);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 36);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 37);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 38);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 10);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => super::t0::n42(s, out),
						['m', s @ ..] => super::t0::n43(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iscumque.
pub(crate) fn n81(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['l', s @ ..] => {
			super::push(out, 75);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 76);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -bet.
pub(crate) fn n42(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['i', 'l', s @ ..] => {
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 63);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 64);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 52);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -met.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 65);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 72);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 73);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 66);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 74);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 54);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 55);
			s.is_empty()
		}
		_ => false,
	}
}

