use flexion::Path;

/// Words ending -δια.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 145);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2360);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4579);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6219);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 781);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2361);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4580);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2362);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2363);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4582);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6220);
									s.is_empty()
								}
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

/// Words ending -εια.
pub(crate) fn n148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 146);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 784);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2365);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2366);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4585);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 787);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2370);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4586);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4587);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2371);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4588);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6222);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7013);
											s.is_empty()
										}
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
		['κ', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2373);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4589);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2374);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4590);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2375);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2376);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 790);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2377);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4592);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4593);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6225);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7014);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7301);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 7015);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7302);
													s.is_empty()
												}
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
				['υ', s @ ..] => {
					super::push(out, 2378);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 791);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['α', 'τ', s @ ..] => {
					super::push(out, 2379);
					match s {
						[] => true,
						['σ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4594);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2380);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2381);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4596);
							match s {
								[] => true,
								['γ', 'υ', 'ξ', '\u{1}', s @ ..] => {
									super::push(out, 6226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2383);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2384);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 794);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2385);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4598);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 4599);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6227);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7016);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 4600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 2386);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4601);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6228);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7017);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2387);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2388);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2389);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4604);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6229);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7018);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2390);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4605);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4606);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'κ', s @ ..] => {
							super::push(out, 4607);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', 'π', s @ ..] => {
					super::push(out, 2391);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4608);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2392);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2393);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2394);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 796);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2395);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2396);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θια.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 797);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2397);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιια.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 148);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['π', 'ο', 'σ', s @ ..] => {
					super::push(out, 2398);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4611);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 6231);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7019);
											s.is_empty()
										}
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

/// Words ending -κια.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 800);
			match s {
				[] => true,
				['κ', 'α', s @ ..] => {
					super::push(out, 2399);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 4612);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2401);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4613);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6233);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7020);
											s.is_empty()
										}
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
		['ι', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2402);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4614);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4615);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6234);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7021);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2405);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4616);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6235);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4617);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λια.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 805);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2407);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 4618);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6237);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7022);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 806);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2409);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 2410);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4619);
							match s {
								[] => true,
								['κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6238);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 4620);
							match s {
								[] => true,
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 6239);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 2411);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4621);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6240);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7023);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 808);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2413);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4623);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2414);
					match s {
						[] => true,
						['κ', 'γ', s @ ..] => {
							super::push(out, 4624);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6241);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 809);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2415);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4625);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 811);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 812);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2416);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4626);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6242);
									s.is_empty()
								}
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

/// Words ending -μια.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['γ', 'ο', s @ ..] => {
					super::push(out, 2417);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4627);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', 'δ', 'ν', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2419);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4628);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', 'υ', 'ρ', 'υ', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'α', s @ ..] => {
			super::push(out, 816);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2421);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4629);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 817);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2422);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4630);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 818);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2423);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4631);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6244);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7024);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 7303);
													match s {
														[] => true,
														['α', '\u{1}', s @ ..] => super::t2::n7422(s, out),
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
						['ο', 'ρ', 'τ', s @ ..] => {
							super::push(out, 4632);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6245);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7025);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2424);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4633);
							match s {
								[] => true,
								['ο', 'π', s @ ..] => {
									super::push(out, 6246);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7026);
											s.is_empty()
										}
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
		['υ', s @ ..] => {
			super::push(out, 819);
			s.is_empty()
		}
		['ω', 'ν', s @ ..] => {
			super::push(out, 820);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2425);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4634);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νια.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 821);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2426);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2427);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4635);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6247);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 822);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 2428);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4636);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['α', 'κ', s @ ..] => {
					super::push(out, 2429);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4637);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6248);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4638);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['γ', 'ο', 'μ', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4639);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2432);
					match s {
						[] => true,
						['η', 'δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4640);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4641);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4642);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6250);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7027);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7304);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 4643);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6251);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 7028);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4644);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4645);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6252);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7029);
											s.is_empty()
										}
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
		['τ', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 828);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξια.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['α', 'ρ', 'α', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2434);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4647);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', 'δ', 'α', s @ ..] => {
			super::push(out, 830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οια.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 831);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2435);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4648);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 833);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πια.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2436);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4649);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['ρ', 'θ', 'ν', 'α', 'σ', s @ ..] => {
					super::push(out, 2437);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4650);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6253);
									s.is_empty()
								}
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

/// Words ending -ρια.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 836);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2438);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4651);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6254);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 837);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2439);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2441);
					match s {
						[] => true,
						['α', 'ν', s @ ..] => {
							super::push(out, 4652);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6255);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 839);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2442);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4653);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2443);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4654);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6256);
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 7030);
											s.is_empty()
										}
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
		['ι', s @ ..] => {
			super::push(out, 841);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2444);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 843);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2445);
					match s {
						[] => true,
						['η', 'σ', 'ι', s @ ..] => {
							super::push(out, 4655);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 844);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2446);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 845);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2447);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4657);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2448);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4658);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2449);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 4659);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2450);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2451);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4660);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2452);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4661);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6257);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7031);
											s.is_empty()
										}
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
		['ω', s @ ..] => {
			super::push(out, 847);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σια.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2453);
					match s {
						[] => true,
						['ι', 'α', s @ ..] => {
							super::push(out, 4662);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2454);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4663);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6258);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4664);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6259);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 4665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2455);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4666);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['κ', 'ι', 'ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2456);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2457);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4667);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6260);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', 'ι', s @ ..] => {
					super::push(out, 2458);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['ν', 'γ', s @ ..] => {
					super::push(out, 2459);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4669);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6261);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', 'α', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2460);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 4670);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2461);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4671);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6262);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 7032);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', 'π', s @ ..] => {
					super::push(out, 2462);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4672);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', 'ῥ', s @ ..] => {
			super::push(out, 853);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τια.
pub(crate) fn n160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2463);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4673);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6263);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4674);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 4675);
							match s {
								[] => true,
								['ο', 'τ', s @ ..] => {
									super::push(out, 6265);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7033);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7305);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t2::n7423(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 7034);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7306);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 4676);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6266);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2464);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4677);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6267);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 855);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2465);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4678);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6268);
									match s {
										[] => true,
										['α', '\u{1}', s @ ..] => {
											super::push(out, 7035);
											s.is_empty()
										}
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
		['σ', s @ ..] => {
			super::push(out, 856);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2466);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4679);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6269);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2468);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4680);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6270);
									s.is_empty()
								}
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

/// Words ending -υια.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 860);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2473);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4681);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6271);
									s.is_empty()
								}
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

/// Words ending -φια.
pub(crate) fn n162(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 160);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['ρ', 'γ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4682);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6272);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7036);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', 'ρ', s @ ..] => {
							super::push(out, 4683);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 6273);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 7037);
											s.is_empty()
										}
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
		['ο', 'ρ', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['τ', 'α', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χια.
pub(crate) fn n163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 161);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 865);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2475);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4684);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6274);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 867);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψια.
pub(crate) fn n164(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 162);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2476);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4685);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6275);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 869);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κατοικημα.
pub(crate) fn n7349(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7307);
	s.is_empty()
}

/// Words ending -μμα.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 917);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2533);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 919);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2534);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2535);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4715);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6284);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2536);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2537);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4716);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6285);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 2538);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', 'κ', 'ι', s @ ..] => {
			super::push(out, 921);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 2539);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4717);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2540);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4718);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2541);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4719);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρμα.
pub(crate) fn n187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['α', 'θ', s @ ..] => {
			super::push(out, 926);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 927);
			s.is_empty()
		}
		['υ', 'θ', s @ ..] => {
			super::push(out, 928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σμα.
pub(crate) fn n188(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2544);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4721);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 4722);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 4723);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'β', s @ ..] => {
					super::push(out, 2545);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2546);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 2547);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'δ', 'β', s @ ..] => {
			super::push(out, 930);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2548);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4724);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 932);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2549);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4725);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4726);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2550);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2551);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4727);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7042);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 4728);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 4729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'κ', s @ ..] => {
					super::push(out, 2552);
					s.is_empty()
				}
				['χ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4730);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4731);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2553);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 4732);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 2554);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2555);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4733);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2556);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2557);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4734);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6288);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'π', s @ ..] => {
					super::push(out, 2558);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμα.
pub(crate) fn n189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 187);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['κ', 'γ', s @ ..] => {
					super::push(out, 2559);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4735);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2560);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 935);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2561);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4736);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 936);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2562);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2563);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2564);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4737);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2565);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 2566);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 937);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 938);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2568);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 939);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2569);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2570);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4738);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6289);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2571);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 940);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 941);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2572);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 942);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 943);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 944);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2573);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2574);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2576);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χμα.
pub(crate) fn n190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 188);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 947);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 948);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2577);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4740);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωμα.
pub(crate) fn n191(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 189);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 949);
			s.is_empty()
		}
		['ν', 'γ', s @ ..] => {
			super::push(out, 950);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2578);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2579);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 951);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρεμαθρα.
pub(crate) fn n6338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6300);
	s.is_empty()
}

/// Words ending -ηθρα.
pub(crate) fn n1021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1014);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 2631);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4771);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6301);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2632);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιθρα.
pub(crate) fn n1022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1015);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2633);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4772);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρθρα.
pub(crate) fn n1023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1016);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2634);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4773);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6303);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χθρα.
pub(crate) fn n1024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1017);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2635);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιρα.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2636);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2637);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4775);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2638);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4776);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1019);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 2639);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2640);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2641);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2642);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2643);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κρα.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	s.is_empty()
}

/// Words ending -ορα.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1021);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1022);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2644);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4778);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πρα.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2645);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4779);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρρα.
pub(crate) fn n223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4780);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1027);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τρα.
pub(crate) fn n224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2647);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2648);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 4781);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6304);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2649);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2650);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4783);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2651);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4784);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2652);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4785);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6305);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2653);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 4786);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2654);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4787);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6307);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7045);
											s.is_empty()
										}
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
		['σ', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2655);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4788);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6308);
									match s {
										[] => true,
										['ε', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 7046);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2656);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4789);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2657);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 4790);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6310);
									match s {
										[] => true,
										['ο', '\u{1}', s @ ..] => {
											super::push(out, 7047);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2658);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4791);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6311);
									match s {
										[] => true,
										['α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 7048);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 2659);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2660);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4793);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6312);
									s.is_empty()
								}
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

/// Words ending -υρα.
pub(crate) fn n225(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 223);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2661);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2662);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2663);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4794);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6313);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1037);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1038);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2664);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4795);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1040);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2665);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2666);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4796);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6314);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2669);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 4797);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6315);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7049);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2670);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4798);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2671);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2672);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 4799);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1044);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2673);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1045);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2674);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2675);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2676);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4802);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φρα.
pub(crate) fn n226(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 224);
	s.is_empty()
}

/// Words ending -χρα.
pub(crate) fn n227(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 225);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 1046);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωρα.
pub(crate) fn n228(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 226);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1047);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2678);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word περισπωμενη.
pub(crate) fn n7350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7308);
	s.is_empty()
}

/// Words ending -ρυνη.
pub(crate) fn n1222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1213);
	match s {
		[] => true,
		['ο', 'τ', s @ ..] => {
			super::push(out, 2781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2782);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4844);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -συνη.
pub(crate) fn n1223(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1214);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2783);
			match s {
				[] => true,
				['μ', 'γ', s @ ..] => {
					super::push(out, 4845);
					match s {
						[] => true,
						['α', 'ρ', 'π', 'α', s @ ..] => {
							super::push(out, 6330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2784);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4846);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6331);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 7053);
									s.is_empty()
								}
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

/// Words ending -χυνη.
pub(crate) fn n1224(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1215);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 2785);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4847);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 6332);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χνη.
pub(crate) fn n329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 326);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1217);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωνη.
pub(crate) fn n330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 327);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1218);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1219);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1220);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2786);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4848);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6333);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1222);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word βυσναιοι.
pub(crate) fn n7351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7309);
	s.is_empty()
}

/// Words ending -ροι.
pub(crate) fn n409(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2850);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2851);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2852);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4891);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1326);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2853);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2854);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζαντοι.
pub(crate) fn n1339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1330);
	s.is_empty()
}

/// Words ending -χοι.
pub(crate) fn n412(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 407);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1331);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1332);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2856);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2857);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2858);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιον.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 449);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1379);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2888);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4908);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6349);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2889);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2890);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2891);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2892);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4909);
							match s {
								[] => true,
								['δ', 'ι', s @ ..] => {
									super::push(out, 6350);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7061);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7310);
													s.is_empty()
												}
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
				['σ', s @ ..] => {
					super::push(out, 2893);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4910);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2894);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4911);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6352);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 2895);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4912);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1381);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2896);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2897);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2898);
					match s {
						[] => true,
						['λ', 'ο', 'ν', s @ ..] => {
							super::push(out, 4913);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6354);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7062);
											s.is_empty()
										}
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
		['δ', s @ ..] => {
			super::push(out, 1382);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2899);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2900);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2901);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4915);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6355);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 4916);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6356);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7063);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7311);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', 'λ', s @ ..] => {
							super::push(out, 4917);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2902);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2903);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4918);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6358);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7064);
											s.is_empty()
										}
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
		['ε', s @ ..] => {
			super::push(out, 1383);
			match s {
				[] => true,
				['γ', 'ο', s @ ..] => {
					super::push(out, 2904);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4919);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', 'α', 'ι', s @ ..] => {
					super::push(out, 2905);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4920);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6359);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', 'υ', s @ ..] => {
					super::push(out, 2906);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4921);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6360);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', 'ι', s @ ..] => {
					super::push(out, 2907);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4922);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6361);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2908);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2909);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4923);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6362);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 4924);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6363);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7065);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 7312);
													s.is_empty()
												}
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
				['π', s @ ..] => {
					super::push(out, 2910);
					s.is_empty()
				}
				['ρ', 'υ', 'α', s @ ..] => {
					super::push(out, 2911);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 4925);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6364);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', 'ι', s @ ..] => {
					super::push(out, 2912);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2913);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2914);
					s.is_empty()
				}
				['χ', 'α', s @ ..] => {
					super::push(out, 2915);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4926);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6365);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 1384);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2916);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4927);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6366);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7066);
											s.is_empty()
										}
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
		['η', s @ ..] => {
			super::push(out, 1385);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2917);
					match s {
						[] => true,
						['ν', s @ ..] => super::t2::n4959(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => super::t2::n1395(s, out),
		['κ', s @ ..] => super::t2::n1396(s, out),
		['λ', s @ ..] => super::t2::n1397(s, out),
		['μ', s @ ..] => {
			super::push(out, 1389);
			s.is_empty()
		}
		['ν', s @ ..] => super::t2::n1399(s, out),
		['ξ', s @ ..] => super::t2::n1400(s, out),
		['ο', s @ ..] => super::t2::n1401(s, out),
		['π', s @ ..] => super::t2::n1402(s, out),
		['ρ', s @ ..] => super::t2::n1403(s, out),
		['σ', s @ ..] => super::t2::n1404(s, out),
		['τ', s @ ..] => super::t2::n1405(s, out),
		['φ', s @ ..] => super::t2::n1406(s, out),
		['χ', s @ ..] => super::t2::n1407(s, out),
		['ῥ', s @ ..] => {
			super::push(out, 1399);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λλον.
pub(crate) fn n1419(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1410);
	match s {
		[] => true,
		['α', 'β', s @ ..] => {
			super::push(out, 2980);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4983);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'φ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4984);
					match s {
						[] => true,
						['τ', 'ν', 'α', s @ ..] => {
							super::push(out, 6400);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 7086);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7319);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4985);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τλον.
pub(crate) fn n1422(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1413);
	match s {
		[] => true,
		['υ', 'ε', s @ ..] => {
			super::push(out, 2983);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 4986);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υλον.
pub(crate) fn n1423(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1414);
	match s {
		[] => true,
		['ξ', s @ ..] => {
			super::push(out, 2984);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2985);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2986);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωλον.
pub(crate) fn n1424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1415);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2987);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μον.
pub(crate) fn n457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1416);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2988);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2989);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4988);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6401);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1417);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2990);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4989);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6402);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7087);
											s.is_empty()
										}
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
		['υ', s @ ..] => {
			super::push(out, 1418);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νον.
pub(crate) fn n458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 453);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1419);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1420);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2993);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4990);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2994);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4991);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1421);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2995);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1422);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1423);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2998);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4992);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6403);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1424);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2999);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4993);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4994);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3000);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3001);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4995);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3002);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3003);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4996);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6404);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7088);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1425);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3005);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3006);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1426);
			match s {
				[] => true,
				['ε', 'ρ', s @ ..] => {
					super::push(out, 3007);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 4997);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ρ', s @ ..] => {
					super::push(out, 3008);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4998);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3009);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4999);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6405);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7089);
											s.is_empty()
										}
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
		['ο', s @ ..] => {
			super::push(out, 1427);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3010);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3011);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5000);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6406);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1428);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3012);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5001);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6407);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7090);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7320);
													s.is_empty()
												}
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
				['υ', s @ ..] => {
					super::push(out, 3013);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1429);
			match s {
				[] => true,
				['α', 'κ', s @ ..] => {
					super::push(out, 3014);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3015);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3016);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1430);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3017);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 3018);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5002);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6408);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3019);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1431);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξον.
pub(crate) fn n459(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1432);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3020);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πον.
pub(crate) fn n461(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 456);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1433);
			match s {
				[] => true,
				['λ', 'β', 'ω', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3021);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρον.
pub(crate) fn n462(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 457);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1434);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3022);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3023);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3024);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3025);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5003);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6409);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3026);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3028);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 5004);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ε', 'δ', 'ο', s @ ..] => {
					super::push(out, 3029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1436);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3030);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1437);
			match s {
				[] => true,
				['χ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3032);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1438);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3033);
					s.is_empty()
				}
				['ε', 'λ', 'π', s @ ..] => {
					super::push(out, 3034);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 3035);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3036);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5005);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6410);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7091);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ῥ', s @ ..] => {
									super::push(out, 6411);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1439);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3038);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5006);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5007);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6412);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3039);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5008);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1440);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1441);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3041);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3042);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1442);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3043);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1443);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5009);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5010);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3044);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 3045);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5011);
							s.is_empty()
						}
						['ν', 'ι', s @ ..] => {
							super::push(out, 5012);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'ν', s @ ..] => {
					super::push(out, 3046);
					s.is_empty()
				}
				['κ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5013);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6413);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 6414);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7092);
											s.is_empty()
										}
										_ => false,
									}
								}
								['φ', s @ ..] => {
									super::push(out, 6415);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 5014);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6416);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7093);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3047);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5015);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 5016);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5017);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6417);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7094);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7321);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', 'ῥ', s @ ..] => {
							super::push(out, 5018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3048);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5019);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3049);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5020);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6418);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 6419);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 5021);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 5022);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6420);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6421);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7095);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 6422);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', 'μ', s @ ..] => {
							super::push(out, 5023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3050);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5024);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6423);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7096);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7322);
													s.is_empty()
												}
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
				['ω', s @ ..] => {
					super::push(out, 3051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t2::n1453(s, out),
		['ω', s @ ..] => {
			super::push(out, 1445);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3060);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3061);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σον.
pub(crate) fn n463(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 458);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1446);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1447);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3062);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1448);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3063);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5032);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5033);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6426);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1449);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3064);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5034);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1450);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3065);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5035);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3066);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5036);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1451);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3067);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τον.
pub(crate) fn n464(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 459);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1452);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3068);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5037);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6428);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7098);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7323);
													s.is_empty()
												}
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
				['ι', s @ ..] => {
					super::push(out, 3069);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3070);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5038);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3071);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1453);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3072);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5039);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', 'ρ', s @ ..] => {
					super::push(out, 3073);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5040);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1454);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3074);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3075);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1455);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3076);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5042);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1456);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3077);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1457);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1458);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3079);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1459);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3080);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5044);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1460);
			match s {
				[] => true,
				['α', 'π', s @ ..] => {
					super::push(out, 3081);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1461);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['π', 'ρ', s @ ..] => {
							super::push(out, 5046);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6431);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3083);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5047);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5048);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6433);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7099);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 5049);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5050);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6434);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1462);
			match s {
				[] => true,
				['ρ', 'β', '\u{1}', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1463);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3086);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5051);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6435);
									s.is_empty()
								}
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

/// Words ending -υον.
pub(crate) fn n465(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 460);
	match s {
		[] => true,
		['ρ', 'θ', s @ ..] => {
			super::push(out, 1464);
			s.is_empty()
		}
		['τ', 'π', s @ ..] => {
			super::push(out, 1465);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φον.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 461);
	s.is_empty()
}

/// Words ending -χον.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 462);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1466);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1467);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3089);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5053);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1468);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3090);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 5054);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6436);
									s.is_empty()
								}
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

/// Words ending -ψον.
pub(crate) fn n468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1469);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωον.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 464);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1470);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αμαζων.
pub(crate) fn n5105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5073);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 6443);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιζων.
pub(crate) fn n1506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1497);
	match s {
		[] => true,
		['ε', 'μ', '\u{1}', s @ ..] => {
			super::push(out, 3119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θων.
pub(crate) fn n478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1498);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1499);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1500);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3122);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5074);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιων.
pub(crate) fn n479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1502);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1503);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3125);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1504);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1505);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3128);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5075);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1506);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1507);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3129);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5076);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3130);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1508);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3132);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5078);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1509);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3133);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1510);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3134);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3135);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3136);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 5080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1511);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3137);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1512);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3138);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5082);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1513);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3139);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5083);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6445);
									s.is_empty()
								}
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

/// Words ending -κων.
pub(crate) fn n480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 475);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1514);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3140);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3141);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3142);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1515);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1516);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3144);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5085);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λων.
pub(crate) fn n481(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 476);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1517);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1518);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3146);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1520);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3147);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1521);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3148);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μων.
pub(crate) fn n482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 477);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1522);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3149);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1523);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3150);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1524);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3151);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 5088);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3152);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5089);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6446);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3153);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5090);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1525);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1526);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3155);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5091);
							match s {
								[] => true,
								['ο', 'κ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6448);
									s.is_empty()
								}
								['υ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 6449);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1527);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3157);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1528);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3158);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1529);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3159);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3160);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5093);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6450);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1530);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3161);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νων.
pub(crate) fn n483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 478);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1531);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3162);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1532);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3163);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1533);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3165);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξων.
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 479);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1534);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1535);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πων.
pub(crate) fn n485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 480);
	match s {
		[] => true,
		['α', s @ ..] => {
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1536);
			match s {
				[] => true,
				['π', 'ο', 'λ', 'η', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1537);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρων.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 481);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1538);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1539);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1540);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1541);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1542);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3175);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σων.
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 482);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1544);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3176);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1545);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1546);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -των.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 483);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1547);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3177);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5096);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6451);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7102);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 6452);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7103);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3178);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1548);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3179);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3180);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5098);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1549);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1550);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5099);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1551);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3183);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5100);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υων.
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 484);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1552);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 3184);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1553);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3185);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5101);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1554);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3186);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5102);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6455);
									s.is_empty()
								}
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

/// Words ending -φων.
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1555);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1556);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1557);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1558);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3190);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χων.
pub(crate) fn n491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1559);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1560);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3193);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3194);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1561);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3195);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5104);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1562);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3196);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word συροπερδιξ.
pub(crate) fn n7366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7324);
	s.is_empty()
}

/// The word βιβλιοπωλησ.
pub(crate) fn n7368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7326);
	s.is_empty()
}

/// Words ending -ουμνησ.
pub(crate) fn n5283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5250);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 6504);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρνησ.
pub(crate) fn n1830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1818);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3438);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υνησ.
pub(crate) fn n1831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1819);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 3439);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5251);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5252);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3440);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5253);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3441);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 5254);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χνησ.
pub(crate) fn n1832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1820);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3442);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 5255);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωνησ.
pub(crate) fn n1833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1821);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 3443);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5256);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οησ.
pub(crate) fn n609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3444);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πησ.
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1823);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3445);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5257);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1824);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3447);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5258);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6508);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1825);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρησ.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 606);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1826);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3449);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5259);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6509);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7116);
											s.is_empty()
										}
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
		['κ', s @ ..] => {
			super::push(out, 1827);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3450);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5260);
							match s {
								[] => true,
								['ο', 'λ', s @ ..] => {
									super::push(out, 6510);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 7117);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3451);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5261);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6511);
									match s {
										[] => true,
										['ρ', 'υ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 7118);
											s.is_empty()
										}
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

/// Words ending -σησ.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 607);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1829);
			s.is_empty()
		}
		['ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1830);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1831);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3452);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5262);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τησ.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3453);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5263);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3454);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3455);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5264);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 6512);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3456);
					s.is_empty()
				}
				['ν', 'η', s @ ..] => {
					super::push(out, 3457);
					match s {
						[] => true,
						['δ', 'ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 5266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3458);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5267);
							match s {
								[] => true,
								['ο', 'π', s @ ..] => {
									super::push(out, 6513);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7119);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7327);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n7426(s, out),
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
				['σ', s @ ..] => {
					super::push(out, 3459);
					match s {
						[] => true,
						['ι', 'π', '\u{1}', s @ ..] => {
							super::push(out, 5268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3460);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5269);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3461);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 5270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1833);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3462);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3463);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5271);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6514);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 7120);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3464);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5272);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6515);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1834);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3465);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 3466);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3467);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5274);
							match s {
								[] => true,
								['ο', 'τ', s @ ..] => {
									super::push(out, 6516);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7121);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 7328);
													s.is_empty()
												}
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
				['λ', s @ ..] => {
					super::push(out, 3468);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3469);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5276);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6517);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7122);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 7329);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 5277);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6518);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3470);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5278);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3471);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3472);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3473);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1835);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3474);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5281);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6519);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3475);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5282);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5283);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3476);
					match s {
						[] => true,
						['ο', 'π', 'υ', s @ ..] => {
							super::push(out, 5284);
							match s {
								[] => true,
								['ο', 'ν', 'α', s @ ..] => {
									super::push(out, 6520);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7123);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7330);
													match s {
														[] => true,
														['δ', 'α', '\u{1}', s @ ..] => super::t2::n7427(s, out),
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
				['μ', s @ ..] => {
					super::push(out, 3477);
					s.is_empty()
				}
				['ν', s @ ..] => super::t2::n3505(s, out),
				['ο', s @ ..] => {
					super::push(out, 3479);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5288);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t2::n3507(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t2::n1848(s, out),
		['λ', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3487);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => super::t2::n1850(s, out),
		['ο', s @ ..] => super::t2::n1851(s, out),
		['π', s @ ..] => super::t2::n1852(s, out),
		['ρ', s @ ..] => super::t2::n1853(s, out),
		['σ', s @ ..] => super::t2::n1854(s, out),
		['υ', s @ ..] => super::t2::n1855(s, out),
		['ω', s @ ..] => super::t2::n1856(s, out),
		_ => false,
	}
}

/// Words ending -υησ.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1845);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χησ.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1846);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3521);
					match s {
						[] => true,
						['γ', 'ι', 'λ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 5337);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5338);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6558);
									s.is_empty()
								}
								['σ', s @ ..] => {
									match s {
										[] => true,
										['α', 'τ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 7142);
											s.is_empty()
										}
										['ε', 'ρ', 'ι', 'α', '\u{1}', s @ ..] => {
											super::push(out, 7143);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5339);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5340);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6559);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7144);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 7341);
													match s {
														[] => true,
														['λ', '\u{1}', s @ ..] => super::t2::n7430(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['θ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 6560);
									s.is_empty()
								}
								['ο', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 6561);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'ε', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 5341);
							s.is_empty()
						}
						['τ', 'ο', 'μ', 'ω', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 5342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αρακισ.
pub(crate) fn n5390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5356);
	s.is_empty()
}

/// Words ending -ηκισ.
pub(crate) fn n1887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1875);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3547);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5357);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ικισ.
pub(crate) fn n1888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1876);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 3548);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οκισ.
pub(crate) fn n1889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1877);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3549);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5358);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6570);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρκισ.
pub(crate) fn n1890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1878);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3550);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 5359);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λισ.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 621);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1879);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3551);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5360);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6572);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3552);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5361);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6573);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 7146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3553);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5362);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1880);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3555);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', 'σ', 'ε', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3556);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1882);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3557);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5363);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6574);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3558);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3559);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3560);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3561);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5365);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1883);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3562);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3563);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1884);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3564);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5368);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6575);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3565);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3566);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3567);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μισ.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1886);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3568);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5371);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6576);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3569);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3570);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5372);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6577);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1887);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3571);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1888);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3572);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5374);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6578);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1889);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1890);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3574);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5375);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6579);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1891);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1892);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3575);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5376);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 5377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νισ.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1894);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3576);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3577);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5379);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3578);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5380);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6580);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7147);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3579);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5381);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 6581);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1895);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3581);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5382);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1897);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3582);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5383);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1899);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5384);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3585);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5385);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξισ.
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1901);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3586);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5386);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3587);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1902);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3588);
					s.is_empty()
				}
				['λ', 'π', s @ ..] => {
					super::push(out, 3589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1903);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1904);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3590);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5387);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3591);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3592);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3593);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5388);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1905);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3594);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 5389);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5390);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6583);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7148);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3596);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3597);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3598);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5391);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 3599);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οισ.
pub(crate) fn n630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1907);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1908);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3600);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1910);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3602);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5392);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3603);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3604);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5393);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6584);
									s.is_empty()
								}
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

/// Words ending -πισ.
pub(crate) fn n631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1912);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3605);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5394);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1913);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3606);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5395);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1914);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3607);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5396);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6585);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1915);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3608);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3609);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5397);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1917);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3611);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5399);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5400);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6586);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3612);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5401);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6587);
									s.is_empty()
								}
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

/// Words ending -ρισ.
pub(crate) fn n632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1918);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3613);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5402);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 6588);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7149);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 6589);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 6590);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7150);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3614);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5403);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6591);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 3615);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5404);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6592);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3616);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5405);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5406);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6593);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 7151);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3618);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3619);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5407);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6594);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3620);
					match s {
						[] => true,
						['α', 'ζ', '\u{1}', s @ ..] => {
							super::push(out, 5408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1919);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3621);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1920);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3622);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1921);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3623);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5410);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6595);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3624);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1922);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3625);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5412);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6596);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 7152);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3626);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3627);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5413);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6597);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3628);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5414);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6598);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3629);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1923);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3630);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3631);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3632);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5416);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6599);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7153);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3633);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5417);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6600);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1924);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3634);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5418);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6601);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7154);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5419);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3635);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3636);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1926);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3637);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1927);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3638);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5420);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1928);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3639);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5421);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1929);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3640);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1930);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3641);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 5423);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6602);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3642);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5424);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6603);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7155);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3643);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5425);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t2::n3671(s, out),
				['π', s @ ..] => super::t2::n3672(s, out),
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1931);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σισ.
pub(crate) fn n633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 628);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1932);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3646);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 3647);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5428);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6606);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7158);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3648);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5429);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6607);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7159);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 7342);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', 'τ', s @ ..] => {
							super::push(out, 5430);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6608);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7160);
											match s {
												[] => true,
												['α', '\u{1}', s @ ..] => {
													super::push(out, 7343);
													s.is_empty()
												}
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
				['λ', s @ ..] => {
					super::push(out, 3649);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5431);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3650);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3651);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5433);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5434);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 5435);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3653);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1933);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 3654);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3655);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3656);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3657);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 3658);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1934);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3660);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3661);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5437);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6609);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7161);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3662);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5438);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6610);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7162);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', 'ν', s @ ..] => {
									super::push(out, 6611);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 7163);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', 'τ', s @ ..] => {
					super::push(out, 3663);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3664);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3665);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 5439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3666);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5440);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5441);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 3667);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1935);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3668);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5442);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3669);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 5443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3670);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5444);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6612);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 7164);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7344);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 5445);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 5446);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3671);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5447);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'σ', s @ ..] => {
					super::push(out, 3672);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1936);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3673);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 5448);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6613);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1937);
			match s {
				[] => true,
				['α', 'π', s @ ..] => {
					super::push(out, 3674);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5449);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 6614);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1938);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1939);
			match s {
				[] => true,
				['ε', 'π', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1940);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 3677);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5450);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6615);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t2::n1953(s, out),
		['ω', s @ ..] => super::t2::n1954(s, out),
		_ => false,
	}
}

/// Words ending -τισ.
pub(crate) fn n634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 629);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1943);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3696);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5472);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6623);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3697);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5473);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3698);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5474);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6624);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1944);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3699);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3701);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5477);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6625);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1945);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 5478);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6626);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7168);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3703);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3704);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3705);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5479);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6627);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5480);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6628);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7169);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3706);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5481);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6629);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7170);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 7345);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t2::n7431(s, out),
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
				['τ', s @ ..] => {
					super::push(out, 3707);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1946);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3708);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5482);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3709);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3710);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1947);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3711);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5484);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6630);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7171);
											match s {
												[] => true,
												['ω', s @ ..] => {
													super::push(out, 7346);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', 'κ', 'ε', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 7172);
											s.is_empty()
										}
										['υ', 'θ', 'χ', 'ι', '\u{1}', s @ ..] => {
											super::push(out, 7173);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3712);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5485);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1948);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3713);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3714);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5487);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1949);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3715);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5488);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6631);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7174);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5489);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3716);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5490);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6633);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7175);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 6634);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5491);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6635);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3717);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5492);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6636);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 5493);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5494);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3718);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5495);
							s.is_empty()
						}
						['μ', s @ ..] => super::t2::n5530(s, out),
						['ξ', s @ ..] => super::t2::n5531(s, out),
						['π', s @ ..] => super::t2::n5532(s, out),
						_ => false,
					}
				}
				['ω', s @ ..] => super::t2::n3746(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t2::n1962(s, out),
		['υ', s @ ..] => super::t2::n1963(s, out),
		['ω', s @ ..] => super::t2::n1964(s, out),
		_ => false,
	}
}

/// Words ending -φισ.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 630);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1953);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3730);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3731);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5503);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6642);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1954);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1955);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1956);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χισ.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 631);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1957);
			match s {
				[] => true,
				['ῥ', s @ ..] => {
					super::push(out, 3733);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1958);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1959);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1960);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1961);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3734);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5504);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1962);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3735);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1963);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1964);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3736);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5505);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψισ.
pub(crate) fn n637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 632);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1965);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3737);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 3738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1966);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1967);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3739);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5506);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3740);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3741);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3742);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1968);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3743);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3744);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5508);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3745);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3746);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5509);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 3747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1969);
			s.is_empty()
		}
		['μ', 'α', 'κ', s @ ..] => {
			super::push(out, 1970);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1971);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1972);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1973);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3750);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5510);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3751);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5511);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωισ.
pub(crate) fn n638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 633);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1974);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3752);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 5512);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6643);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3753);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ϝισ.
pub(crate) fn n639(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1975);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ῥισ.
pub(crate) fn n640(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 635);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1976);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γοσ.
pub(crate) fn n645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 640);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1994);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3776);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3777);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5530);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6651);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 5531);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6652);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3778);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3779);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5532);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6653);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3780);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3781);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3782);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3783);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5535);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3784);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5536);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6654);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1995);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3785);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5537);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3786);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 5538);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3787);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5539);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6656);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3788);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5540);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6657);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 5541);
							match s {
								[] => true,
								['σ', 'ο', s @ ..] => {
									super::push(out, 6658);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 7178);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3789);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5542);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6659);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5543);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6660);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7179);
											s.is_empty()
										}
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
		['ε', s @ ..] => {
			super::push(out, 1996);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3790);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1997);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3792);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5545);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6661);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3793);
					match s {
						[] => true,
						['α', 'ρ', 'τ', 'σ', 'ο', s @ ..] => {
							super::push(out, 5546);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6662);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7180);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3794);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3795);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5547);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6663);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 3796);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1998);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3797);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5548);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6664);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3798);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1999);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3799);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3800);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5550);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2000);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3801);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5551);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5552);
							match s {
								[] => true,
								['μ', 'υ', s @ ..] => {
									super::push(out, 6665);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 6666);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 7181);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7349);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', 'α', s @ ..] => {
									super::push(out, 6667);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6668);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2001);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3802);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5553);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6669);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 5554);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6670);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3803);
					match s {
						[] => true,
						['μ', s @ ..] => super::t2::n5590(s, out),
						['τ', s @ ..] => super::t2::n5591(s, out),
						_ => false,
					}
				}
				['υ', s @ ..] => super::t2::n3831(s, out),
				_ => false,
			}
		}
		['σ', s @ ..] => super::t2::n2014(s, out),
		['υ', s @ ..] => super::t2::n2015(s, out),
		['ω', s @ ..] => super::t2::n2016(s, out),
		_ => false,
	}
}

/// Words ending -δοσ.
pub(crate) fn n646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 641);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2005);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3808);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3809);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3810);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5561);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3811);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3812);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5562);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6673);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 5563);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3813);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3814);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3815);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5566);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 5567);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6674);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7184);
											s.is_empty()
										}
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
		['β', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3816);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 5568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3817);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5569);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5570);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 6675);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3818);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5571);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6676);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2007);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3819);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5572);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6677);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5573);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6678);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3820);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5574);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3821);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3822);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3823);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2009);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3824);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3825);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3826);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3827);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3828);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5579);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6679);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7186);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3829);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5580);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6680);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3830);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2011);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3831);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3832);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5582);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3833);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3834);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2012);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3835);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3836);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3837);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3838);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3839);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3840);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5584);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3841);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5585);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5586);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5587);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6682);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3842);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5588);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6683);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 5589);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6684);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3843);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 5590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3844);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5591);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3845);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3846);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3847);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5592);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6685);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2015);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θοσ.
pub(crate) fn n649(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 644);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3857);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3858);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5596);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6686);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3859);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 5597);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3860);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5598);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6687);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 6688);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 5599);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6689);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7188);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3861);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5600);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5601);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5602);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6690);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3862);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3863);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5604);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 5605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3864);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5606);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6691);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3865);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3866);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3867);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5608);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6692);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3868);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5609);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 3869);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2028);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3870);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3871);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3872);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5611);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3873);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3874);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3875);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5612);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6693);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3876);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3877);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3878);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5614);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2030);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5615);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5616);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 5617);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6695);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3881);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6696);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 5619);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6697);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5620);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6698);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3882);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5621);
							match s {
								[] => true,
								['ε', 'ρ', 'ε', 'τ', s @ ..] => {
									super::push(out, 6699);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5622);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6700);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 5623);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6701);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6702);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 6703);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5624);
							match s {
								[] => true,
								['η', 'μ', s @ ..] => {
									super::push(out, 6704);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3883);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5626);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5627);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6705);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3884);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2032);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2033);
			s.is_empty()
		}
		['σ', s @ ..] => super::t2::n2046(s, out),
		['τ', s @ ..] => super::t2::n2047(s, out),
		['υ', s @ ..] => super::t2::n2048(s, out),
		['χ', s @ ..] => super::t2::n2049(s, out),
		_ => false,
	}
}

/// Words ending -ιοσ.
pub(crate) fn n650(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 645);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2038);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3900);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3901);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3902);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3903);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5642);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6712);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7190);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3904);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5643);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6713);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 7191);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3906);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 2039);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3907);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5645);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6714);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3908);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 2040);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3909);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 3910);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5647);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2041);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3911);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3912);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5648);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6716);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3913);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2043);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3914);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5649);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3915);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3916);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3917);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5651);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 5652);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5653);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6717);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7192);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3918);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2044);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3919);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5654);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6718);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 7193);
											s.is_empty()
										}
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
		['ο', s @ ..] => {
			super::push(out, 2045);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3920);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2046);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3922);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3923);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5655);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3924);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5656);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2047);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3925);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5657);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6720);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7194);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 3926);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5658);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6721);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3927);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3928);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3929);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5659);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5660);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3930);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5661);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2048);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2049);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3931);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2050);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3932);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3933);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5662);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6722);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3934);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5663);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5664);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κοσ.
pub(crate) fn n651(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 646);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2053);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3936);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3937);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3938);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5665);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3939);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5666);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 6723);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5667);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6724);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3940);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5668);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3941);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5669);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 5670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3942);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3943);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5671);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6726);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7195);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6727);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 6728);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3944);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5673);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 3945);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2054);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3946);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3947);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 5675);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6729);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', 'ρ', s @ ..] => {
					super::push(out, 3948);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3949);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5677);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6730);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2055);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5678);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2056);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3951);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5679);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6731);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7196);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3952);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5680);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6732);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3953);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3954);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5681);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2057);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3956);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 3957);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3958);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5682);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6733);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7197);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3959);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5683);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5684);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5685);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6734);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 6735);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7198);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7352);
													s.is_empty()
												}
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
				['ν', s @ ..] => {
					super::push(out, 3960);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3961);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5686);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5687);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3962);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5688);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6736);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3963);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5689);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2058);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3964);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3965);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5690);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6737);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2059);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3966);
					match s {
						[] => true,
						['χ', s @ ..] => super::t2::n5727(s, out),
						_ => false,
					}
				}
				['ε', s @ ..] => super::t2::n3994(s, out),
				['ο', s @ ..] => {
					super::push(out, 3968);
					s.is_empty()
				}
				['υ', s @ ..] => super::t2::n3996(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t2::n2072(s, out),
		['ρ', s @ ..] => super::t2::n2073(s, out),
		['σ', s @ ..] => super::t2::n2074(s, out),
		['υ', s @ ..] => super::t2::n2075(s, out),
		_ => false,
	}
}

/// Words ending -λοσ.
pub(crate) fn n652(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 647);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2064);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3985);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3986);
					s.is_empty()
				}
				['ι', 'γ', s @ ..] => {
					super::push(out, 3987);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5707);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6748);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3988);
					match s {
						[] => true,
						['σ', 'ε', s @ ..] => {
							super::push(out, 5708);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 6749);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', 'ι', s @ ..] => {
					super::push(out, 3989);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3990);
					s.is_empty()
				}
				['φ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5709);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 5710);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6751);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 2065);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3991);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5711);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6752);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3992);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5712);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2066);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3993);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5713);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2067);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3994);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5714);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3995);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5715);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6753);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3996);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5716);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6754);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5717);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6755);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5718);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3997);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3998);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3999);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5720);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 4001);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5722);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2068);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4002);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 4003);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4004);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4005);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5723);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6757);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5724);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6758);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7202);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 4007);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2069);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4008);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2070);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4010);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 4011);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5725);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4012);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5726);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6759);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7203);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 5727);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6760);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7204);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7353);
													s.is_empty()
												}
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
				['θ', s @ ..] => {
					super::push(out, 4013);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4014);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4015);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5728);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5729);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4016);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5730);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4017);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4018);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5731);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5732);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6761);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 4019);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 4020);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5734);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2071);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4021);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4022);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4023);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4024);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t2::n2084(s, out),
		['ο', s @ ..] => super::t2::n2085(s, out),
		['π', s @ ..] => {
			super::push(out, 2074);
			match s {
				[] => true,
				['ε', s @ ..] => super::t2::n4067(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t2::n2087(s, out),
		['υ', s @ ..] => super::t2::n2088(s, out),
		['χ', s @ ..] => super::t2::n2089(s, out),
		['ω', s @ ..] => super::t2::n2090(s, out),
		_ => false,
	}
}

/// Words ending -μοσ.
pub(crate) fn n653(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 648);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2079);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4061);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5770);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5771);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4062);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 5772);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4063);
					match s {
						[] => true,
						['ο', 'π', 'ο', s @ ..] => {
							super::push(out, 5773);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2080);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4065);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4066);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2081);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4067);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5774);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6780);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 7209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 4068);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2082);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2083);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2084);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4070);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5775);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6781);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4071);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4072);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4073);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 5776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4074);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 4075);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5777);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2085);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4076);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5778);
							match s {
								[] => true,
								['φ', 'ο', s @ ..] => {
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7210);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7355);
													match s {
														[] => true,
														['ε', s @ ..] => super::t2::n7432(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 7211);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4077);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2086);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4078);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2087);
			match s {
				[] => true,
				['δ', 'ο', 'ρ', s @ ..] => {
					super::push(out, 4079);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5779);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'φ', 'ο', s @ ..] => {
					super::push(out, 4080);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 4081);
					match s {
						[] => true,
						['ο', 'π', s @ ..] => {
							super::push(out, 5781);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6782);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4082);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5782);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 5783);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4083);
					match s {
						[] => true,
						['ο', 'ι', s @ ..] => {
							super::push(out, 5785);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6783);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5786);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6784);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2088);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4084);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5787);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2089);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4085);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4086);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5788);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6785);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4087);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5789);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 6786);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7212);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', 'α', s @ ..] => {
							super::push(out, 5790);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6787);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 7213);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 7356);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 5791);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6788);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 7214);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7357);
													s.is_empty()
												}
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
				['ο', s @ ..] => {
					super::push(out, 4088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2090);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4089);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5792);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t2::n2103(s, out),
		['χ', s @ ..] => {
			super::push(out, 2092);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 2093);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4096);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4097);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4098);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νοσ.
pub(crate) fn n654(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 649);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2094);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4099);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5798);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6791);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6792);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5799);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 4100);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5800);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6793);
									s.is_empty()
								}
								['χ', s @ ..] => {
									super::push(out, 6794);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 4101);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5801);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4102);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4103);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4104);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5802);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6795);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4105);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5803);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6796);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 5804);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 5805);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6797);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 5806);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6798);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 7217);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7358);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 5807);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6799);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4106);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4107);
					s.is_empty()
				}
				['ο', 'χ', s @ ..] => {
					super::push(out, 4108);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4109);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5808);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4110);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5809);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6800);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 5810);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 5811);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5812);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5813);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5814);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5815);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6802);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4111);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5816);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6803);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4112);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5818);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 5819);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5820);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6804);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5821);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4113);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5822);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 4114);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5823);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 6805);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 5824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 4115);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2095);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4116);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 5826);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6806);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 2096);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 4117);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2097);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4118);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 4119);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 4120);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5828);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6807);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7218);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 7359);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4121);
					match s {
						[] => true,
						['ε', s @ ..] => super::t2::n5867(s, out),
						['υ', s @ ..] => super::t2::n5868(s, out),
						_ => false,
					}
				}
				['ξ', s @ ..] => super::t2::n4151(s, out),
				['τ', s @ ..] => super::t2::n4152(s, out),
				['φ', s @ ..] => super::t2::n4153(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t2::n2110(s, out),
		['θ', s @ ..] => super::t2::n2111(s, out),
		['ι', s @ ..] => super::t2::n2112(s, out),
		['κ', s @ ..] => super::t2::n2113(s, out),
		['μ', s @ ..] => super::t2::n2114(s, out),
		['ν', s @ ..] => super::t2::n2115(s, out),
		['ο', s @ ..] => super::t2::n2116(s, out),
		['π', s @ ..] => super::t2::n2117(s, out),
		['ρ', s @ ..] => super::t2::n2118(s, out),
		['τ', s @ ..] => super::t2::n2119(s, out),
		['υ', s @ ..] => super::t2::n2120(s, out),
		['φ', s @ ..] => super::t2::n2121(s, out),
		['χ', s @ ..] => super::t2::n2122(s, out),
		['ω', s @ ..] => super::t2::n2123(s, out),
		_ => false,
	}
}

/// Words ending -ξοσ.
pub(crate) fn n655(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 650);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2112);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2113);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2114);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4186);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οοσ.
pub(crate) fn n656(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 651);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2115);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2116);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4187);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4188);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5893);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2117);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4189);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4190);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4191);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', 'ο', 'ρ', s @ ..] => {
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4193);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4194);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 2118);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ποσ.
pub(crate) fn n657(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 652);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2119);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4195);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4196);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2120);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2121);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4198);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4199);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4200);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5895);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6851);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7238);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4201);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5896);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2122);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4202);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2123);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4203);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5898);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6852);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6853);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7239);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4204);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5899);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5900);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2124);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4205);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5901);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6855);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7240);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4206);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5902);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 4207);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5903);
							s.is_empty()
						}
						['σ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6856);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7241);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7366);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 6857);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4208);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4209);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 5904);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6858);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7242);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 6859);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7243);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7367);
													s.is_empty()
												}
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
		['π', s @ ..] => {
			super::push(out, 2125);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4210);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2126);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4211);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5905);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2127);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4212);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5906);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4213);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5907);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6860);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ῥ', s @ ..] => {
					super::push(out, 4214);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2128);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4215);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5908);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4216);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5909);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6861);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7244);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4217);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5910);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6862);
									s.is_empty()
								}
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

/// Words ending -ροσ.
pub(crate) fn n658(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 653);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2129);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4218);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5911);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 4219);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 4220);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 4221);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5912);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 4222);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4223);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4224);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5913);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4225);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4226);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 2130);
			match s {
				[] => true,
				['ε', 'ν', s @ ..] => {
					super::push(out, 4227);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4228);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5915);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6863);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5916);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5917);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6864);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 7245);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4229);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5918);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2131);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4230);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5919);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6865);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7246);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5920);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5921);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 5922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 4231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 2132);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4232);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5923);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6866);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 7247);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 5924);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 6867);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7248);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 4233);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5926);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 6868);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4234);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2133);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4235);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4236);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4237);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4238);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4239);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4240);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5928);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6869);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7249);
											s.is_empty()
										}
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
		['η', s @ ..] => {
			super::push(out, 2134);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4241);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4242);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4243);
					match s {
						[] => true,
						['κ', 'υ', s @ ..] => {
							super::push(out, 5929);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4244);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5930);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4245);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2135);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4246);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5931);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4247);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4248);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5933);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4249);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 4250);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5935);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2136);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4251);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5936);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5937);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4252);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5938);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 5939);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6872);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 7250);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6873);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 5940);
							s.is_empty()
						}
						['π', 'η', s @ ..] => {
							super::push(out, 5941);
							s.is_empty()
						}
						['φ', s @ ..] => super::t2::n5980(s, out),
						_ => false,
					}
				}
				['κ', s @ ..] => super::t2::n4282(s, out),
				['ο', s @ ..] => super::t2::n4283(s, out),
				['σ', s @ ..] => {
					super::push(out, 4255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => super::t2::n2151(s, out),
		['ο', s @ ..] => super::t2::n2152(s, out),
		['π', s @ ..] => super::t2::n2153(s, out),
		['ρ', s @ ..] => super::t2::n2154(s, out),
		['τ', s @ ..] => super::t2::n2155(s, out),
		['υ', s @ ..] => super::t2::n2156(s, out),
		['φ', s @ ..] => super::t2::n2157(s, out),
		['χ', s @ ..] => super::t2::n2158(s, out),
		['ω', s @ ..] => super::t2::n2159(s, out),
		_ => false,
	}
}

/// Words ending -σοσ.
pub(crate) fn n659(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 654);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2146);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4300);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4301);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4302);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4303);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5994);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6907);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 7265);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4304);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5995);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6908);
									s.is_empty()
								}
								_ => false,
							}
						}
						['θ', s @ ..] => {
							super::push(out, 5996);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5997);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6909);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7266);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4305);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5998);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4306);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2147);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4307);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2148);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4308);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5999);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6911);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2149);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4309);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6000);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6912);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 6001);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4310);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6002);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4311);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6003);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6914);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4312);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4313);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4314);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6004);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6915);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2150);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4315);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2151);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4316);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4317);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6005);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2152);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4318);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 6006);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 6007);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4319);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 6008);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4320);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 6009);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 6010);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 6011);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2153);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4321);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6012);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4322);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6013);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6917);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7267);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 7372);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 6918);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7268);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 7373);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 6014);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6919);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 7269);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 7374);
													s.is_empty()
												}
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
				['ο', s @ ..] => {
					super::push(out, 4323);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4324);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6015);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6920);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 6016);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6921);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 4325);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4326);
					match s {
						[] => true,
						['ν', s @ ..] => super::t2::n6056(s, out),
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4327);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4328);
					s.is_empty()
				}
				['ο', s @ ..] => super::t2::n4358(s, out),
				['ρ', s @ ..] => super::t2::n4359(s, out),
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τοσ.
pub(crate) fn n660(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 655);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2155);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4331);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6021);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6924);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 6022);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6925);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 7272);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 6023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 4332);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4333);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6025);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4334);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4335);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4336);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6028);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6927);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2156);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4337);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 4338);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6029);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 6030);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4339);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6031);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4340);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6032);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6928);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 6033);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2157);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4341);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6034);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 4342);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4343);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4344);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 4345);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6037);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6929);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 7273);
											s.is_empty()
										}
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
		['ι', s @ ..] => {
			super::push(out, 2158);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4346);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6038);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6930);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 4347);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6039);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6931);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 7274);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4348);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6040);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6932);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4349);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6041);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 6933);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 7275);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 7376);
													s.is_empty()
												}
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
				['σ', s @ ..] => {
					super::push(out, 4350);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6042);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 6043);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6934);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2159);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4351);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6044);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 6045);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 6935);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7276);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4352);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6046);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6936);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4353);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4354);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 4355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2160);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4356);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 6048);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 6937);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 7277);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4357);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6049);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 6050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4358);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 6051);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4359);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6052);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2161);
			match s {
				[] => true,
				['ο', s @ ..] => super::t2::n4389(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t2::n2176(s, out),
		['π', s @ ..] => super::t2::n2177(s, out),
		['ρ', s @ ..] => super::t2::n2178(s, out),
		['σ', s @ ..] => super::t2::n2179(s, out),
		['τ', s @ ..] => super::t2::n2180(s, out),
		['υ', s @ ..] => super::t2::n2181(s, out),
		['ω', s @ ..] => super::t2::n2182(s, out),
		_ => false,
	}
}

/// Words ending -υοσ.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 656);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2169);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4386);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2170);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 2171);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2172);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 2173);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2174);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2175);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4388);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φοσ.
pub(crate) fn n662(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 657);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2176);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4389);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 4390);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6083);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6952);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4391);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6084);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6085);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'η', s @ ..] => {
					super::push(out, 4392);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6087);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4393);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6088);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['θ', s @ ..] => {
											super::push(out, 7282);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 7377);
													match s {
														[] => true,
														['ο', s @ ..] => super::t2::n7435(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 7283);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 7378);
													match s {
														[] => true,
														['ν', s @ ..] => super::t2::n7436(s, out),
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
				['τ', s @ ..] => {
					super::push(out, 4394);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6089);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2177);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4395);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4396);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6091);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2178);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 4397);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2179);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 4398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6092);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4399);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6093);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6955);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 6094);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2180);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4400);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6096);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2181);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4401);
					match s {
						[] => true,
						['ῥ', s @ ..] => {
							super::push(out, 6097);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4402);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4403);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4404);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 6100);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6957);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7284);
											s.is_empty()
										}
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
		['ο', s @ ..] => {
			super::push(out, 2182);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4405);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6101);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4406);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 6103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2183);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4407);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6104);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6958);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4408);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6105);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2184);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 4409);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6106);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6959);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2185);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4410);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 6107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4411);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6108);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6960);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 7285);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 6961);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 6109);
							match s {
								[] => true,
								['κ', s @ ..] => super::t2::n7003(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t2::n4441(s, out),
				['ρ', s @ ..] => super::t2::n4442(s, out),
				['σ', s @ ..] => super::t2::n4443(s, out),
				['τ', s @ ..] => {
					super::push(out, 4415);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

