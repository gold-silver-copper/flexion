use flexion::Path;

/// Words ending -ulandus.
pub(crate) fn n7997(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5856);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9356);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12620);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16889);
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
						['p', s @ ..] => {
							super::push(out, 15163);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15164);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12621);
					match s {
						[] => true,
						['r', 't', s @ ..] => {
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
								['t', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 12622);
					match s {
						[] => true,
						['a', s @ ..] => {
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
								['e', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'c', '\u{1}', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 9357);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12623);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12624);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16890);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 16891);
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
				['i', s @ ..] => {
					super::push(out, 12625);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12626);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12627);
					match s {
						[] => true,
						['o', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9358);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12628);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12629);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12630);
					match s {
						[] => true,
						['m', s @ ..] => {
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
		['g', s @ ..] => {
			super::push(out, 9359);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12631);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12632);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12633);
					match s {
						[] => true,
						['a', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 't', 's', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				['u', s @ ..] => {
					super::push(out, 12634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9360);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12635);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12636);
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
		['l', s @ ..] => {
			super::push(out, 9361);
			match s {
				[] => true,
				['u', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 9362);
			match s {
				[] => true,
				['e', 'a', s @ ..] => {
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
				['i', 's', s @ ..] => {
					super::push(out, 12637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15165);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', 'c', s @ ..] => super::t12::n20933(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t12::n14351(s, out),
		['s', s @ ..] => {
			super::push(out, 9364);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12641);
					s.is_empty()
				}
				['x', s @ ..] => super::t12::n20938(s, out),
				_ => false,
			}
		}
		['t', s @ ..] => super::t12::n14353(s, out),
		_ => false,
	}
}

/// Words ending -ximandus.
pub(crate) fn n14369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9381);
	match s {
		[] => true,
		['o', 'r', 'p', s @ ..] => {
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

/// Words ending -mmandus.
pub(crate) fn n8002(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5861);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9382);
			match s {
				[] => true,
				['l', 'f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15173);
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
		['e', s @ ..] => {
			super::push(out, 9383);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9384);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -omandus.
pub(crate) fn n8003(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5862);
	match s {
		[] => true,
		['d', s @ ..] => {
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

/// Words ending -rmandus.
pub(crate) fn n8004(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5863);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9385);
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
		['i', s @ ..] => {
			super::push(out, 9386);
			match s {
				[] => true,
				['f', s @ ..] => {
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
						['f', s @ ..] => {
							super::push(out, 15174);
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
						['n', s @ ..] => {
							super::push(out, 15175);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16896);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16897);
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
		['o', s @ ..] => {
			super::push(out, 9387);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15176);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16898);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16899);
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
						['f', s @ ..] => {
							super::push(out, 15177);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15178);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 16900);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 16901);
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
							super::push(out, 15179);
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
					super::push(out, 12655);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -smandus.
pub(crate) fn n8005(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5864);
	match s {
		[] => true,
		['a', 'l', 'p', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 't', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -umandus.
pub(crate) fn n8006(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5865);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 9388);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 9389);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 9390);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9391);
			s.is_empty()
		}
		['p', s @ ..] => {
			super::push(out, 9392);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15180);
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
		['r', s @ ..] => {
			super::push(out, 9393);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9394);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ugnandus.
pub(crate) fn n14394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9402);
	match s {
		[] => true,
		['p', s @ ..] => {
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
					super::push(out, 15187);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 16906);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 16907);
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
					super::push(out, 15188);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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

/// Words ending -inandus.
pub(crate) fn n8010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5869);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9403);
			s.is_empty()
		}
		['c', s @ ..] => {
			super::push(out, 9404);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12661);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12662);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15189);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15190);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 15191);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12663);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12664);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12665);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15192);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15193);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12666);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12667);
					match s {
						[] => true,
						['l', 'a', s @ ..] => {
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
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9405);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12668);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15194);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15195);
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12669);
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
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9406);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12670);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15196);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15197);
							s.is_empty()
						}
						['v', s @ ..] => {
							super::push(out, 15198);
							s.is_empty()
						}
						_ => false,
					}
				}
				['i', s @ ..] => {
					super::push(out, 12671);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', s @ ..] => {
									super::push(out, 16908);
									s.is_empty()
								}
								['u', s @ ..] => {
									super::push(out, 16909);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12672);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15199);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15200);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['h', s @ ..] => {
			super::push(out, 9407);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9408);
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
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15201);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16910);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16911);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15202);
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
		['m', s @ ..] => {
			super::push(out, 9409);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12673);
					match s {
						[] => true,
						['l', 'f', 'f', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							match s {
								[] => true,
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'a', '\u{1}', s @ ..] => {
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
				['e', s @ ..] => {
					super::push(out, 12674);
					match s {
						[] => true,
						['f', s @ ..] => super::t12::n27115(s, out),
						['g', s @ ..] => super::t12::n27116(s, out),
						['s', s @ ..] => super::t12::n27117(s, out),
						_ => false,
					}
				}
				['i', s @ ..] => super::t12::n21004(s, out),
				['l', s @ ..] => super::t12::n21005(s, out),
				['m', s @ ..] => super::t12::n21006(s, out),
				['o', s @ ..] => super::t12::n21007(s, out),
				['r', s @ ..] => super::t12::n21008(s, out),
				['u', s @ ..] => super::t12::n21009(s, out),
				_ => false,
			}
		}
		['p', s @ ..] => super::t12::n14402(s, out),
		['r', s @ ..] => super::t12::n14403(s, out),
		['t', s @ ..] => super::t12::n14404(s, out),
		['u', s @ ..] => super::t12::n14405(s, out),
		['v', s @ ..] => {
			super::push(out, 9414);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -onandus.
pub(crate) fn n8013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5872);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 9415);
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
		['d', s @ ..] => {
			super::push(out, 9416);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9417);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12691);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12692);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15216);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15217);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15218);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9418);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9419);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9420);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 12693);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							super::push(out, 15219);
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9422);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12694);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -rnandus.
pub(crate) fn n8014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5873);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9423);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9424);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12695);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15220);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15221);
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
										['g', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
					super::push(out, 12696);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12697);
					match s {
						[] => true,
						['s', s @ ..] => {
							super::push(out, 15222);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9425);
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
				['t', s @ ..] => {
					super::push(out, 12698);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// The word emancipandus.
pub(crate) fn n36542(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -erandus.
pub(crate) fn n8029(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5886);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9441);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12713);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16920);
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
				['r', s @ ..] => {
					super::push(out, 12714);
					match s {
						[] => true,
						['e', 'v', s @ ..] => {
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
								['s', s @ ..] => {
									super::push(out, 16921);
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
				['u', s @ ..] => {
					super::push(out, 12715);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15235);
							match s {
								[] => true,
								['o', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 15236);
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
		['c', s @ ..] => {
			super::push(out, 9442);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12716);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12717);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15237);
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 15238);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', s @ ..] => {
									super::push(out, 16922);
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
						['m', s @ ..] => {
							super::push(out, 15239);
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
					super::push(out, 12718);
					match s {
						[] => true,
						['u', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9443);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12719);
					match s {
						[] => true,
						['o', 'f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16923);
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
					super::push(out, 12720);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16924);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16925);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12721);
					match s {
						[] => true,
						['o', 'p', s @ ..] => {
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
				['o', s @ ..] => {
					super::push(out, 12722);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15240);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15241);
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
		['f', s @ ..] => {
			super::push(out, 9444);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9445);
			match s {
				[] => true,
				['g', s @ ..] => {
					super::push(out, 12724);
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
								['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
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
				['i', s @ ..] => {
					super::push(out, 12725);
					match s {
						[] => true,
						['l', s @ ..] => {
							super::push(out, 15242);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15243);
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
			super::push(out, 9446);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15244);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 15245);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 15246);
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
					super::push(out, 12726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9447);
			match s {
				[] => true,
				['e', 'c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'a', '\u{1}', s @ ..] => super::t12::n27200(s, out),
						['d', 'a', '\u{1}', s @ ..] => super::t12::n27201(s, out),
						['s', s @ ..] => super::t12::n27202(s, out),
						_ => false,
					}
				}
				['o', 't', s @ ..] => super::t12::n21099(s, out),
				_ => false,
			}
		}
		['m', s @ ..] => super::t12::n14455(s, out),
		['n', s @ ..] => super::t12::n14456(s, out),
		['p', s @ ..] => super::t12::n14457(s, out),
		['s', s @ ..] => super::t12::n14458(s, out),
		['t', s @ ..] => super::t12::n14459(s, out),
		['u', s @ ..] => super::t12::n14460(s, out),
		['v', s @ ..] => super::t12::n14461(s, out),
		_ => false,
	}
}

/// Words ending -grandus.
pub(crate) fn n8030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5887);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9455);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12735);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16928);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16929);
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
				['r', s @ ..] => {
					super::push(out, 12736);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15255);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 15256);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9456);
			match s {
				[] => true,
				['t', 'n', 'i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'e', 'r', '\u{1}', s @ ..] => {
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
			super::push(out, 9457);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 12737);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15257);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 16930);
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
						['m', s @ ..] => {
							super::push(out, 15258);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15259);
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
					super::push(out, 12738);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12739);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -irandus.
pub(crate) fn n8031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5888);
	match s {
		[] => true,
		['m', s @ ..] => {
			super::push(out, 9458);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12740);
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
			super::push(out, 9459);
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
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15260);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', 's', '\u{1}', s @ ..] => {
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
			super::push(out, 9460);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 9461);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -orandus.
pub(crate) fn n8032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5889);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 9462);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12741);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16931);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12742);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
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
				_ => false,
			}
		}
		['c', s @ ..] => {
			super::push(out, 9463);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12743);
					match s {
						[] => true,
						['d', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16932);
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
				['l', s @ ..] => {
					super::push(out, 12744);
					match s {
						[] => true,
						['u', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12745);
					match s {
						[] => true,
						['e', 't', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9464);
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
		['f', s @ ..] => {
			super::push(out, 9465);
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
		['g', s @ ..] => {
			super::push(out, 9466);
			match s {
				[] => true,
				['i', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9467);
			match s {
				[] => true,
				['e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'e', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9468);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 12746);
					match s {
						[] => true,
						['c', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16933);
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
				['p', s @ ..] => {
					super::push(out, 12747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15261);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
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
			super::push(out, 9469);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15262);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 15263);
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
								_ => false,
							}
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
			super::push(out, 9470);
			match s {
				[] => true,
				['g', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'm', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
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
		['p', s @ ..] => {
			super::push(out, 9471);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12748);
					match s {
						[] => true,
						['v', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16934);
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
				['o', s @ ..] => {
					super::push(out, 12749);
					match s {
						[] => true,
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12750);
					match s {
						[] => true,
						['o', 'c', s @ ..] => {
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
				_ => false,
			}
		}
		['r', s @ ..] => super::t12::n14480(s, out),
		['t', s @ ..] => super::t12::n14481(s, out),
		['v', s @ ..] => super::t12::n14482(s, out),
		['x', s @ ..] => {
			super::push(out, 9475);
			match s {
				[] => true,
				['e', '\u{1}', s @ ..] => super::t12::n21158(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -prandus.
pub(crate) fn n8033(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5890);
	s.is_empty()
}

/// Words ending -rrandus.
pub(crate) fn n8034(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5891);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9476);
			match s {
				[] => true,
				['n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15264);
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
		['e', s @ ..] => {
			super::push(out, 9477);
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9478);
			match s {
				[] => true,
				['s', 'u', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15265);
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

/// Words ending -trandus.
pub(crate) fn n8035(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5892);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9479);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 12752);
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
						['l', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12753);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9480);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9481);
			match s {
				[] => true,
				['c', 'l', 'a', 'c', s @ ..] => {
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
			super::push(out, 9482);
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
		['s', s @ ..] => {
			super::push(out, 9483);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12754);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12755);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12756);
					match s {
						[] => true,
						['n', 'i', 'm', s @ ..] => {
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
								['m', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12757);
					match s {
						[] => true,
						['o', 'm', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['a', 'r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 17726);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12758);
					match s {
						[] => true,
						['l', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -urandus.
pub(crate) fn n8036(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5893);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9484);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12759);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 16935);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16936);
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
		['c', s @ ..] => {
			super::push(out, 9485);
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
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12760);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'b', 'o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9486);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'o', '\u{1}', s @ ..] => {
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
		['f', s @ ..] => {
			super::push(out, 9487);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9488);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12761);
					match s {
						[] => true,
						['f', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16937);
									match s {
										[] => true,
										['r', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 16938);
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 16939);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 16940);
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
				['l', s @ ..] => {
					super::push(out, 12762);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12763);
					match s {
						[] => true,
						['a', s @ ..] => {
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
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9489);
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
					super::push(out, 12764);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15266);
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 15267);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 15268);
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
				['n', 'o', 'c', '\u{1}', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 9490);
			match s {
				[] => true,
				['r', 'u', 'm', s @ ..] => {
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
						['m', s @ ..] => {
							match s {
								[] => true,
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
		['p', s @ ..] => {
			super::push(out, 9491);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9492);
			s.is_empty()
		}
		['t', s @ ..] => {
			super::push(out, 9493);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12765);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15269);
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
						['s', s @ ..] => {
							super::push(out, 15270);
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
				['b', s @ ..] => {
					super::push(out, 12766);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12767);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -yrandus.
pub(crate) fn n8037(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5894);
	match s {
		[] => true,
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
		_ => false,
	}
}

/// The word detractandus.
pub(crate) fn n36546(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18016);
	s.is_empty()
}

/// Words ending -ectandus.
pub(crate) fn n14522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9510);
	match s {
		[] => true,
		['i', s @ ..] => {
			super::push(out, 12785);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15276);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15277);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 15278);
					s.is_empty()
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 12786);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 15279);
					match s {
						[] => true,
						['o', s @ ..] => {
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
					super::push(out, 15280);
					match s {
						[] => true,
						['d', s @ ..] => {
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
				['l', s @ ..] => {
					super::push(out, 15281);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 15282);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 12787);
			match s {
				[] => true,
				['u', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16942);
							s.is_empty()
						}
						['h', s @ ..] => {
							super::push(out, 16943);
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
		['p', s @ ..] => {
			super::push(out, 12788);
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
						['e', s @ ..] => {
							super::push(out, 16944);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17728);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17729);
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
							super::push(out, 16945);
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
						['o', s @ ..] => {
							super::push(out, 16946);
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
				['x', s @ ..] => {
					super::push(out, 15283);
					match s {
						[] => true,
						['e', s @ ..] => {
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
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 12789);
			match s {
				[] => true,
				['t', 'e', s @ ..] => {
					super::push(out, 15284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['s', s @ ..] => {
			super::push(out, 12790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15285);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 16947);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 16948);
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 12791);
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

/// Words ending -flictandus.
pub(crate) fn n27367(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 16949);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 16950);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rctandus.
pub(crate) fn n14524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9512);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -uctandus.
pub(crate) fn n14525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9513);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 12795);
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
		['l', s @ ..] => {
			super::push(out, 12796);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15287);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 16951);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 16952);
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
			super::push(out, 12797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15288);
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

/// Words ending -etandus.
pub(crate) fn n8047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5904);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9514);
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 9515);
			s.is_empty()
		}
		['f', s @ ..] => {
			super::push(out, 9516);
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
		['g', s @ ..] => {
			super::push(out, 9517);
			s.is_empty()
		}
		['h', s @ ..] => {
			super::push(out, 9518);
			s.is_empty()
		}
		['i', s @ ..] => {
			match s {
				[] => true,
				['r', s @ ..] => {
					super::push(out, 12798);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12799);
					match s {
						[] => true,
						['q', s @ ..] => {
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
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9519);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', 'u', 'c', 'o', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', 'r', 't', 's', 'a', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9521);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12800);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9522);
			s.is_empty()
		}
		['v', s @ ..] => {
			super::push(out, 9523);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -itandus.
pub(crate) fn n8048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5905);
	match s {
		[] => true,
		['b', s @ ..] => {
			super::push(out, 9524);
			match s {
				[] => true,
				['a', 'h', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
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
						['d', s @ ..] => {
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
		['c', s @ ..] => {
			super::push(out, 9525);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12802);
					match s {
						[] => true,
						['l', 'l', 'o', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', 'b', 'e', 'f', '\u{1}', s @ ..] => {
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
					match s {
						[] => true,
						['l', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					match s {
						[] => true,
						['e', 'x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12803);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15289);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15290);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15291);
							match s {
								[] => true,
								['c', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15292);
							match s {
								[] => true,
								['n', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['u', s @ ..] => {
							super::push(out, 15293);
							match s {
								[] => true,
								['s', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'r', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
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
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12804);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12805);
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15294);
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
							super::push(out, 15295);
							match s {
								[] => true,
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15296);
							match s {
								[] => true,
								['e', 'h', s @ ..] => {
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
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12806);
					match s {
						[] => true,
						['e', 'v', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9527);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 12807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['l', s @ ..] => {
							super::push(out, 15297);
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15298);
							match s {
								[] => true,
								['c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['x', s @ ..] => {
							super::push(out, 15299);
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
				['i', s @ ..] => {
					super::push(out, 12808);
					match s {
						[] => true,
						['d', s @ ..] => super::t12::n27407(s, out),
						_ => false,
					}
				}
				['o', s @ ..] => super::t12::n21282(s, out),
				['r', s @ ..] => {
					super::push(out, 12810);
					s.is_empty()
				}
				['u', s @ ..] => super::t12::n21284(s, out),
				_ => false,
			}
		}
		['l', s @ ..] => super::t12::n14541(s, out),
		['m', s @ ..] => super::t12::n14542(s, out),
		['n', s @ ..] => super::t12::n14543(s, out),
		['p', s @ ..] => super::t12::n14544(s, out),
		['r', s @ ..] => super::t12::n14545(s, out),
		['s', s @ ..] => super::t12::n14546(s, out),
		['t', s @ ..] => super::t12::n14547(s, out),
		['u', s @ ..] => super::t12::n14548(s, out),
		['v', s @ ..] => super::t12::n14549(s, out),
		_ => false,
	}
}

/// Words ending -ltandus.
pub(crate) fn n8049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5906);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9537);
			match s {
				[] => true,
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
		['u', 's', s @ ..] => {
			super::push(out, 9538);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12837);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15309);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15310);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ntandus.
pub(crate) fn n8050(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5907);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9539);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12838);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 16957);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12839);
					match s {
						[] => true,
						['p', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['m', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 16958);
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9540);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 12840);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12841);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15311);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['l', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 15312);
							s.is_empty()
						}
						['g', s @ ..] => {
							super::push(out, 15313);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15314);
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15315);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15316);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12842);
					match s {
						[] => true,
						['e', 'a', 'r', 'p', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 12843);
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
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
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
						['t', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 12844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -otandus.
pub(crate) fn n8051(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5908);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 9541);
			s.is_empty()
		}
		['m', s @ ..] => {
			super::push(out, 9542);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9543);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 12845);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12846);
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
				['n', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9544);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15318);
							s.is_empty()
						}
						['n', '\u{1}', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 9545);
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

/// Words ending -ptandus.
pub(crate) fn n8052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5909);
	match s {
		[] => true,
		['a', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9546);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12847);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12848);
					s.is_empty()
				}
				_ => false,
			}
		}
		['m', 'e', 't', s @ ..] => {
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
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', 'a', '\u{1}', s @ ..] => {
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
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -rtandus.
pub(crate) fn n8053(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5910);
	match s {
		[] => true,
		['a', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 9547);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12849);
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 12850);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15319);
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
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9548);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 12851);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 12852);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12853);
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
							super::push(out, 15320);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16959);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16960);
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
						['p', s @ ..] => {
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
						['s', s @ ..] => {
							super::push(out, 15321);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16961);
									match s {
										[] => true,
										['a', 'r', 't', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -standus.
pub(crate) fn n8054(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5911);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 9549);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['v', s @ ..] => {
					super::push(out, 12854);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15322);
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
		['b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9550);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', 'n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 12855);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15323);
							match s {
								[] => true,
								['n', 'a', 'm', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15324);
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
				['g', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12856);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 15325);
							match s {
								[] => true,
								['h', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', 'd', '\u{1}', s @ ..] => {
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
							super::push(out, 15326);
							match s {
								[] => true,
								['f', '\u{1}', s @ ..] => {
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
				['t', s @ ..] => {
					super::push(out, 12857);
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
							super::push(out, 15327);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15328);
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
						['o', s @ ..] => {
							super::push(out, 15329);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
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
				['u', 'q', 'n', 'o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9551);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 12858);
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 12859);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 12860);
					match s {
						[] => true,
						['t', s @ ..] => {
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
		['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9552);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9553);
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
		['u', s @ ..] => {
			super::push(out, 9554);
			match s {
				[] => true,
				['b', 'r', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15330);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 16962);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 16963);
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
						['n', 'a', s @ ..] => {
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
						['u', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', 'e', 'v', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', s @ ..] => {
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
						['f', '\u{1}', s @ ..] => {
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

/// Words ending -utandus.
pub(crate) fn n8055(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5912);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 9555);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12861);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 9556);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9557);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9558);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15331);
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
				_ => false,
			}
		}
		['m', s @ ..] => {
			super::push(out, 9559);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12863);
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
					super::push(out, 12864);
					match s {
						[] => true,
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
				['n', 'i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12865);
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
			super::push(out, 9560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 't', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 9561);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12866);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15332);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15333);
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
					super::push(out, 12867);
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
				['p', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 12868);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15334);
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15335);
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
		['r', s @ ..] => {
			super::push(out, 9562);
			match s {
				[] => true,
				['c', 's', s @ ..] => {
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
		['t', s @ ..] => {
			super::push(out, 9563);
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

/// Words ending -scendus.
pub(crate) fn n8089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5945);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9611);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 12920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16975);
									s.is_empty()
								}
								['r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12921);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15360);
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
				['r', s @ ..] => {
					super::push(out, 12922);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9612);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12923);
					s.is_empty()
				}
				['i', s @ ..] => {
					super::push(out, 12924);
					match s {
						[] => true,
						['u', 'q', s @ ..] => {
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 12925);
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 12926);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12927);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15361);
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
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15362);
							s.is_empty()
						}
						['t', s @ ..] => {
							super::push(out, 15363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 12928);
					s.is_empty()
				}
				['u', s @ ..] => {
					super::push(out, 12929);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16976);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 17736);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 17737);
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
								['s', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['v', s @ ..] => {
					super::push(out, 12930);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 'f', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9613);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12931);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15364);
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
										_ => false,
									}
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 15365);
							match s {
								[] => true,
								['p', s @ ..] => {
									match s {
										[] => true,
										['e', s @ ..] => {
											super::push(out, 17738);
											s.is_empty()
										}
										['m', s @ ..] => {
											super::push(out, 17739);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							super::push(out, 15366);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 16977);
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
				['d', s @ ..] => {
					super::push(out, 12932);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15368);
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
					super::push(out, 12933);
					s.is_empty()
				}
				['h', s @ ..] => {
					super::push(out, 12934);
					s.is_empty()
				}
				['l', s @ ..] => {
					super::push(out, 12935);
					s.is_empty()
				}
				['m', s @ ..] => super::t12::n21535(s, out),
				['n', s @ ..] => {
					super::push(out, 12937);
					match s {
						[] => true,
						['i', s @ ..] => super::t12::n27647(s, out),
						['u', s @ ..] => super::t12::n27648(s, out),
						_ => false,
					}
				}
				['p', s @ ..] => super::t12::n21537(s, out),
				['r', s @ ..] => super::t12::n21538(s, out),
				['t', s @ ..] => super::t12::n21539(s, out),
				['v', s @ ..] => super::t12::n21540(s, out),
				_ => false,
			}
		}
		['o', s @ ..] => super::t12::n14653(s, out),
		_ => false,
	}
}

/// Words ending -ucendus.
pub(crate) fn n8090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5946);
	match s {
		[] => true,
		['d', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 12944);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 12945);
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
					super::push(out, 12946);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15374);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 15375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15376);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', s @ ..] => {
							super::push(out, 15377);
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
					super::push(out, 12947);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12948);
					match s {
						[] => true,
						['i', s @ ..] => {
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
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12949);
					match s {
						[] => true,
						['r', s @ ..] => {
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['t', 'n', 'i', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 12950);
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
								['t', s @ ..] => {
									match s {
										[] => true,
										['b', 'u', 's', '\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['s', s @ ..] => {
					super::push(out, 12951);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							super::push(out, 15378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9615);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'o', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -idendus.
pub(crate) fn n8095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5951);
	match s {
		[] => true,
		['c', s @ ..] => {
			super::push(out, 9629);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12962);
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12963);
					match s {
						[] => true,
						['d', s @ ..] => {
							super::push(out, 15381);
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 12964);
					match s {
						[] => true,
						['i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 16983);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 16984);
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
							super::push(out, 15382);
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 12965);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 12966);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12967);
					match s {
						[] => true,
						['n', s @ ..] => {
							super::push(out, 15383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['d', s @ ..] => {
			super::push(out, 9630);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 9631);
			match s {
				[] => true,
				['e', s @ ..] => {
					super::push(out, 12968);
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9632);
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
					super::push(out, 12969);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
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
		['s', s @ ..] => {
			super::push(out, 9633);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 12970);
					match s {
						[] => true,
						['o', s @ ..] => {
							super::push(out, 15384);
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15385);
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 12971);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 12972);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 15386);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 12973);
					s.is_empty()
				}
				['s', s @ ..] => {
					super::push(out, 12974);
					s.is_empty()
				}
				_ => false,
			}
		}
		['v', s @ ..] => {
			super::push(out, 9634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 12975);
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
					super::push(out, 12976);
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
					super::push(out, 12977);
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
					super::push(out, 12978);
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

/// Words ending -ndendus.
pub(crate) fn n8096(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5952);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9635);
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
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
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
						['p', 'o', '\u{1}', s @ ..] => {
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
		['e', s @ ..] => {
			super::push(out, 9636);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 12979);
					match s {
						[] => true,
						['c', s @ ..] => {
							super::push(out, 15388);
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15389);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15390);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 16985);
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 16986);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17741);
											s.is_empty()
										}
										['d', s @ ..] => {
											super::push(out, 17742);
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
									super::push(out, 16987);
									match s {
										[] => true,
										['a', s @ ..] => {
											super::push(out, 17743);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 17744);
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									super::push(out, 16988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 12980);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15391);
							s.is_empty()
						}
						['f', s @ ..] => {
							super::push(out, 15392);
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
					super::push(out, 12981);
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
								['e', s @ ..] => {
									super::push(out, 16989);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 17745);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 17746);
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
								['n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 12982);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15393);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['c', 'c', 'o', 'l', 'f', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['v', 'r', 'a', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['m', 'i', s @ ..] => {
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
						['n', 'i', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => super::t12::n32655(s, out),
								_ => false,
							}
						}
						['p', 'a', '\u{1}', s @ ..] => super::t12::n27729(s, out),
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t12::n27730(s, out),
						['s', 'u', 's', '\u{1}', s @ ..] => super::t12::n27731(s, out),
						['x', 'e', '\u{1}', s @ ..] => super::t12::n27732(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t12::n21619(s, out),
				['t', s @ ..] => super::t12::n21620(s, out),
				['v', s @ ..] => super::t12::n21621(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t12::n14679(s, out),
		['o', s @ ..] => super::t12::n14680(s, out),
		['u', s @ ..] => super::t12::n14681(s, out),
		_ => false,
	}
}

/// Words ending -lodendus.
pub(crate) fn n14682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9640);
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

/// Words ending -rodendus.
pub(crate) fn n14683(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 12990);
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
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rdendus.
pub(crate) fn n8098(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5954);
	match s {
		[] => true,
		['a', s @ ..] => {
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
			super::push(out, 9641);
			match s {
				[] => true,
				['p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15404);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
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
		['o', s @ ..] => {
			super::push(out, 9642);
			match s {
				[] => true,
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
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
								['d', s @ ..] => {
									super::push(out, 16995);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -udendus.
pub(crate) fn n8099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5955);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9643);
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
		['c', s @ ..] => {
			super::push(out, 9644);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 12991);
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
		['l', s @ ..] => {
			super::push(out, 9645);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['c', s @ ..] => {
					super::push(out, 12992);
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
					super::push(out, 12993);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15405);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 15406);
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
		['p', s @ ..] => {
			super::push(out, 9646);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9647);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 12994);
					s.is_empty()
				}
				['t', s @ ..] => {
					super::push(out, 12995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15407);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 16996);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 16997);
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
							super::push(out, 15408);
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
		_ => false,
	}
}

/// Words ending -ligendus.
pub(crate) fn n14700(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9653);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 13006);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
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
				['s', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 13007);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 15415);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15416);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 16998);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 16999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['g', s @ ..] => {
			super::push(out, 13008);
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 13009);
			s.is_empty()
		}
		['l', s @ ..] => {
			super::push(out, 13010);
			match s {
				[] => true,
				['e', 't', 'n', 'i', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -rigendus.
pub(crate) fn n14701(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9654);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13011);
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
		['f', s @ ..] => {
			super::push(out, 13012);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 13013);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13014);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
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
								['e', 'r', '\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -lgendus.
pub(crate) fn n8103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5959);
	match s {
		[] => true,
		['u', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ngendus.
pub(crate) fn n8104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5960);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9657);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['p', s @ ..] => {
					super::push(out, 13016);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15417);
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
				['r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['t', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', s @ ..] => {
			super::push(out, 9658);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13017);
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
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', 'a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['f', s @ ..] => {
					super::push(out, 13018);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15418);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 15419);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15420);
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
				['l', s @ ..] => {
					super::push(out, 13019);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15421);
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
						['l', s @ ..] => {
							super::push(out, 15422);
							match s {
								[] => true,
								['o', 'p', '\u{1}', s @ ..] => {
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
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13020);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15423);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
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
						['p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13021);
					match s {
						[] => true,
						['f', s @ ..] => {
							super::push(out, 15424);
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 17000);
									match s {
										[] => true,
										['d', s @ ..] => {
											super::push(out, 17749);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 17750);
											s.is_empty()
										}
										_ => false,
									}
								}
								['f', s @ ..] => {
									super::push(out, 17001);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17002);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 17751);
											s.is_empty()
										}
										['o', s @ ..] => {
											super::push(out, 17752);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', s @ ..] => super::t12::n27872(s, out),
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13022);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
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
						['r', 'e', 'p', '\u{1}', s @ ..] => super::t12::n27877(s, out),
						['t', 'a', '\u{1}', s @ ..] => super::t12::n27878(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => super::t12::n14707(s, out),
		_ => false,
	}
}

/// Words ending -rgendus.
pub(crate) fn n8106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5962);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9660);
			match s {
				[] => true,
				['p', 's', s @ ..] => {
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
							super::push(out, 15431);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									super::push(out, 17004);
									s.is_empty()
								}
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
			super::push(out, 9661);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13027);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15432);
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
							super::push(out, 15433);
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
					super::push(out, 13028);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17005);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 17755);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 17756);
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
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13029);
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
		['u', s @ ..] => {
			super::push(out, 9662);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'e', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15434);
							match s {
								[] => true,
								['i', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['s', 'a', '\u{1}', s @ ..] => {
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

/// Words ending -ugendus.
pub(crate) fn n8107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5963);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9663);
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
		['l', s @ ..] => {
			super::push(out, 9664);
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 9665);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -aciendus.
pub(crate) fn n14717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9666);
	match s {
		[] => true,
		['f', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15441);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 17006);
							match s {
								[] => true,
								['a', s @ ..] => {
									match s {
										[] => true,
										['l', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['t', s @ ..] => {
											super::push(out, 18021);
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
								['u', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['c', s @ ..] => {
							super::push(out, 17007);
							match s {
								[] => true,
								['i', 'l', s @ ..] => {
									match s {
										[] => true,
										['d', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['l', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 17008);
							match s {
								[] => true,
								['a', 'm', s @ ..] => {
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
								['n', 'a', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['g', s @ ..] => {
							super::push(out, 17009);
							match s {
								[] => true,
								['r', 'e', 'p', 'x', 'e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['l', s @ ..] => {
							super::push(out, 17010);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17757);
									match s {
										[] => true,
										['c', 'r', 'e', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['m', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										['v', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17758);
									match s {
										[] => true,
										['b', 'a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['m', s @ ..] => {
							super::push(out, 17011);
							match s {
								[] => true,
								['e', 'r', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 17012);
							match s {
								[] => true,
								['e', 'b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['o', 'm', 'm', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['p', s @ ..] => {
							super::push(out, 17013);
							match s {
								[] => true,
								['e', 't', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 't', 's', s @ ..] => {
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
						['r', s @ ..] => {
							super::push(out, 17014);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17759);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 18022);
											s.is_empty()
										}
										['r', s @ ..] => {
											super::push(out, 18023);
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
								['b', s @ ..] => {
									super::push(out, 17760);
									match s {
										[] => true,
										['a', 'f', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17761);
									match s {
										[] => true,
										['o', 't', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['t', s @ ..] => {
									super::push(out, 17762);
									match s {
										[] => true,
										['u', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['u', s @ ..] => {
									super::push(out, 17763);
									match s {
										[] => true,
										['d', 'b', 'o', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['t', 'a', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 17015);
							match s {
								[] => true,
								['c', s @ ..] => super::t12::n35471(s, out),
								['q', s @ ..] => super::t12::n35472(s, out),
								['s', s @ ..] => super::t12::n35473(s, out),
								_ => false,
							}
						}
						['v', s @ ..] => super::t12::n32783(s, out),
						_ => false,
					}
				}
				['i', 'r', 'c', 'u', 'l', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['l', s @ ..] => super::t12::n27942(s, out),
				['s', 'i', 't', 'a', 's', '\u{1}', s @ ..] => super::t12::n27943(s, out),
				_ => false,
			}
		}
		['i', s @ ..] => super::t12::n21743(s, out),
		['l', '\u{1}', s @ ..] => super::t12::n21744(s, out),
		_ => false,
	}
}

/// Words ending -iciendus.
pub(crate) fn n14720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9667);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 13039);
			s.is_empty()
		}
		['a', s @ ..] => {
			super::push(out, 13040);
			match s {
				[] => true,
				['r', 't', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['b', s @ ..] => {
			super::push(out, 13041);
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
			super::push(out, 13042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 15442);
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 15443);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 15444);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15445);
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
		['f', s @ ..] => {
			super::push(out, 13043);
			match s {
				[] => true,
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15446);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 17017);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 17018);
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
						['r', s @ ..] => {
							super::push(out, 17019);
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
				['f', s @ ..] => {
					super::push(out, 15447);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', '\u{1}', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 15448);
					match s {
						[] => true,
						['i', s @ ..] => {
							super::push(out, 17020);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 17021);
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
				['r', s @ ..] => {
					super::push(out, 15449);
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
		['i', s @ ..] => {
			super::push(out, 13044);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 15450);
					match s {
						[] => true,
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 15451);
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
					super::push(out, 15452);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 17022);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 17023);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 17024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 17025);
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
					super::push(out, 15453);
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
					super::push(out, 15454);
					match s {
						[] => true,
						['r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15455);
					match s {
						[] => true,
						['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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
		['l', s @ ..] => {
			super::push(out, 13045);
			match s {
				[] => true,
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15456);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 15457);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 'p', '\u{1}', s @ ..] => {
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
		['m', s @ ..] => {
			super::push(out, 13046);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13047);
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
			super::push(out, 13048);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 15458);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 15459);
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
		['p', s @ ..] => {
			super::push(out, 13049);
			match s {
				[] => true,
				['s', s @ ..] => super::t12::n27979(s, out),
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 13050);
			match s {
				[] => true,
				['e', 't', 'n', 'i', '\u{1}', s @ ..] => {
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

/// Words ending -fulciendus.
pub(crate) fn n21759(s: &[char], _out: &mut Path<'_>) -> bool {
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

/// Words ending -nciendus.
pub(crate) fn n14722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9669);
	match s {
		[] => true,
		['i', 'v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 15460);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 17034);
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
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rciendus.
pub(crate) fn n14723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9670);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 13051);
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
							super::push(out, 17035);
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
			super::push(out, 13052);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['e', 'r', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 17036);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17772);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17773);
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
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -sciendus.
pub(crate) fn n14724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9671);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word exciendus.
pub(crate) fn n14725(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -diendus.
pub(crate) fn n8112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5968);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9672);
			match s {
				[] => true,
				['r', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15461);
							match s {
								[] => true,
								['r', 't', 'e', 'r', '\u{1}', s @ ..] => {
									s.is_empty()
								}
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
			super::push(out, 9673);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13053);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 13054);
					match s {
						[] => true,
						['b', 'o', s @ ..] => {
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
				['p', s @ ..] => {
					super::push(out, 13055);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13056);
					match s {
						[] => true,
						['g', s @ ..] => {
							match s {
								[] => true,
								['e', s @ ..] => {
									super::push(out, 17037);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17774);
											s.is_empty()
										}
										['a', s @ ..] => {
											super::push(out, 17775);
											match s {
												[] => true,
												['r', 'p', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['d', s @ ..] => {
											super::push(out, 17776);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['r', s @ ..] => {
											super::push(out, 17777);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['t', s @ ..] => {
											super::push(out, 17778);
											match s {
												[] => true,
												['n', 'a', '\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['i', s @ ..] => {
									super::push(out, 17038);
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17039);
									s.is_empty()
								}
								['o', s @ ..] => {
									super::push(out, 17040);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 17041);
									s.is_empty()
								}
								['s', s @ ..] => {
									super::push(out, 17042);
									s.is_empty()
								}
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
			super::push(out, 9674);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9675);
			match s {
				[] => true,
				['a', 'l', 'b', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15462);
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
		['o', s @ ..] => {
			super::push(out, 9676);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['f', s @ ..] => {
					super::push(out, 13057);
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
							super::push(out, 15463);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17043);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 17044);
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
						['f', s @ ..] => {
							super::push(out, 15464);
							match s {
								[] => true,
								['a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', '\u{1}', s @ ..] => {
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
							super::push(out, 15465);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17045);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17046);
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
						['r', s @ ..] => {
							super::push(out, 15466);
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
							super::push(out, 15467);
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
				['t', s @ ..] => {
					super::push(out, 13058);
					match s {
						[] => true,
						['s', 'u', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9677);
			match s {
				[] => true,
				['o', s @ ..] => {
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
			super::push(out, 9678);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							match s {
								[] => true,
								['o', '\u{1}', s @ ..] => super::t12::n32861(s, out),
								['u', 's', '\u{1}', s @ ..] => super::t12::n32862(s, out),
								_ => false,
							}
						}
						['n', 'i', '\u{1}', s @ ..] => super::t12::n28012(s, out),
						['x', 'e', '\u{1}', s @ ..] => super::t12::n28013(s, out),
						_ => false,
					}
				}
				['r', s @ ..] => super::t12::n21777(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -eiendus.
pub(crate) fn n8113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5969);
	s.is_empty()
}

/// Words ending -fiendus.
pub(crate) fn n8114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5970);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 9679);
			s.is_empty()
		}
		['f', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -giendus.
pub(crate) fn n8115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5971);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9680);
			s.is_empty()
		}
		['r', s @ ..] => {
			super::push(out, 9681);
			match s {
				[] => true,
				['a', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['i', s @ ..] => {
							super::push(out, 15468);
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
		['u', s @ ..] => {
			super::push(out, 9682);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13061);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15469);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['f', s @ ..] => {
							super::push(out, 15470);
							match s {
								[] => true,
								['e', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['i', 'd', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15471);
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 13062);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -liendus.
pub(crate) fn n8116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5972);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9683);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9684);
			match s {
				[] => true,
				['p', 'e', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15472);
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
			super::push(out, 9685);
			match s {
				[] => true,
				['b', s @ ..] => {
					super::push(out, 13063);
					match s {
						[] => true,
						['a', 't', 's', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['n', s @ ..] => {
									super::push(out, 17047);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 17779);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 17780);
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
				['s', s @ ..] => {
					super::push(out, 13064);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15473);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15474);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17048);
									s.is_empty()
								}
								['r', s @ ..] => {
									super::push(out, 17049);
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15475);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15476);
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15477);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['l', s @ ..] => {
			super::push(out, 9686);
			match s {
				[] => true,
				['o', s @ ..] => {
					super::push(out, 13065);
					match s {
						[] => true,
						['m', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17781);
											s.is_empty()
										}
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
				['u', s @ ..] => {
					super::push(out, 13066);
					match s {
						[] => true,
						['b', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 17782);
											s.is_empty()
										}
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9687);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13067);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15478);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 15479);
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
					super::push(out, 13068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -miendus.
pub(crate) fn n8117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5973);
	match s {
		[] => true,
		['r', 'o', 'd', s @ ..] => {
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
					super::push(out, 13069);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -niendus.
pub(crate) fn n8118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5974);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9688);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13070);
					match s {
						[] => true,
						['i', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 13071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9689);
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 13072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15480);
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
				['v', s @ ..] => {
					super::push(out, 13073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15481);
							match s {
								[] => true,
								['r', 't', 'n', 'o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['b', s @ ..] => {
							super::push(out, 15482);
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
							super::push(out, 15483);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17050);
									s.is_empty()
								}
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17051);
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
							super::push(out, 15484);
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
							super::push(out, 15485);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15486);
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
						['s', s @ ..] => {
							super::push(out, 15487);
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
		['g', s @ ..] => {
			super::push(out, 9690);
			match s {
				[] => true,
				['i', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', s @ ..] => {
							super::push(out, 15488);
							match s {
								[] => true,
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
		['i', s @ ..] => {
			super::push(out, 9691);
			match s {
				[] => true,
				['f', s @ ..] => {
					super::push(out, 13074);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15489);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15490);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17052);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 17053);
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
				['l', s @ ..] => {
					super::push(out, 13075);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15491);
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
		['m', s @ ..] => {
			super::push(out, 9692);
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9693);
			s.is_empty()
		}
		['u', s @ ..] => {
			super::push(out, 9694);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15492);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17054);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 17055);
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
							match s {
								[] => true,
								['o', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['r', 'f', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -piendus.
pub(crate) fn n8119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5975);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9695);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', 't', 'n', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							super::push(out, 15493);
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
				_ => false,
			}
		}
		['e', s @ ..] => {
			super::push(out, 9696);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13076);
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
									super::push(out, 17056);
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
					super::push(out, 13077);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['m', s @ ..] => {
							super::push(out, 15494);
							match s {
								[] => true,
								['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15495);
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
			super::push(out, 9697);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13078);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15496);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17057);
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17058);
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
								['r', s @ ..] => {
									super::push(out, 17059);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13079);
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15497);
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15498);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 17060);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 17061);
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
							super::push(out, 15499);
							s.is_empty()
						}
						['o', s @ ..] => {
							super::push(out, 15500);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15501);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9698);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15502);
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
		['u', s @ ..] => {
			super::push(out, 9699);
			match s {
				[] => true,
				['c', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
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

/// Words ending -riendus.
pub(crate) fn n8120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5976);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9700);
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9701);
			match s {
				[] => true,
				['p', s @ ..] => {
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9702);
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
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['h', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['m', s @ ..] => {
					super::push(out, 13080);
					match s {
						[] => true,
						['e', s @ ..] => {
							super::push(out, 15503);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 17062);
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
								_ => false,
							}
						}
						['m', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['x', 'e', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9703);
			match s {
				[] => true,
				['a', 's', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15504);
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
		['t', s @ ..] => {
			super::push(out, 9704);
			match s {
				[] => true,
				['u', 'n', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15505);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15506);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['u', s @ ..] => {
			super::push(out, 9705);
			match s {
				[] => true,
				['a', s @ ..] => {
					super::push(out, 13081);
					match s {
						[] => true,
						['h', s @ ..] => {
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
						_ => false,
					}
				}
				['g', s @ ..] => {
					super::push(out, 13082);
					match s {
						[] => true,
						['i', 'l', s @ ..] => {
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
				['s', s @ ..] => {
					super::push(out, 13083);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tiendus.
pub(crate) fn n8121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5977);
	match s {
		[] => true,
		['a', s @ ..] => {
			super::push(out, 9706);
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
			super::push(out, 9707);
			match s {
				[] => true,
				['m', s @ ..] => {
					super::push(out, 13084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', 'a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15507);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17063);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17064);
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
							super::push(out, 15508);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
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
						['r', 'e', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['p', s @ ..] => {
					super::push(out, 13085);
					s.is_empty()
				}
				['r', s @ ..] => {
					super::push(out, 13086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9708);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['m', s @ ..] => {
							super::push(out, 15509);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['e', s @ ..] => {
									super::push(out, 17065);
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
						['s', s @ ..] => {
							super::push(out, 15510);
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
								['n', s @ ..] => {
									super::push(out, 17066);
									match s {
										[] => true,
										['o', 'c', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['s', s @ ..] => {
									match s {
										[] => true,
										['a', '\u{1}', s @ ..] => {
											s.is_empty()
										}
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
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9709);
			match s {
				[] => true,
				['a', 'p', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15511);
							match s {
								[] => true,
								['d', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['i', s @ ..] => {
							match s {
								[] => true,
								['b', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['r', 'd', 'a', 'u', 'q', '\u{1}', s @ ..] => {
									s.is_empty()
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
						['n', 'o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', 's', s @ ..] => {
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
		['s', s @ ..] => {
			super::push(out, 9710);
			match s {
				[] => true,
				['e', 'v', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', s @ ..] => {
			super::push(out, 9711);
			match s {
				[] => true,
				['u', 'l', 'g', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15512);
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
		['u', s @ ..] => {
			super::push(out, 9712);
			match s {
				[] => true,
				['c', 'r', 'e', 'p', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -viendus.
pub(crate) fn n8122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5978);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['r', 'e', 's', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', 'u', 's', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', s @ ..] => {
					super::push(out, 13087);
					match s {
						[] => true,
						['a', s @ ..] => {
							super::push(out, 15513);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 15514);
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
					super::push(out, 13088);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['s', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -plendus.
pub(crate) fn n8128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5983);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 9715);
			match s {
				[] => true,
				['d', s @ ..] => {
					super::push(out, 13092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['r', s @ ..] => {
					super::push(out, 13093);
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
		['p', s @ ..] => {
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
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -culendus.
pub(crate) fn n14789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9716);
	s.is_empty()
}

/// Words ending -sulendus.
pub(crate) fn n14790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9717);
	s.is_empty()
}

/// Words ending -ponendus.
pub(crate) fn n14810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9728);
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
			super::push(out, 13108);
			match s {
				[] => true,
				['a', 'r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', s @ ..] => {
					super::push(out, 15529);
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
					super::push(out, 15530);
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
		['m', s @ ..] => {
			super::push(out, 13109);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 15531);
					match s {
						[] => true,
						['c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13110);
			match s {
				[] => true,
				['i', '\u{1}', s @ ..] => {
					super::push(out, 15532);
					s.is_empty()
				}
				['o', s @ ..] => {
					super::push(out, 15533);
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
			super::push(out, 13111);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['p', s @ ..] => {
			super::push(out, 13112);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['o', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['u', 's', s @ ..] => {
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
		['r', s @ ..] => {
			super::push(out, 13113);
			match s {
				[] => true,
				['e', s @ ..] => {
					match s {
						[] => true,
						['p', 'u', 's', '\u{1}', s @ ..] => {
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
			super::push(out, 13114);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 15534);
					match s {
						[] => true,
						['d', s @ ..] => {
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
				['n', s @ ..] => {
					super::push(out, 15535);
					match s {
						[] => true,
						['a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['t', 's', 'o', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rnendus.
pub(crate) fn n8142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5996);
	match s {
		[] => true,
		['e', s @ ..] => {
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13115);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15536);
							match s {
								[] => true,
								['d', s @ ..] => {
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
								['s', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['n', s @ ..] => {
							super::push(out, 15537);
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
				['p', s @ ..] => {
					super::push(out, 13116);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['s', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['t', s @ ..] => {
					super::push(out, 13117);
					match s {
						[] => true,
						['s', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 17071);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['b', s @ ..] => {
									super::push(out, 17072);
									match s {
										[] => true,
										['u', 's', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['e', s @ ..] => {
									super::push(out, 17073);
									match s {
										[] => true,
										['d', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['n', s @ ..] => {
									super::push(out, 17074);
									match s {
										[] => true,
										['i', s @ ..] => {
											super::push(out, 17783);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													s.is_empty()
												}
												_ => false,
											}
										}
										['o', s @ ..] => {
											super::push(out, 17784);
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
									super::push(out, 17075);
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
		_ => false,
	}
}

/// The word aggerendus.
pub(crate) fn n28232(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word suggerendus.
pub(crate) fn n28233(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -igerendus.
pub(crate) fn n21960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13139);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ngerendus.
pub(crate) fn n21961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13140);
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

/// Words ending -herendus.
pub(crate) fn n14822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9736);
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

/// Words ending -merendus.
pub(crate) fn n14823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9737);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13141);
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
			super::push(out, 13142);
			match s {
				[] => true,
				['o', 'c', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 13143);
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

/// Words ending -serendus.
pub(crate) fn n14824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9738);
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
			super::push(out, 13144);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['i', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 13145);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 15545);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 15546);
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
		['r', 'e', 't', 'n', 'i', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['s', s @ ..] => {
			super::push(out, 13146);
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
							super::push(out, 17077);
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
		['x', 'e', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -terendus.
pub(crate) fn n14825(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9739);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 13147);
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
			super::push(out, 13148);
			match s {
				[] => true,
				['d', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 13149);
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
			super::push(out, 13150);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
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

/// Words ending -uerendus.
pub(crate) fn n14826(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9740);
	match s {
		[] => true,
		['q', 'n', 'o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -verendus.
pub(crate) fn n14827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9741);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'u', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'r', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// The word aegrendus.
pub(crate) fn n8150(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -irendus.
pub(crate) fn n8151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6003);
	match s {
		[] => true,
		['u', 'q', s @ ..] => {
			match s {
				[] => true,
				['c', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['d', 'a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['e', 'r', '\u{1}', s @ ..] => {
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

/// Words ending -rrendus.
pub(crate) fn n8152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6004);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9742);
			match s {
				[] => true,
				['t', s @ ..] => {
					super::push(out, 13151);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15547);
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
							super::push(out, 15548);
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
						['s', s @ ..] => {
							match s {
								[] => true,
								['b', 'a', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['p', 'a', '\u{1}', s @ ..] => {
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
					super::push(out, 13152);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17078);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 17079);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 17080);
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
				_ => false,
			}
		}
		['o', s @ ..] => {
			super::push(out, 9743);
			match s {
				[] => true,
				['h', s @ ..] => {
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
				['t', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15549);
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
		['u', s @ ..] => {
			super::push(out, 9744);
			match s {
				[] => true,
				['c', s @ ..] => {
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
							super::push(out, 15550);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17081);
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
							super::push(out, 15551);
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

/// Words ending -urendus.
pub(crate) fn n8153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6005);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
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

/// The word deflectendus.
pub(crate) fn n36621(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word reflectendus.
pub(crate) fn n36622(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -itendus.
pub(crate) fn n8162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6014);
	match s {
		[] => true,
		['f', s @ ..] => {
			super::push(out, 9750);
			match s {
				[] => true,
				['n', s @ ..] => {
					super::push(out, 13156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['n', s @ ..] => {
			super::push(out, 9751);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['b', s @ ..] => {
					super::push(out, 13157);
					match s {
						[] => true,
						['o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 13158);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 13159);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15555);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15556);
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
								_ => false,
							}
						}
						['o', s @ ..] => {
							super::push(out, 15557);
							match s {
								[] => true,
								['p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15558);
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
					super::push(out, 13160);
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
					super::push(out, 13161);
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

/// Words ending -rtendus.
pub(crate) fn n8164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6016);
	match s {
		[] => true,
		['e', 'v', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 13162);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 13163);
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
					super::push(out, 13164);
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
					super::push(out, 13165);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15559);
							s.is_empty()
						}
						['a', 'r', 'p', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['d', s @ ..] => {
							super::push(out, 15560);
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
					super::push(out, 13166);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['n', s @ ..] => {
					super::push(out, 13167);
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
					super::push(out, 13168);
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
					super::push(out, 13169);
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

/// Words ending -ttendus.
pub(crate) fn n8166(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6018);
	match s {
		[] => true,
		['i', 'm', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['a', s @ ..] => {
					super::push(out, 13171);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['b', s @ ..] => {
					super::push(out, 13172);
					match s {
						[] => true,
						['u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['d', s @ ..] => {
					super::push(out, 13173);
					match s {
						[] => true,
						['a', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['e', s @ ..] => {
					super::push(out, 13174);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15561);
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15562);
							match s {
								[] => true,
								['r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['d', s @ ..] => {
							super::push(out, 15563);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['r', s @ ..] => {
							super::push(out, 15564);
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
					super::push(out, 13175);
					match s {
						[] => true,
						['d', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['m', s @ ..] => {
					super::push(out, 13176);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['o', 'c', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['u', s @ ..] => {
							match s {
								[] => true,
								['c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
									s.is_empty()
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
				['n', s @ ..] => {
					super::push(out, 13177);
					match s {
						[] => true,
						['i', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13178);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 15565);
							s.is_empty()
						}
						['r', s @ ..] => {
							super::push(out, 15566);
							match s {
								[] => true,
								['p', s @ ..] => {
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
										['p', 'a', '\u{1}', s @ ..] => {
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
				['r', s @ ..] => {
					super::push(out, 13179);
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
					super::push(out, 13180);
					match s {
						[] => true,
						['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['u', s @ ..] => {
					super::push(out, 13181);
					match s {
						[] => true,
						['n', 'a', 'm', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -utendus.
pub(crate) fn n8167(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6019);
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
		['e', 'd', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -espuendus.
pub(crate) fn n22070(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13184);
	match s {
		[] => true,
		['d', s @ ..] => {
			super::push(out, 15572);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 15573);
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

/// The word conspuendus.
pub(crate) fn n28362(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// The word exspuendus.
pub(crate) fn n22072(s: &[char], _out: &mut Path<'_>) -> bool {
	s.is_empty()
}

/// Words ending -quendus.
pub(crate) fn n8175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6025);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 9755);
			match s {
				[] => true,
				['s', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', s @ ..] => {
							super::push(out, 15574);
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
							super::push(out, 15575);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17088);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17089);
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
							super::push(out, 15576);
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
		['n', s @ ..] => {
			super::push(out, 9756);
			match s {
				[] => true,
				['i', 'l', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17090);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										['e', s @ ..] => {
											super::push(out, 17787);
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
		['o', s @ ..] => {
			super::push(out, 9757);
			match s {
				[] => true,
				['c', s @ ..] => {
					super::push(out, 13186);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15577);
							match s {
								[] => true,
								['d', s @ ..] => {
									super::push(out, 17092);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['r', s @ ..] => {
									super::push(out, 17093);
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
						['s', 'i', 'd', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['l', s @ ..] => {
					super::push(out, 13187);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['a', s @ ..] => {
							super::push(out, 15578);
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
							super::push(out, 15579);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 17094);
									s.is_empty()
								}
								['a', s @ ..] => {
									super::push(out, 17095);
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
							super::push(out, 15580);
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
							super::push(out, 15581);
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
		['r', s @ ..] => {
			super::push(out, 9758);
			match s {
				[] => true,
				['o', 't', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['b', 'o', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['e', s @ ..] => {
							super::push(out, 15582);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17096);
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

/// Words ending -ruendus.
pub(crate) fn n8176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6026);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', s @ ..] => {
			super::push(out, 9759);
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
		['e', s @ ..] => {
			super::push(out, 9760);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['f', s @ ..] => {
			super::push(out, 9761);
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
			super::push(out, 9762);
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
			super::push(out, 9763);
			match s {
				[] => true,
				['r', 'p', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		['r', s @ ..] => {
			super::push(out, 9764);
			match s {
				[] => true,
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
		['t', s @ ..] => {
			super::push(out, 9765);
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
						['b', s @ ..] => {
							super::push(out, 15583);
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
						['e', s @ ..] => {
							super::push(out, 15584);
							match s {
								[] => true,
								['a', s @ ..] => {
									super::push(out, 17097);
									match s {
										[] => true,
										['r', 'p', '\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['d', s @ ..] => {
									super::push(out, 17098);
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
							super::push(out, 15585);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17099);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17100);
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
						['r', 'e', 'p', 'u', 's', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['x', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
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
		_ => false,
	}
}

/// Words ending -suendus.
pub(crate) fn n8177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6027);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['d', 'a', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9766);
			match s {
				[] => true,
				['i', s @ ..] => {
					super::push(out, 13188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						_ => false,
					}
				}
				['o', s @ ..] => {
					super::push(out, 13189);
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
			super::push(out, 9767);
			match s {
				[] => true,
				['a', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				['i', 'd', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -tuendus.
pub(crate) fn n8178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6028);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['a', 't', 's', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['b', 'o', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', 'm', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['i', s @ ..] => {
			super::push(out, 9768);
			match s {
				[] => true,
				['t', 's', s @ ..] => {
					match s {
						[] => true,
						['b', s @ ..] => {
							super::push(out, 15586);
							match s {
								[] => true,
								['u', 's', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								_ => false,
							}
						}
						['e', s @ ..] => {
							super::push(out, 15587);
							match s {
								[] => true,
								['a', 'r', 'p', '\u{1}', s @ ..] => {
									s.is_empty()
								}
								['d', s @ ..] => {
									super::push(out, 17101);
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
							super::push(out, 15588);
							match s {
								[] => true,
								['i', s @ ..] => {
									super::push(out, 17102);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											s.is_empty()
										}
										_ => false,
									}
								}
								['o', s @ ..] => {
									super::push(out, 17103);
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
							super::push(out, 15589);
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
		['m', 'u', 'c', 'r', 'i', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['n', s @ ..] => {
			super::push(out, 9769);
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
		['u', s @ ..] => {
			super::push(out, 9770);
			match s {
				[] => true,
				['f', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							s.is_empty()
						}
						['c', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['f', 'e', '\u{1}', s @ ..] => {
							s.is_empty()
						}
						['n', s @ ..] => {
							super::push(out, 15590);
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

/// Words ending -imovendus.
pub(crate) fn n22116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13196);
	match s {
		[] => true,
		['d', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -mmovendus.
pub(crate) fn n22117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13197);
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

/// Words ending -omovendus.
pub(crate) fn n22119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13198);
	match s {
		[] => true,
		['r', 'p', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rmovendus.
pub(crate) fn n22120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13199);
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

/// Words ending -smovendus.
pub(crate) fn n22121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13200);
	match s {
		[] => true,
		['n', 'a', 'r', 't', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -vovendus.
pub(crate) fn n14907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9775);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			s.is_empty()
		}
		['e', s @ ..] => {
			super::push(out, 13201);
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

/// Words ending -citabundus.
pub(crate) fn n28473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15617);
	match s {
		[] => true,
		['s', 'o', s @ ..] => {
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					s.is_empty()
				}
				['n', '\u{1}', s @ ..] => {
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -gitabundus.
pub(crate) fn n28474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15618);
	s.is_empty()
}

/// Words ending -litabundus.
pub(crate) fn n28475(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15619);
	match s {
		[] => true,
		['c', 'i', 'r', 'e', 'p', '\u{1}', s @ ..] => {
			super::push(out, 17112);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -sitabundus.
pub(crate) fn n28476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15620);
	s.is_empty()
}

/// Words ending -titabundus.
pub(crate) fn n28477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15621);
	s.is_empty()
}

/// Words ending -vitabundus.
pub(crate) fn n28478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15622);
	s.is_empty()
}

/// Words ending -ltabundus.
pub(crate) fn n22160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13228);
	match s {
		[] => true,
		['u', 's', 'n', s @ ..] => {
			super::push(out, 15623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ntabundus.
pub(crate) fn n22161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13229);
	match s {
		[] => true,
		['a', 'c', '\u{1}', s @ ..] => {
			s.is_empty()
		}
		['o', s @ ..] => {
			super::push(out, 15624);
			match s {
				[] => true,
				['c', '\u{1}', s @ ..] => {
					super::push(out, 17113);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ptabundus.
pub(crate) fn n22162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13230);
	match s {
		[] => true,
		['e', s @ ..] => {
			super::push(out, 15625);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -rtabundus.
pub(crate) fn n22163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13231);
	s.is_empty()
}

/// Words ending -stabundus.
pub(crate) fn n22164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13232);
	s.is_empty()
}

/// Words ending -utabundus.
pub(crate) fn n22165(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13233);
	s.is_empty()
}

/// Words ending -uabundus.
pub(crate) fn n14925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9788);
	match s {
		[] => true,
		['t', 'c', 'u', s @ ..] => {
			super::push(out, 13234);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -xabundus.
pub(crate) fn n14926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9789);
	match s {
		[] => true,
		['i', s @ ..] => {
			match s {
				[] => true,
				['l', s @ ..] => {
					super::push(out, 15626);
					s.is_empty()
				}
				['n', s @ ..] => {
					super::push(out, 15627);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

