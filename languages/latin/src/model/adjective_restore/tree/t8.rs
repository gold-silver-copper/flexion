use flexion::Path;

/// Words ending -mendus.
pub(crate) fn n3456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2937);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9718);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9719);
					match s {
						[] => true,
						['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 5986);
			match s {
				[] => true,
				['r', 'p', 'e', s @ ..] => {
					super::push(out, 9720);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13094);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 13095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 5987);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9721);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9722);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15521);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9723);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13096);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 5988);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 13097);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 13098);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 13099);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15522);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13100);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15523);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 15524);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15525);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13101);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -nendus.
pub(crate) fn n3457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2938);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 5989);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 5990);
			match s {
				[] => true,
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 5991);
			match s {
				[] => true,
				['i', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13102);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 5992);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13103);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9725);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13104);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9726);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13105);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15526);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15527);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5993);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 5994);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 5995);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 9727);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13106);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15528);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13107);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => super::t10::n14810(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => super::t10::n8142(s, out),
		_ => false,
	}
}

/// Words ending -pendus.
pub(crate) fn n3458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2939);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 5997);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 9729);
					s.is_empty()
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 13118);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13119);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 13120);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13121);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 17076);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['l', s @ ..] => {
			super::push(out, 5998);
			match s {
				[] => true,
				['u', 'c', 's', 'n', s @ ..] => {
					super::push(out, 9730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 5999);
			match s {
				[] => true,
				['u', 'r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13122);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15538);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 15539);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 13123);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13124);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13125);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6000);
			match s {
				[] => true,
				['e', 'c', 'e', s @ ..] => {
					super::push(out, 9731);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13126);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 13127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rendus.
pub(crate) fn n3459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2940);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 6001);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9732);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13128);
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13129);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 13130);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13131);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6002);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9733);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 9734);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13132);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15540);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 13133);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13134);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15541);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15542);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13135);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13136);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13137);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15543);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15544);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 13138);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => super::t10::n28232(s, out),
								['u', 's', '\u{1}', s @ ..] => super::t10::n28233(s, out),
								_ => false,
							}
						}
						['i', s @ ..] => super::t10::n21960(s, out),
						['n', s @ ..] => super::t10::n21961(s, out),
						_ => false,
					}
				}
				['h', s @ ..] => super::t10::n14822(s, out),
				['m', s @ ..] => super::t10::n14823(s, out),
				['s', s @ ..] => super::t10::n14824(s, out),
				['t', s @ ..] => super::t10::n14825(s, out),
				['u', s @ ..] => super::t10::n14826(s, out),
				['v', s @ ..] => super::t10::n14827(s, out),
				_ => false,
			}
		}
		['g', 'e', 'a', '\u{1}', s @ ..] => super::t10::n8150(s, out),
		['i', s @ ..] => super::t10::n8151(s, out),
		['r', s @ ..] => super::t10::n8152(s, out),
		['u', s @ ..] => super::t10::n8153(s, out),
		_ => false,
	}
}

/// Words ending -sendus.
pub(crate) fn n3460(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2941);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6006);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6007);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9745);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9746);
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6008);
			match s {
				[] => true,
				['e', 'd', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6009);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6010);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tendus.
pub(crate) fn n3461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2942);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6011);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 6012);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13153);
							match s {
								[] => true,
								['f', s @ ..] => {
									super::push(out, 15552);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17082);
											match s {
												[] => true,
												['d', s @ ..] => {
													super::push(out, 17785);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t10::n36621(s, out),
														_ => false,
													}
												}
												['r', s @ ..] => {
													super::push(out, 17786);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t10::n36622(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['f', s @ ..] => {
											super::push(out, 17083);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['m', s @ ..] => {
											super::push(out, 17084);
											match s {
												[] => true,
												['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', s @ ..] => {
											super::push(out, 17085);
											match s {
												[] => true,
												['i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['u', s @ ..] => {
											super::push(out, 17086);
											match s {
												[] => true,
												['n', 'e', 'g', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['p', s @ ..] => {
									super::push(out, 15553);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 17087);
											match s {
												[] => true,
												['a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13154);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15554);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['e', s @ ..] => {
			super::push(out, 6013);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9747);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9748);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13155);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9749);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t10::n8162(s, out),
		['o', s @ ..] => {
			super::push(out, 6015);
			s.is_empty()
		}
		['r', s @ ..] => super::t10::n8164(s, out),
		['s', s @ ..] => {
			super::push(out, 6017);
			match s {
				[] => true,
				['i', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13170);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => super::t10::n8166(s, out),
		['u', s @ ..] => super::t10::n8167(s, out),
		_ => false,
	}
}

/// Words ending -uendus.
pub(crate) fn n3462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2943);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6020);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13182);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15567);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 15568);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13183);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['c', 'a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'n', 'i', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6021);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['x', 'e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
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
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9752);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 9753);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 'o', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6023);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9754);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15569);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 15570);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 15571);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6024);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => super::t10::n22070(s, out),
						['n', s @ ..] => {
							super::push(out, 13185);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => super::t10::n28362(s, out),
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => super::t10::n22072(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => super::t10::n8175(s, out),
		['r', s @ ..] => super::t10::n8176(s, out),
		['s', s @ ..] => super::t10::n8177(s, out),
		['t', s @ ..] => super::t10::n8178(s, out),
		_ => false,
	}
}

/// Words ending -vendus.
pub(crate) fn n3463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2944);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6029);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6030);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9771);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 9772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6031);
			match s {
				[] => true,
				['o', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 13190);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['v', s @ ..] => {
							super::push(out, 13191);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15591);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 17104);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 17105);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15592);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 15593);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6032);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13192);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9774);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13193);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 13194);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 13195);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 15594);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 15595);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15596);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 15597);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => super::t10::n22116(s, out),
						['m', s @ ..] => super::t10::n22117(s, out),
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => super::t10::n22119(s, out),
						['r', s @ ..] => super::t10::n22120(s, out),
						['s', s @ ..] => super::t10::n22121(s, out),
						_ => false,
					}
				}
				['v', s @ ..] => super::t10::n14907(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -xendus.
pub(crate) fn n3464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2945);
	match s {
		[] => true,
		['e', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -abundus.
pub(crate) fn n8184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6033);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9776);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 9777);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13202);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13203);
					match s {
						[] => true,
						['p', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9778);
			match s {
				[] => true,
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9779);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13206);
					s.is_empty()
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9780);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9781);
			match s {
				[] => true,
				['c', 'u', 'r', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13207);
					match s {
						[] => true,
						['o', 'l', 'g', '\u{1}', s @ ..] => {
							super::push(out, 15598);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13208);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 15599);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9782);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 13209);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 't', 'n', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 13210);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', 'a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', 'm', 'e', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13211);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15600);
							match s {
								[] => true,
								['e', 'p', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'n', 'i', 'm', 'd', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 17106);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15601);
							match s {
								[] => true,
								['a', 'r', 'g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9783);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13212);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17107);
									s.is_empty()
								}
								['c', 'a', 'l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13213);
					match s {
						[] => true,
						['r', 'c', 'a', 'l', '\u{1}', s @ ..] => {
							super::push(out, 15602);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9784);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13214);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15603);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 15604);
							match s {
								[] => true,
								['a', 'm', 'i', '\u{1}', s @ ..] => {
									super::push(out, 17108);
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 15605);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15606);
							s.is_empty()
						}
						['t', 's', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 15607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9785);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13216);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13217);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17109);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 17110);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15608);
							match s {
								[] => true,
								['u', 'm', '\u{1}', s @ ..] => {
									super::push(out, 17111);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13218);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13219);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 15609);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13220);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 9786);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13221);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13222);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 13223);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15611);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15612);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13224);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9787);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13225);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 13226);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15614);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15615);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15616);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 13227);
					match s {
						[] => true,
						['c', s @ ..] => super::t10::n28473(s, out),
						['g', s @ ..] => super::t10::n28474(s, out),
						['l', s @ ..] => super::t10::n28475(s, out),
						['s', s @ ..] => super::t10::n28476(s, out),
						['t', s @ ..] => super::t10::n28477(s, out),
						['v', s @ ..] => super::t10::n28478(s, out),
						_ => false,
					}
				}
				['l', s @ ..] => super::t10::n22160(s, out),
				['n', s @ ..] => super::t10::n22161(s, out),
				['p', s @ ..] => super::t10::n22162(s, out),
				['r', s @ ..] => super::t10::n22163(s, out),
				['s', s @ ..] => super::t10::n22164(s, out),
				['u', s @ ..] => super::t10::n22165(s, out),
				_ => false,
			}
		}
		['u', s @ ..] => super::t10::n14925(s, out),
		['x', s @ ..] => super::t10::n14926(s, out),
		_ => false,
	}
}

/// The word describundus.
pub(crate) fn n35600(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -anscribundus.
pub(crate) fn n35601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17788);
	match s {
		[] => true,
		['r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -inscribundus.
pub(crate) fn n35602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17789);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -cundus.
pub(crate) fn n3466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2947);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6037);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13243);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9797);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6038);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 9798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13244);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', 'f', '\u{1}', s @ ..] => {
					super::push(out, 9799);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9800);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6039);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9802);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 9803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6040);
			match s {
				[] => true,
				['i', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13246);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6041);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6042);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9804);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9805);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9806);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['g', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6043);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -dundus.
pub(crate) fn n3467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2948);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6044);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9807);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 9808);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9809);
					match s {
						[] => true,
						['s', 'b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'c', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 9810);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6046);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9811);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 9812);
					match s {
						[] => true,
						['i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 13248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6047);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13249);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 13250);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15633);
									s.is_empty()
								}
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
			super::push(out, 6048);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -eundus.
pub(crate) fn n3468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2949);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6049);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13251);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6050);
			match s {
				[] => true,
				['r', 't', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6051);
			match s {
				[] => true,
				['e', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13252);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6052);
			match s {
				[] => true,
				['n', 'a', 'r', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fundus.
pub(crate) fn n3469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2950);
	s.is_empty()
}

/// Words ending -gundus.
pub(crate) fn n3470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2951);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6053);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9814);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9815);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9816);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13253);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13254);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6054);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['f', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13255);
							match s {
								[] => true,
								['t', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', s @ ..] => {
											super::push(out, 17116);
											match s {
												[] => true,
												['o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
		['r', s @ ..] => {
			super::push(out, 6055);
			match s {
				[] => true,
				['e', 'm', s @ ..] => {
					super::push(out, 9817);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iundus.
pub(crate) fn n3471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2952);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6056);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9818);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 17117);
											match s {
												[] => true,
												['o', 'b', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'a', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 9819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6057);
			s.is_empty()
		}
		['n', 'e', 'v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6058);
			match s {
				[] => true,
				['u', s @ ..] => {
					super::push(out, 9820);
					match s {
						[] => true,
						['g', 'i', 'l', 'b', 'a', '\u{1}', s @ ..] => {
							super::push(out, 13256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6059);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 9821);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lundus.
pub(crate) fn n3472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2953);
	s.is_empty()
}

/// Words ending -mundus.
pub(crate) fn n3473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2954);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6060);
			match s {
				[] => true,
				['d', 'e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6061);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 13257);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 13258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -nundus.
pub(crate) fn n3474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2955);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 6062);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					super::push(out, 9822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', 'e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13259);
							match s {
								[] => true,
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9824);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -pundus.
pub(crate) fn n3475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2956);
	s.is_empty()
}

/// Words ending -rundus.
pub(crate) fn n3476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2957);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6063);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13260);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6064);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sundus.
pub(crate) fn n3477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2958);
	s.is_empty()
}

/// Words ending -tundus.
pub(crate) fn n3478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2959);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6065);
			match s {
				[] => true,
				['e', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13261);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6066);
			match s {
				[] => true,
				['r', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13262);
							match s {
								[] => true,
								['m', 'e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6067);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6068);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -larius.
pub(crate) fn n3756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3177);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 6274);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 9925);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 9926);
					match s {
						[] => true,
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9927);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9928);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6275);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6276);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9929);
					match s {
						[] => true,
						['c', 'n', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13306);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9930);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 13307);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6277);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9931);
					match s {
						[] => true,
						['a', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 9932);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13308);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13309);
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 15652);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 15653);
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', 'a', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9933);
					match s {
						[] => true,
						['n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9934);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9935);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9936);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 9937);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -narius.
pub(crate) fn n3758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3179);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6282);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9940);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9941);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6283);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9942);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 9943);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13310);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9944);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 9945);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13311);
							match s {
								[] => true,
								['u', 'q', s @ ..] => {
									super::push(out, 15654);
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9946);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 13313);
							s.is_empty()
						}
						['p', s @ ..] => {
							super::push(out, 13314);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9947);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9948);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9949);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9950);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9951);
					match s {
						[] => true,
						['n', 'e', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 6284);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6285);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9953);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 9954);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 13315);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13316);
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 9955);
					match s {
						[] => true,
						['r', s @ ..] => {
							super::push(out, 13317);
							match s {
								[] => true,
								['o', 'a', s @ ..] => {
									super::push(out, 15655);
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 13318);
							match s {
								[] => true,
								['t', 'e', 'u', 's', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 15656);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 9956);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13319);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 13320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['h', s @ ..] => {
					super::push(out, 9957);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9958);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9959);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 13321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9960);
					match s {
						[] => true,
						['u', s @ ..] => {
							super::push(out, 13322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15657);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 9961);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9962);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9963);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 13323);
							s.is_empty()
						}
						['q', s @ ..] => {
							super::push(out, 13324);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 9964);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6286);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 9965);
					match s {
						[] => true,
						['c', 'r', 'e', 'm', '\u{1}', s @ ..] => {
							super::push(out, 13325);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6287);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 9966);
					match s {
						[] => true,
						['g', s @ ..] => {
							super::push(out, 13326);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13327);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 13328);
							match s {
								[] => true,
								['c', s @ ..] => {
									super::push(out, 15658);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15659);
									s.is_empty()
								}
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
			super::push(out, 6288);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6289);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -oarius.
pub(crate) fn n3759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3180);
	s.is_empty()
}

/// Words ending -parius.
pub(crate) fn n3760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3181);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6290);
			s.is_empty()
		}
		['o', 'c', 's', '\u{1}', s @ ..] => {
			super::push(out, 6291);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rarius.
pub(crate) fn n3761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3182);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6292);
			match s {
				[] => true,
				['n', 'u', s @ ..] => {
					super::push(out, 9967);
					match s {
						[] => true,
						['f', '\u{1}', s @ ..] => {
							super::push(out, 13329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6293);
			match s {
				[] => true,
				['h', s @ ..] => {
					super::push(out, 9968);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9969);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9970);
					match s {
						[] => true,
						['m', 'e', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6294);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6295);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9971);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9972);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 9973);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9974);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sarius.
pub(crate) fn n3762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3183);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 6296);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6297);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'c', 'e', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tarius.
pub(crate) fn n3763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3184);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6298);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9975);
					s.is_empty()
				}
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 13330);
							s.is_empty()
						}
						['u', 'e', 'f', '\u{1}', s @ ..] => {
							super::push(out, 13331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', 't', 's', '\u{1}', s @ ..] => {
					super::push(out, 9976);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 9977);
					match s {
						[] => true,
						['o', 'l', 'p', 'i', 'd', '\u{1}', s @ ..] => {
							super::push(out, 13332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 9978);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 9979);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 9980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6299);
			match s {
				[] => true,
				['e', 's', s @ ..] => {
					super::push(out, 9981);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 13333);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 9982);
					match s {
						[] => true,
						['c', 'i', 'n', 'a', 'p', '\u{1}', s @ ..] => {
							super::push(out, 13334);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6300);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 9983);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 9984);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9985);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 9986);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 9987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6301);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 9988);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 9989);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13335);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 9990);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13337);
							s.is_empty()
						}
						['y', s @ ..] => {
							super::push(out, 13338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 9991);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13339);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13340);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 9992);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 9993);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6302);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 9994);
					match s {
						[] => true,
						['r', 'd', 'o', s @ ..] => {
							super::push(out, 13341);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 9995);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 13342);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 13343);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15660);
									match s {
										[] => true,
										['l', s @ ..] => {
											super::push(out, 17121);
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 15661);
									match s {
										[] => true,
										['l', 'p', s @ ..] => {
											super::push(out, 17122);
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 15662);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 15663);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15664);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15665);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 15666);
									s.is_empty()
								}
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
			super::push(out, 6303);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6304);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 6305);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6306);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 9996);
					s.is_empty()
				}
				['c', 's', '\u{1}', s @ ..] => {
					super::push(out, 9997);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 9998);
					match s {
						[] => true,
						['t', 'y', 'h', 'p', 'm', 'e', '\u{1}', s @ ..] => {
							super::push(out, 13344);
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 9999);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 10000);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uarius.
pub(crate) fn n3764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3185);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 6307);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6308);
			match s {
				[] => true,
				['a', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13345);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 13346);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10001);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							super::push(out, 13347);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['q', s @ ..] => {
			super::push(out, 6309);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10002);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10003);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6310);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 6311);
			match s {
				[] => true,
				['s', s @ ..] => {
					super::push(out, 10004);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 10005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6312);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10006);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 10007);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 13348);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 13349);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10008);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 10009);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 10010);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10011);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -varius.
pub(crate) fn n3765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3186);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6313);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 6314);
			match s {
				[] => true,
				['t', 'c', 'o', '\u{1}', s @ ..] => {
					super::push(out, 10012);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6315);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10013);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6316);
			match s {
				[] => true,
				['b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 6317);
			match s {
				[] => true,
				['e', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -iatorius.
pub(crate) fn n15248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10030);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 13360);
			match s {
				[] => true,
				['n', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 15673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 13361);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 13362);
			match s {
				[] => true,
				['n', 'u', 'n', '\u{1}', s @ ..] => {
					super::push(out, 15674);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 15675);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -latorius.
pub(crate) fn n15249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10031);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 13363);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13364);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13365);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 17129);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17130);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13366);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15676);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17131);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 17132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 15677);
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 15678);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 15679);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -matorius.
pub(crate) fn n15250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10032);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'f', 'i', 'd', '\u{1}', s @ ..] => {
					super::push(out, 15680);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 15681);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17794);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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

/// Words ending -natorius.
pub(crate) fn n15251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10033);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13367);
			match s {
				[] => true,
				['s', '\u{1}', s @ ..] => {
					super::push(out, 15682);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 13368);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15683);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13369);
			match s {
				[] => true,
				['u', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13370);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 17133);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -ratorius.
pub(crate) fn n15252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10034);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13371);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 13372);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13373);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13374);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 13375);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17134);
							s.is_empty()
						}
						['e', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
							super::push(out, 17135);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 13376);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17136);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17137);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13377);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -satorius.
pub(crate) fn n15253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10035);
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 13378);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 13379);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tatorius.
pub(crate) fn n15254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10036);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 13380);
			match s {
				[] => true,
				['v', 'n', 'i', '\u{1}', s @ ..] => {
					super::push(out, 15685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13381);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13382);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15686);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15687);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 13383);
			match s {
				[] => true,
				['p', 'm', 'o', 'c', '\u{1}', s @ ..] => {
					super::push(out, 15688);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ctorius.
pub(crate) fn n8546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6327);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10037);
			match s {
				[] => true,
				['f', 'o', s @ ..] => {
					super::push(out, 13384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 10038);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 13385);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10039);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15690);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15691);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10040);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13386);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 13387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 10041);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15692);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17138);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15693);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15694);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -etorius.
pub(crate) fn n8547(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6328);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10042);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10043);
			match s {
				[] => true,
				['c', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 13388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -itorius.
pub(crate) fn n8548(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6329);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 10044);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 10045);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 10046);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13389);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13390);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 10047);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 10048);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10049);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 13392);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 10050);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ltorius.
pub(crate) fn n8549(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6330);
	s.is_empty()
}

/// Words ending -otorius.
pub(crate) fn n8550(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6331);
	s.is_empty()
}

/// Words ending -ptorius.
pub(crate) fn n8551(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6332);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10051);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10052);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 10053);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10054);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -storius.
pub(crate) fn n8552(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6333);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10055);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 10056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -utorius.
pub(crate) fn n8553(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6334);
	s.is_empty()
}

/// Words ending -xtorius.
pub(crate) fn n8554(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6335);
	s.is_empty()
}

/// Words ending -ssimus.
pub(crate) fn n3918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3309);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6425);
			s.is_empty()
		}
		['i', s @ ..] => super::t11::n8697(s, out),
		_ => false,
	}
}

/// Words ending -tianus.
pub(crate) fn n3965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3349);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6480);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 6481);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 6482);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 6483);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6484);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10188);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10189);
					match s {
						[] => true,
						['t', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 15819);
									match s {
										[] => true,
										['m', 'e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 15820);
									s.is_empty()
								}
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
			super::push(out, 6485);
			match s {
				[] => true,
				['i', 'r', 'h', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 't', 'n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6486);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ianopolitanus.
pub(crate) fn n35701(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['r', s @ ..] => {
			super::push(out, 18059);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 18060);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -visurus.
pub(crate) fn n9074(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6719);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 10281);
			match s {
				[] => true,
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10282);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10283);
			match s {
				[] => true,
				['d', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10284);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 10285);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 10286);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lsurus.
pub(crate) fn n4194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3527);
	match s {
		[] => true,
		['u', s @ ..] => {
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 10287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15828);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15829);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10288);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13508);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15830);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15831);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13509);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 10289);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13510);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 13511);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15832);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15833);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 13512);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13513);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -nsurus.
pub(crate) fn n4195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3528);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6720);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13514);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15834);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 15835);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6721);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10290);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 13515);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13516);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 15836);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15837);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17261);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 17262);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'o', 'c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 15838);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 17263);
											match s {
												[] => true,
												['r', 't', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 17264);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 15839);
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10291);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 13517);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 13518);
							match s {
								[] => true,
								['o', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['b', 'u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10292);
					match s {
						[] => true,
						['e', 'r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => super::t11::n28834(s, out),
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => super::t11::n28836(s, out),
								['p', 'a', '\u{1}', s @ ..] => super::t11::n28837(s, out),
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => super::t11::n28838(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t11::n15646(s, out),
				['p', s @ ..] => super::t11::n15647(s, out),
				['r', s @ ..] => super::t11::n15648(s, out),
				['s', s @ ..] => super::t11::n15649(s, out),
				['t', s @ ..] => super::t11::n15650(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t11::n9078(s, out),
		['o', s @ ..] => super::t11::n9079(s, out),
		['u', s @ ..] => super::t11::n9080(s, out),
		_ => false,
	}
}

/// Words ending -osurus.
pub(crate) fn n4196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3529);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 6725);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
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
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10300);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -psurus.
pub(crate) fn n4197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3530);
	match s {
		[] => true,
		['a', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10301);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 13527);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13528);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10302);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10303);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10304);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10305);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -rsurus.
pub(crate) fn n4198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3531);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6726);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 15846);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											super::push(out, 17270);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6727);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 10306);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13529);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 13530);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10307);
					match s {
						[] => true,
						['s', 'n', s @ ..] => {
							super::push(out, 13531);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 15847);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15848);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10308);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13532);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15849);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15850);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 10309);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13533);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 13534);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 13535);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', 'i', 'n', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 13536);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 15851);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15852);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 13537);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13538);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 13539);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', s @ ..] => {
											match s {
												[] => true,
												['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												['n', 'i', '\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 13540);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6728);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10310);
					s.is_empty()
				}
				['m', s @ ..] => super::t11::n15675(s, out),
				['x', s @ ..] => {
					super::push(out, 10312);
					match s {
						[] => true,
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t11::n9088(s, out),
		_ => false,
	}
}

/// Words ending -ssurus.
pub(crate) fn n4199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3532);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6730);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6731);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10313);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 10314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							super::push(out, 13543);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13544);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 15854);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 15855);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13545);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13546);
							match s {
								[] => true,
								['r', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13547);
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10315);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 13548);
							match s {
								[] => true,
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 13549);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13550);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10316);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13551);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', 'r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => super::t11::n15684(s, out),
				['s', s @ ..] => {
					super::push(out, 10318);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13554);
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 13555);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13556);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => super::t11::n22754(s, out),
						['n', s @ ..] => super::t11::n22755(s, out),
						['p', 'o', '\u{1}', s @ ..] => super::t11::n22756(s, out),
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => super::t11::n22757(s, out),
						['s', s @ ..] => super::t11::n22758(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => super::t11::n9091(s, out),
		['o', s @ ..] => super::t11::n9092(s, out),
		['u', s @ ..] => super::t11::n9093(s, out),
		_ => false,
	}
}

/// Words ending -usurus.
pub(crate) fn n4200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3533);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 6735);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => {
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 6736);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10324);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 6737);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10325);
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10326);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10327);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 13576);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 13577);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10328);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10329);
					match s {
						[] => true,
						['e', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10330);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6738);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 10331);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10332);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 13578);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 13579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 6739);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10333);
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10334);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 13580);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 15877);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 15878);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13581);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'b', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 6740);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10335);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10336);
					match s {
						[] => true,
						['e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -aturus.
pub(crate) fn n4201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3534);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 6741);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10337);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13582);
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13583);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15879);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'l', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10338);
					match s {
						[] => true,
						['r', 'p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10339);
					match s {
						[] => true,
						['e', 'c', 'a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 't', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 15880);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15881);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', s @ ..] => {
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', 'm', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10340);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 13584);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 15882);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 15883);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => super::t11::n9106(s, out),
		['d', s @ ..] => super::t11::n9107(s, out),
		['e', s @ ..] => {
			super::push(out, 6744);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10361);
					match s {
						[] => true,
						['l', 'a', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10362);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 13636);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10363);
					match s {
						[] => true,
						['i', s @ ..] => super::t11::n22901(s, out),
						['u', s @ ..] => super::t11::n22902(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t11::n15746(s, out),
				['u', s @ ..] => super::t11::n15747(s, out),
				_ => false,
			}
		}
		['f', s @ ..] => super::t11::n9109(s, out),
		['g', s @ ..] => super::t11::n9110(s, out),
		['h', s @ ..] => super::t11::n9111(s, out),
		['i', s @ ..] => super::t11::n9112(s, out),
		['l', s @ ..] => super::t11::n9113(s, out),
		['m', s @ ..] => super::t11::n9114(s, out),
		['n', s @ ..] => super::t11::n9115(s, out),
		['o', s @ ..] => super::t11::n9116(s, out),
		['p', s @ ..] => super::t11::n9117(s, out),
		['r', s @ ..] => super::t11::n9118(s, out),
		['s', s @ ..] => super::t11::n9119(s, out),
		['t', s @ ..] => super::t11::n9120(s, out),
		['u', s @ ..] => super::t11::n9121(s, out),
		['v', s @ ..] => super::t11::n9122(s, out),
		['x', s @ ..] => super::t11::n9123(s, out),
		['z', s @ ..] => super::t11::n9124(s, out),
		_ => false,
	}
}

/// Words ending -cturus.
pub(crate) fn n4202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3535);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6761);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10477);
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 10478);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 10479);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 13967);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10480);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => super::t11::n23326(s, out),
						['i', 'r', 'c', 'u', 'l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 13968);
							match s {
								[] => true,
								['a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10481);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => super::t11::n15877(s, out),
				['n', s @ ..] => super::t11::n15878(s, out),
				['o', s @ ..] => super::t11::n15879(s, out),
				['p', s @ ..] => super::t11::n15880(s, out),
				['r', s @ ..] => super::t11::n15881(s, out),
				['s', s @ ..] => super::t11::n15882(s, out),
				['t', s @ ..] => super::t11::n15883(s, out),
				['x', s @ ..] => super::t11::n15884(s, out),
				_ => false,
			}
		}
		['e', s @ ..] => super::t11::n9126(s, out),
		['i', s @ ..] => super::t11::n9127(s, out),
		['l', s @ ..] => super::t11::n9128(s, out),
		['n', s @ ..] => super::t11::n9129(s, out),
		['o', s @ ..] => super::t11::n9130(s, out),
		['u', s @ ..] => super::t11::n9131(s, out),
		_ => false,
	}
}

/// Words ending -iturus.
pub(crate) fn n4204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3537);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 6772);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 14056);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14057);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10522);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14058);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14059);
							s.is_empty()
						}
						['r', 'p', '\u{1}', s @ ..] => {
							super::push(out, 14060);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10523);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14061);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16403);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['h', s @ ..] => {
							super::push(out, 14062);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16404);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											super::push(out, 17600);
											s.is_empty()
										}
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
					super::push(out, 10524);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16405);
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10525);
					match s {
						[] => true,
						['o', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10526);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', s @ ..] => {
									super::push(out, 16406);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16407);
									match s {
										[] => true,
										['s', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16408);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6773);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 10527);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', 'p', s @ ..] => super::t11::n23513(s, out),
						['t', '\u{1}', s @ ..] => super::t11::n23514(s, out),
						_ => false,
					}
				}
				['c', s @ ..] => super::t11::n15930(s, out),
				['e', s @ ..] => super::t11::n15931(s, out),
				['i', s @ ..] => super::t11::n15932(s, out),
				['n', 'o', 'c', '\u{1}', s @ ..] => super::t11::n15933(s, out),
				['o', s @ ..] => super::t11::n15934(s, out),
				['r', s @ ..] => super::t11::n15935(s, out),
				['s', s @ ..] => super::t11::n15936(s, out),
				['x', s @ ..] => super::t11::n15937(s, out),
				_ => false,
			}
		}
		['d', s @ ..] => super::t11::n9140(s, out),
		['e', s @ ..] => super::t11::n9141(s, out),
		['f', s @ ..] => super::t11::n9142(s, out),
		['g', s @ ..] => super::t11::n9143(s, out),
		['l', s @ ..] => super::t11::n9144(s, out),
		['m', s @ ..] => super::t11::n9145(s, out),
		['n', s @ ..] => super::t11::n9146(s, out),
		['o', s @ ..] => super::t11::n9147(s, out),
		['p', s @ ..] => super::t11::n9148(s, out),
		['r', s @ ..] => super::t11::n9149(s, out),
		['s', s @ ..] => super::t11::n9150(s, out),
		['t', s @ ..] => super::t11::n9151(s, out),
		['u', s @ ..] => super::t11::n9152(s, out),
		['v', s @ ..] => super::t11::n9153(s, out),
		['x', s @ ..] => super::t11::n9154(s, out),
		_ => false,
	}
}

/// Words ending -lturus.
pub(crate) fn n4205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3538);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6789);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 10596);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 10597);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10598);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10599);
					match s {
						[] => true,
						['e', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16472);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10600);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14159);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16473);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16474);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14160);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16475);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16476);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14161);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 14162);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -nturus.
pub(crate) fn n4207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3540);
	match s {
		[] => true,
		['a', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 10601);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14163);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 16477);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'o', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 10602);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14164);
							match s {
								[] => true,
								['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14165);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16478);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 16479);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14166);
							match s {
								[] => true,
								['i', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14167);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14168);
							match s {
								[] => true,
								['e', s @ ..] => {
									match s {
										[] => true,
										['p', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												['u', 's', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', 'n', 'i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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

/// Words ending -oturus.
pub(crate) fn n4208(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3541);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 6790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10603);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 6791);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 10604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 10605);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10606);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14169);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14170);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14172);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10607);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 10608);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10609);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10610);
					match s {
						[] => true,
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10611);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 6792);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 16480);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17630);
											s.is_empty()
										}
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
					super::push(out, 10612);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 6793);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10613);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 6794);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10614);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -pturus.
pub(crate) fn n4209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3542);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6795);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14173);
							match s {
								[] => true,
								['s', 'u', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6796);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10615);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16481);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['c', s @ ..] => {
					super::push(out, 10616);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14174);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16482);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10617);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10618);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10619);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 14175);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 16483);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16484);
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 14176);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 16485);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 16486);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14177);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14178);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14179);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14180);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16487);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 10620);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 14181);
							match s {
								[] => true,
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14182);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6797);
			match s {
				[] => true,
				['r', 'c', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => super::t11::n23748(s, out),
						['x', s @ ..] => {
							super::push(out, 14183);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 6798);
			match s {
				[] => true,
				['u', 'c', 's', s @ ..] => super::t11::n16063(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t11::n9168(s, out),
		['r', s @ ..] => super::t11::n9169(s, out),
		['u', s @ ..] => super::t11::n9170(s, out),
		_ => false,
	}
}

/// Words ending -rturus.
pub(crate) fn n4210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3543);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6802);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14193);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
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
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 6803);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 10629);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14194);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 14195);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14196);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16509);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16510);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 10630);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['c', 's', 'i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10631);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14197);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 14198);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14199);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16511);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16512);
									match s {
										[] => true,
										['c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 14200);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17639);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 14201);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6804);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14202);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16513);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sturus.
pub(crate) fn n4211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3544);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6805);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14203);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6806);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 10632);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 10633);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14204);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16515);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 16516);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 14205);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14206);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 14207);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10634);
					match s {
						[] => true,
						['q', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6807);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 6808);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14208);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', 'e', 'd', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6809);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -uturus.
pub(crate) fn n4212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3545);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6810);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 6811);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 14209);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16517);
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 16518);
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14210);
							match s {
								[] => true,
								['t', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['n', 'o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['s', 'i', 'd', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['c', s @ ..] => {
			super::push(out, 6812);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10635);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10636);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 16519);
									match s {
										[] => true,
										['o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16520);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 17640);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 17641);
											match s {
												[] => true,
												['c', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16521);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10637);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 16522);
									match s {
										[] => true,
										['r', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16523);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17642);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 17643);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['l', s @ ..] => {
									super::push(out, 16524);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16525);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6813);
			match s {
				[] => true,
				['n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t11::n9184(s, out),
		['g', s @ ..] => {
			super::push(out, 6815);
			match s {
				[] => true,
				['r', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6816);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => super::t11::n9187(s, out),
		['n', s @ ..] => super::t11::n9188(s, out),
		['p', s @ ..] => super::t11::n9189(s, out),
		['r', s @ ..] => super::t11::n9190(s, out),
		['s', s @ ..] => super::t11::n9191(s, out),
		['t', s @ ..] => super::t11::n9192(s, out),
		['u', s @ ..] => super::t11::n9193(s, out),
		_ => false,
	}
}

/// Words ending -xturus.
pub(crate) fn n4213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3546);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 6824);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10658);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14222);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14223);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', s @ ..] => {
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['t', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word percuriosus.
pub(crate) fn n23955(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -furiosus.
pub(crate) fn n16287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10725);
	s.is_empty()
}

/// Words ending -iuriosus.
pub(crate) fn n16288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10726);
	s.is_empty()
}

/// Words ending -xuriosus.
pub(crate) fn n16289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10727);
	s.is_empty()
}

/// Words ending -siosus.
pub(crate) fn n4318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3613);
	match s {
		[] => true,
		['r', 'e', 'v', 'o', 'r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			super::push(out, 6900);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiosus.
pub(crate) fn n4319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3614);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 6901);
			match s {
				[] => true,
				['p', s @ ..] => {
					super::push(out, 10728);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 10729);
					s.is_empty()
				}
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 6902);
			match s {
				[] => true,
				['e', 'f', 'n', s @ ..] => {
					super::push(out, 10730);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 6903);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 10731);
					s.is_empty()
				}
				['g', s @ ..] => {
					super::push(out, 10732);
					match s {
						[] => true,
						['a', 'l', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 6904);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 6905);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', 'e', 'n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 6906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -uiosus.
pub(crate) fn n4320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3615);
	s.is_empty()
}

/// Words ending -viosus.
pub(crate) fn n4321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3616);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 6907);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 6908);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -icatus.
pub(crate) fn n4463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3716);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 7036);
			match s {
				[] => true,
				['l', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 7037);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 10853);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16547);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 10854);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10855);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14296);
							match s {
								[] => true,
								['p', s @ ..] => {
									super::push(out, 16548);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16549);
									match s {
										[] => true,
										['p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 14297);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 14298);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 14299);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16550);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10856);
					match s {
						[] => true,
						['r', 'i', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10857);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 14300);
							match s {
								[] => true,
								['m', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17650);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
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
							super::push(out, 14301);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10858);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 14302);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 14303);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10859);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14304);
							match s {
								[] => true,
								['l', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', s @ ..] => {
									super::push(out, 16551);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16552);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 17651);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 17652);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 16553);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 14306);
							match s {
								[] => true,
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => super::t11::n9640(s, out),
		['l', s @ ..] => {
			super::push(out, 7039);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 10861);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10862);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => super::t11::n16604(s, out),
				['l', s @ ..] => super::t11::n16605(s, out),
				['p', s @ ..] => super::t11::n16606(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t11::n9642(s, out),
		['n', s @ ..] => super::t11::n9643(s, out),
		['p', s @ ..] => super::t12::n9644(s, out),
		['r', s @ ..] => super::t12::n9645(s, out),
		['s', s @ ..] => super::t12::n9646(s, out),
		['t', s @ ..] => super::t12::n9647(s, out),
		['v', s @ ..] => super::t12::n9648(s, out),
		_ => false,
	}
}

/// Words ending -ocatus.
pub(crate) fn n4466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3719);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 7047);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7048);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 7049);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10893);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7050);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 7051);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 10894);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 10895);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10896);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 14347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10897);
					match s {
						[] => true,
						['u', 'q', 'e', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10898);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10899);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -mercatus.
pub(crate) fn n16652(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 14348);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -urcatus.
pub(crate) fn n9658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7053);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'b', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 14349);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -scatus.
pub(crate) fn n4468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3721);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7054);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7055);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 10900);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10901);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 10902);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7056);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 14350);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -ucatus.
pub(crate) fn n4469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3722);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 7057);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10903);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10904);
					match s {
						[] => true,
						['a', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7058);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 10905);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 7059);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word praemandatus.
pub(crate) fn n34806(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -demandatus.
pub(crate) fn n30766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16589);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -pandatus.
pub(crate) fn n16684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10911);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -endatus.
pub(crate) fn n9683(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7075);
	match s {
		[] => true,
		['m', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14357);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 14358);
					match s {
						[] => true,
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -undatus.
pub(crate) fn n9684(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7076);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'a', 'r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 10912);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14359);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 16590);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 16591);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 14360);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', 'e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['f', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 10913);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 14361);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 10914);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 14362);
					match s {
						[] => true,
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 14363);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 'o', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -odatus.
pub(crate) fn n4476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3729);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 7077);
			match s {
				[] => true,
				['m', 'o', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7078);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rdatus.
pub(crate) fn n4477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3730);
	match s {
		[] => true,
		['a', 't', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', 'c', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10916);
					match s {
						[] => true,
						['m', 'e', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udatus.
pub(crate) fn n4478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3731);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7079);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10917);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', s @ ..] => {
									super::push(out, 16592);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['g', s @ ..] => {
									super::push(out, 16593);
									match s {
										[] => true,
										['n', 'o', 'l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['m', s @ ..] => {
									super::push(out, 16594);
									match s {
										[] => true,
										['e', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 16595);
									s.is_empty()
								}
								['v', s @ ..] => {
									super::push(out, 16596);
									match s {
										[] => true,
										['e', 'r', 'b', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14364);
							match s {
								[] => true,
								['f', 'u', 'r', '\u{1}', s @ ..] => {
									super::push(out, 16597);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 10918);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10919);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16598);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
		['e', s @ ..] => {
			super::push(out, 7080);
			match s {
				[] => true,
				['f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 7081);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 7082);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10920);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 7083);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10921);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ydatus.
pub(crate) fn n4479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3732);
	s.is_empty()
}

/// Words ending -ueatus.
pub(crate) fn n4488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3741);
	match s {
		[] => true,
		['q', 'a', 'l', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -fligatus.
pub(crate) fn n16760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10937);
	match s {
		[] => true,
		['n', s @ ..] => {
			super::push(out, 14375);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 14376);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -lligatus.
pub(crate) fn n16761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10938);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -nligatus.
pub(crate) fn n16763(s: &[char], _out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -migatus.
pub(crate) fn n9730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7106);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 10939);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rigatus.
pub(crate) fn n9731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7107);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['t', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 10941);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tigatus.
pub(crate) fn n9732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7108);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10942);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16601);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10943);
			match s {
				[] => true,
				['f', 'e', 'd', '\u{1}', s @ ..] => {
					super::push(out, 14377);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 10944);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10945);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 14378);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 14379);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17662);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 14380);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 14381);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -vigatus.
pub(crate) fn n9733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7109);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 10946);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 16602);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 16603);
							match s {
								[] => true,
								['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 10947);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -lgatus.
pub(crate) fn n4498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3745);
	match s {
		[] => true,
		['o', s @ ..] => {
			super::push(out, 7110);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14382);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7111);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 10948);
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 10949);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14383);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 14384);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 14385);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 14386);
							match s {
								[] => true,
								['e', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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

/// Words ending -ngatus.
pub(crate) fn n4499(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3746);
	match s {
		[] => true,
		['o', 'l', s @ ..] => {
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10950);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 10951);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ogatus.
pub(crate) fn n4500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3747);
	match s {
		[] => true,
		['r', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 10952);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 14387);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 10954);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 10955);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rgatus.
pub(crate) fn n4501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3748);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 7112);
			match s {
				[] => true,
				['v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 7113);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 10956);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', '\u{1}', s @ ..] => {
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
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 14388);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ugatus.
pub(crate) fn n4502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3749);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 7114);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7115);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 10957);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 10958);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 10959);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 10960);
					match s {
						[] => true,
						['b', '\u{1}', s @ ..] => {
							super::push(out, 14389);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 14390);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 10961);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7116);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7117);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word unifasciatus.
pub(crate) fn n36468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18006);
	s.is_empty()
}

/// The word rubrofasciatus.
pub(crate) fn n36469(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word nigrofasciatus.
pub(crate) fn n36470(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -idiatus.
pub(crate) fn n9753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7126);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 10972);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 10973);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 10974);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word odiatus.
pub(crate) fn n16818(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word inodiatus.
pub(crate) fn n16819(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -udiatus.
pub(crate) fn n9755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7128);
	s.is_empty()
}

/// Words ending -giatus.
pub(crate) fn n4507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3754);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 7129);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7130);
			match s {
				[] => true,
				['t', 's', 'a', 'f', '\u{1}', s @ ..] => {
					super::push(out, 10975);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -hiatus.
pub(crate) fn n4508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3755);
	match s {
		[] => true,
		['c', 'a', s @ ..] => {
			super::push(out, 7131);
			match s {
				[] => true,
				['r', 'b', '\u{1}', s @ ..] => {
					super::push(out, 10976);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -liatus.
pub(crate) fn n4509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3756);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 7132);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 7133);
			match s {
				[] => true,
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14394);
							match s {
								[] => true,
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', 'o', 'r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7134);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', 'u', 'h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10977);
					match s {
						[] => true,
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['x', 'u', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', 'a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7135);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 10978);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 14395);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16607);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16608);
									match s {
										[] => true,
										['i', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['x', s @ ..] => {
									super::push(out, 16609);
									match s {
										[] => true,
										['e', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 7136);
			match s {
				[] => true,
				['m', 'a', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['y', s @ ..] => {
			super::push(out, 7137);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -miatus.
pub(crate) fn n4510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3757);
	s.is_empty()
}

/// Words ending -niatus.
pub(crate) fn n4511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3758);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7138);
			match s {
				[] => true,
				['l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 14396);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7139);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7140);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10979);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 7141);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7142);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -piatus.
pub(crate) fn n4512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3759);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 7143);
			match s {
				[] => true,
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 7144);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -riatus.
pub(crate) fn n4513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3760);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7145);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 7146);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 10981);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 10982);
					match s {
						[] => true,
						['i', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7147);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 10983);
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 10984);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 10985);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['n', s @ ..] => {
									super::push(out, 16610);
									match s {
										[] => true,
										['u', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 10986);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 14397);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 14398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 7148);
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 7149);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 10987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 7150);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['h', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 7151);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 14399);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 14400);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 10988);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 10989);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 10990);
					s.is_empty()
				}
				['x', s @ ..] => {
					super::push(out, 10991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -siatus.
pub(crate) fn n4514(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3761);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7152);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7153);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -tiatus.
pub(crate) fn n4515(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3762);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 7154);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 7155);
			match s {
				[] => true,
				['r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['p', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 7156);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 10992);
					s.is_empty()
				}
				['p', 'o', 'r', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 7157);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 10993);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 10994);
					match s {
						[] => true,
						['c', 's', s @ ..] => {
							super::push(out, 14401);
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 10995);
					match s {
						[] => true,
						['n', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16611);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17665);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 17666);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 17667);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 17668);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['n', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16612);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
			super::push(out, 7158);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 7159);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 7160);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 7161);
			s.is_empty()
		}
		_ => false,
	}
}

