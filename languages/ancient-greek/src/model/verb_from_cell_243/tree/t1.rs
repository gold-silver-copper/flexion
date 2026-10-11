use flexion::Path;

/// Words ending -λησα.
pub(crate) fn n280(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 280);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 789);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1509);
					match s {
						[] => true,
						['ε', '\u{1}', s @ ..] => {
							super::push(out, 2271);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2272);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2892);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1510);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2273);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2893);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1511);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2274);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2894);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 2275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1512);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1513);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 790);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1514);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2278);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1515);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2279);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2895);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 2280);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2896);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3298);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3503);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3586(s, out),
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
					super::push(out, 1516);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1517);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 792);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1518);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2283);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2897);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3299);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3504);
													s.is_empty()
												}
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
					super::push(out, 1519);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2284);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2898);
									s.is_empty()
								}
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
			super::push(out, 793);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2285);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2899);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3300);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3505);
													s.is_empty()
												}
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
							super::push(out, 2286);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 2900);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2287);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2901);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3301);
											s.is_empty()
										}
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
					super::push(out, 1522);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2288);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2902);
									s.is_empty()
								}
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
			super::push(out, 794);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2289);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2903);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2904);
									s.is_empty()
								}
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
			super::push(out, 795);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1524);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2290);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2905);
									s.is_empty()
								}
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
			super::push(out, 796);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1525);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2291);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2906);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3302);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3506);
													s.is_empty()
												}
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
					super::push(out, 1526);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2292);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2907);
									s.is_empty()
								}
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
			super::push(out, 797);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1527);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2293);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 798);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1528);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2294);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2908);
									s.is_empty()
								}
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

/// Words ending -μησα.
pub(crate) fn n281(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 281);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 799);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2295);
							match s {
								[] => true,
								['π', 'ε', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3303);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 3304);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3507);
													match s {
														[] => true,
														['ξ', '\u{1}', s @ ..] => super::t1::n3587(s, out),
														['σ', s @ ..] => super::t1::n3588(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3305);
											s.is_empty()
										}
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
			super::push(out, 800);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1530);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1531);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 801);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2296);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2909);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3306);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3508);
													s.is_empty()
												}
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
		['ι', s @ ..] => {
			super::push(out, 802);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2297);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2910);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2298);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 803);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2299);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2911);
									s.is_empty()
								}
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
			super::push(out, 804);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1536);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2300);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2912);
									s.is_empty()
								}
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
			super::push(out, 805);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1537);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1538);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2301);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2913);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1539);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1540);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 806);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1541);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2302);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 807);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1542);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2303);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1543);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2304);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2914);
									s.is_empty()
								}
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
			super::push(out, 808);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1544);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 809);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1545);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2305);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2915);
									s.is_empty()
								}
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

/// Words ending -νησα.
pub(crate) fn n282(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 282);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 810);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2306);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1547);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2307);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2917);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1548);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2308);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2918);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3307);
											s.is_empty()
										}
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
					super::push(out, 1549);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2309);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2919);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3308);
											s.is_empty()
										}
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
					super::push(out, 1550);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2310);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2920);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3309);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3509);
													s.is_empty()
												}
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
				['χ', s @ ..] => {
					super::push(out, 1551);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2311);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2921);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3310);
											s.is_empty()
										}
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
			super::push(out, 811);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1552);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 1553);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2312);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2922);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 3311);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2313);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2923);
									s.is_empty()
								}
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
			super::push(out, 812);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1554);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 813);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1555);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2314);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2924);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1556);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2315);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1557);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2316);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2925);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3312);
											s.is_empty()
										}
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
					super::push(out, 1558);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 814);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1560);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 815);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1561);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2320);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2321);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2926);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1562);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2322);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2927);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3313);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3510);
													s.is_empty()
												}
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
					super::push(out, 1563);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2323);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 816);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1564);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2324);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2928);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1565);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2325);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2929);
									s.is_empty()
								}
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
			super::push(out, 817);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1566);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2326);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2930);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3314);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 2327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2931);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1567);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1568);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1569);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2329);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1570);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2330);
							match s {
								[] => true,
								['η', 'π', 'α', '\u{1}', s @ ..] => {
									super::push(out, 2932);
									s.is_empty()
								}
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
			super::push(out, 818);
			s.is_empty()
		}
		['ρ', s @ ..] => super::t1::n819(s, out),
		['υ', s @ ..] => super::t1::n820(s, out),
		['ω', s @ ..] => super::t1::n821(s, out),
		_ => false,
	}
}

/// Words ending -ξησα.
pub(crate) fn n283(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 283);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1576);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2337);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 823);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2338);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οησα.
pub(crate) fn n284(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 284);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 824);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2339);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2340);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 825);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2342);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1581);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 828);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1582);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2343);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2936);
									s.is_empty()
								}
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

/// Words ending -πησα.
pub(crate) fn n285(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 285);
	match s {
		[] => true,
		['α', 'γ', 'η', '\u{1}', s @ ..] => {
			super::push(out, 829);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1584);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2344);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2937);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3317);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3513);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3589(s, out),
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
		['υ', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1585);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2345);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2938);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1586);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2346);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2939);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3318);
											s.is_empty()
										}
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
					super::push(out, 1587);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2347);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2940);
									s.is_empty()
								}
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
			super::push(out, 833);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1588);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2348);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2941);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2349);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρησα.
pub(crate) fn n286(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 286);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2350);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2942);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3319);
											s.is_empty()
										}
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
					super::push(out, 1591);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2351);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 2352);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2943);
									s.is_empty()
								}
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
			super::push(out, 836);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2353);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2944);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2354);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2945);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3320);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3514);
													s.is_empty()
												}
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
							super::push(out, 2355);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2946);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3321);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 3322);
											match s {
												[] => true,
												['α', '\u{1}', s @ ..] => {
													super::push(out, 3515);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 2947);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3323);
											s.is_empty()
										}
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
			super::push(out, 837);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1595);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2356);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2948);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3324);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3516);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3590(s, out),
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
					super::push(out, 1597);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2357);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2358);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2949);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3325);
											s.is_empty()
										}
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
					super::push(out, 1599);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2359);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2950);
									s.is_empty()
								}
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
			super::push(out, 839);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1600);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2360);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2951);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3326);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3517);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3591(s, out),
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
									super::push(out, 2952);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3327);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3518);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3592(s, out),
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
				['θ', s @ ..] => {
					super::push(out, 1601);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1602);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2362);
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 2953);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3328);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3519);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3593(s, out),
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
				['φ', s @ ..] => {
					super::push(out, 1603);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2363);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 840);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1604);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2364);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2365);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2954);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t1::n841(s, out),
		['τ', s @ ..] => {
			super::push(out, 842);
			match s {
				[] => true,
				['ε', s @ ..] => super::t1::n1607(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n843(s, out),
		['χ', s @ ..] => super::t1::n844(s, out),
		['ω', s @ ..] => super::t1::n845(s, out),
		_ => false,
	}
}

/// Words ending -σησα.
pub(crate) fn n287(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 287);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 846);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1615);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1616);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2376);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2964);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3333);
											s.is_empty()
										}
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
			super::push(out, 847);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 848);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1618);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 849);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1619);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 850);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1620);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τησα.
pub(crate) fn n288(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 851);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 2382);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2965);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2383);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2966);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2967);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3334);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 3522);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3595(s, out),
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
					super::push(out, 1623);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 852);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1624);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2384);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2385);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2968);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3335);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3523);
													s.is_empty()
												}
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
					super::push(out, 1625);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2386);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2969);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3336);
											s.is_empty()
										}
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
					super::push(out, 1626);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2387);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2970);
									s.is_empty()
								}
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
			super::push(out, 853);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1627);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2388);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1628);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2389);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2971);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2390);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2391);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2972);
									s.is_empty()
								}
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
			super::push(out, 854);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2392);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2973);
									s.is_empty()
								}
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
			super::push(out, 855);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2393);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
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
			super::push(out, 856);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1633);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2394);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2975);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1634);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2395);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2976);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3337);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2396);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2977);
									s.is_empty()
								}
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
			super::push(out, 857);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1635);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1636);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2398);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1637);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2399);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 859);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1638);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2400);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2978);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t1::n860(s, out),
		['υ', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1644);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2413);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2998);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 3352);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', 'λ', 'ε', 'τ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 1645);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1646);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2414);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1647);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2415);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υησα.
pub(crate) fn n289(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 289);
	s.is_empty()
}

/// Words ending -φησα.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2416);
							match s {
								[] => true,
								['ψ', s @ ..] => {
									super::push(out, 2999);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3353);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3532);
													s.is_empty()
												}
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
				['ρ', s @ ..] => {
					super::push(out, 1649);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2417);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 864);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1650);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1652);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2418);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3000);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3354);
											s.is_empty()
										}
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

/// Words ending -χησα.
pub(crate) fn n291(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2419);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2420);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3001);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3355);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3533);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3603(s, out),
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
		['ι', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1654);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2421);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3002);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3356);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3534);
													s.is_empty()
												}
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
		['ο', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1655);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2422);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3003);
									s.is_empty()
								}
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
			super::push(out, 870);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1656);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2423);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2424);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3005);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3357);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3535);
													match s {
														[] => true,
														['ω', s @ ..] => super::t1::n3604(s, out),
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
		_ => false,
	}
}

/// Words ending -ψησα.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1659);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2425);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3006);
									s.is_empty()
								}
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
			super::push(out, 873);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2426);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3007);
									s.is_empty()
								}
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

/// The word ανεποδισα.
pub(crate) fn n3538(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3536);
	s.is_empty()
}

/// Words ending -περιεδεισα.
pub(crate) fn n3539(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3537);
	s.is_empty()
}

/// Words ending -αλισα.
pub(crate) fn n893(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 893);
	s.is_empty()
}

/// Words ending -βλισα.
pub(crate) fn n894(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 894);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1682);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2443);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηλισα.
pub(crate) fn n895(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 895);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1683);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1684);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2444);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3016);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υλισα.
pub(crate) fn n896(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 896);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1685);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2445);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μισα.
pub(crate) fn n302(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 302);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 897);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νισα.
pub(crate) fn n303(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 898);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 899);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1686);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2446);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 900);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 901);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2447);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3018);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2448);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3019);
									s.is_empty()
								}
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
			super::push(out, 902);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1689);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2449);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3020);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3362);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3538);
													s.is_empty()
												}
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
					super::push(out, 1690);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2450);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1691);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οισα.
pub(crate) fn n304(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 903);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πισα.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2452);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3022);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3363);
											s.is_empty()
										}
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
			super::push(out, 905);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1694);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρισα.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1695);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2454);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3023);
									s.is_empty()
								}
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
			super::push(out, 907);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1696);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1697);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2455);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3024);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 909);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 910);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 911);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1701);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2456);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2457);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3025);
									s.is_empty()
								}
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
			super::push(out, 912);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 913);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1702);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 914);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1703);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2458);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3026);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3364);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3539);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3605(s, out),
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
				['φ', s @ ..] => {
					super::push(out, 1704);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3027);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1705);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τισα.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 915);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1706);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 916);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1707);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2461);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3028);
									s.is_empty()
								}
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
			super::push(out, 917);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 918);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1709);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2462);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3029);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2463);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3030);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3365);
											s.is_empty()
										}
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
			super::push(out, 919);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φισα.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 308);
	s.is_empty()
}

/// Words ending -χισα.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 309);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 920);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1712);
					match s {
						[] => true,
						['τ', 'ε', 'ι', s @ ..] => {
							super::push(out, 2465);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3031);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3366);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3540);
													s.is_empty()
												}
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
			super::push(out, 921);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωισα.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 310);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 922);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1713);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2466);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3032);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3367);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3541);
													match s {
														[] => true,
														['η', s @ ..] => super::t1::n3606(s, out),
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
		['λ', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1714);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2467);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ευσα.
pub(crate) fn n326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 326);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 942);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2483);
							match s {
								[] => true,
								['ε', 'ι', s @ ..] => {
									super::push(out, 3043);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3372);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3542);
													s.is_empty()
												}
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
		['δ', s @ ..] => {
			super::push(out, 943);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1736);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1737);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 944);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 945);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1738);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2484);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2485);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3044);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3373);
											s.is_empty()
										}
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
					super::push(out, 1740);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 946);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1741);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2487);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 2488);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3045);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1743);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2489);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3046);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2490);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3047);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3374);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 3375);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 3048);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3376);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ξ', s @ ..] => {
									super::push(out, 3049);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3377);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 3050);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3378);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 3379);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3543);
													s.is_empty()
												}
												['τ', s @ ..] => {
													super::push(out, 3544);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3607(s, out),
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
									super::push(out, 3051);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3380);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3545);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 3052);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3381);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3546);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3382);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3547);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3053);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3383);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3548);
													s.is_empty()
												}
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
					super::push(out, 1745);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2491);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 3054);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3384);
											s.is_empty()
										}
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
					super::push(out, 1746);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2492);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3055);
									s.is_empty()
								}
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
			super::push(out, 947);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2493);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1748);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => super::t1::n948(s, out),
		['ξ', s @ ..] => {
			super::push(out, 949);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 950);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1754);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['π', s @ ..] => super::t1::n2498(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				['σ', s @ ..] => super::t1::n1757(s, out),
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t1::n951(s, out),
		['σ', s @ ..] => super::t1::n952(s, out),
		['τ', s @ ..] => super::t1::n953(s, out),
		['χ', s @ ..] => super::t1::n954(s, out),
		['ψ', s @ ..] => super::t1::n955(s, out),
		_ => false,
	}
}

/// Words ending -καθειλκυσα.
pub(crate) fn n3561(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3559);
	s.is_empty()
}

/// Words ending -ειλυσα.
pub(crate) fn n1777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1777);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κλυσα.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 964);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2518);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2519);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3082);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3410);
									s.is_empty()
								}
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

/// Words ending -φλυσα.
pub(crate) fn n965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 965);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1779);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2520);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωλυσα.
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 966);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2521);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3083);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 3084);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μυσα.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2522);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3085);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3411);
											s.is_empty()
										}
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
			super::push(out, 968);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1782);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 969);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1783);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2523);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3086);
									s.is_empty()
								}
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

/// Words ending -νυσα.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 970);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1784);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2524);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2525);
							match s {
								[] => true,
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 3087);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3088);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3412);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3560);
													s.is_empty()
												}
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

/// Words ending -ξυσα.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ουσα.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 972);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1786);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 973);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2526);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3089);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 3090);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3413);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3561);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3614(s, out),
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
				['ω', s @ ..] => {
					super::push(out, 1788);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2527);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2528);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3091);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3414);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2529);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3092);
									s.is_empty()
								}
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

/// Words ending -πυσα.
pub(crate) fn n335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 335);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 975);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυσα.
pub(crate) fn n336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 336);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2531);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3093);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3415);
											s.is_empty()
										}
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
			super::push(out, 978);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2534);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3094);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3416);
											s.is_empty()
										}
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
			super::push(out, 981);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2535);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3095);
									s.is_empty()
								}
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

/// Words ending -συσα.
pub(crate) fn n337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 337);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1795);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2536);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3096);
									s.is_empty()
								}
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

/// Words ending -τυσα.
pub(crate) fn n338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 338);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2538);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3097);
									s.is_empty()
								}
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
			super::push(out, 984);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2539);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1798);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χυσα.
pub(crate) fn n339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 339);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2540);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3098);
									s.is_empty()
								}
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
			super::push(out, 987);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2541);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3099);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3417);
											s.is_empty()
										}
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

/// Words ending -κατεσκηνωσα.
pub(crate) fn n3564(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3562);
	s.is_empty()
}

/// Words ending -υμνωσα.
pub(crate) fn n1817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1817);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2558);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3109);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3422);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνωσα.
pub(crate) fn n1012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1012);
	s.is_empty()
}

/// Words ending -ρωσα.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1013);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1818);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2559);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3110);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3423);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3563);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n3615(s, out),
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
		['β', s @ ..] => {
			super::push(out, 1014);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1819);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2560);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1015);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1820);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1016);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1017);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1822);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1018);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1823);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2562);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2563);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3111);
									s.is_empty()
								}
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
			super::push(out, 1019);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1824);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1825);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2565);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1826);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2566);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1827);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2567);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3113);
									s.is_empty()
								}
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

/// Words ending -σωσα.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1828);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1829);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1830);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2569);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3114);
									s.is_empty()
								}
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
			super::push(out, 1022);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τωσα.
pub(crate) fn n353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	s.is_empty()
}

/// Words ending -φωσα.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1024);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χωσα.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	s.is_empty()
}

/// Words ending -υθον.
pub(crate) fn n410(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 410);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1940);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2669);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3191);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3457);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3569);
													s.is_empty()
												}
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
							super::push(out, 2670);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3192);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3458);
											s.is_empty()
										}
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

/// The word εισεβαλον.
pub(crate) fn n3572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3570);
	s.is_empty()
}

/// Words ending -προσεβαλον.
pub(crate) fn n3573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3571);
	s.is_empty()
}

/// Words ending -περιετεμον.
pub(crate) fn n3574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3572);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3615);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ανεβλαστον.
pub(crate) fn n3575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3573);
	s.is_empty()
}

/// Words ending -συνεστιχον.
pub(crate) fn n3576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3574);
	s.is_empty()
}

/// The word συνετυχον.
pub(crate) fn n3577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3575);
	s.is_empty()
}

/// Words ending -ρετυχον.
pub(crate) fn n2724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2724);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3224);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3475);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πλειστηριασα.
pub(crate) fn n3619(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3617);
	s.is_empty()
}

/// The word εζωοποιησα.
pub(crate) fn n3583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3581);
	s.is_empty()
}

/// Words ending -προσεδοκησα.
pub(crate) fn n3584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3582);
	s.is_empty()
}

/// Words ending -εξεπολιορκησα.
pub(crate) fn n3585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3583);
	s.is_empty()
}

/// Words ending -προωφειλησα.
pub(crate) fn n3586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3584);
	s.is_empty()
}

/// The word ξυνεπολεμησα.
pub(crate) fn n3587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3585);
	s.is_empty()
}

/// Words ending -συνεπολεμησα.
pub(crate) fn n3588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3586);
	s.is_empty()
}

/// Words ending -ρνησα.
pub(crate) fn n819(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 819);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1571);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2331);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2933);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3315);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3511);
											s.is_empty()
										}
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

/// Words ending -υνησα.
pub(crate) fn n820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 820);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1572);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2333);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2934);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1573);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2335);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3316);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3512);
											s.is_empty()
										}
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

/// Words ending -ωνησα.
pub(crate) fn n821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 821);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1574);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1575);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2336);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word επεσκοπησα.
pub(crate) fn n3589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3587);
	s.is_empty()
}

/// Words ending -εναυκληρησα.
pub(crate) fn n3590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3588);
	s.is_empty()
}

/// Words ending -μακρηγορησα.
pub(crate) fn n3591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3589);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3618);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κατηγορησα.
pub(crate) fn n3592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3590);
	s.is_empty()
}

/// Words ending -επεζοπορησα.
pub(crate) fn n3593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3591);
	s.is_empty()
}

/// Words ending -ρρησα.
pub(crate) fn n841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 841);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1605);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2366);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2955);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3329);
									s.is_empty()
								}
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
			super::push(out, 1606);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2367);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ετρησα.
pub(crate) fn n1607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1607);
	s.is_empty()
}

/// Words ending -υρησα.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1608);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χρησα.
pub(crate) fn n844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 844);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1609);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2369);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωρησα.
pub(crate) fn n845(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 845);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1610);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1611);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2371);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1612);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1613);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2373);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2959);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1614);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2374);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2960);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3330);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2961);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3331);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3520);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2962);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2963);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3332);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3521);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3592);
													s.is_empty()
												}
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
				['ο', 'ν', 'ε', 'τ', 'σ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δημοκρατησα.
pub(crate) fn n3595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3593);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3619);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -στησα.
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 860);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2401);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2979);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3338);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3524);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3594);
													s.is_empty()
												}
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
		['ε', s @ ..] => {
			super::push(out, 1640);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2403);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2980);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3339);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2981);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3340);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3525);
											s.is_empty()
										}
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
					super::push(out, 2404);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2982);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3341);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2983);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2984);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3342);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 2405);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2985);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2406);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2986);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3343);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3526);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3595);
													s.is_empty()
												}
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
							super::push(out, 2987);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2988);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2407);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2989);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3344);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2408);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2990);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3345);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 3527);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3596);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 3528);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3597);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3529);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3598);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3622(s, out),
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
							super::push(out, 2991);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3346);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2992);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3347);
									s.is_empty()
								}
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
			super::push(out, 1641);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2409);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2993);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2410);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2994);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 3348);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3530);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 3599);
													s.is_empty()
												}
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
		['ο', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2411);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2995);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3349);
									s.is_empty()
								}
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
			super::push(out, 1643);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2412);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2996);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3350);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3531);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3600);
													s.is_empty()
												}
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
							super::push(out, 2997);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3351);
									s.is_empty()
								}
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

/// Words ending -νεναυμαχησα.
pub(crate) fn n3603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3601);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 3621);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωλιγοψυχησα.
pub(crate) fn n3604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3602);
	s.is_empty()
}

/// The word ανεγνωρισα.
pub(crate) fn n3605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3603);
	s.is_empty()
}

/// Words ending -ηγχονιζωισα.
pub(crate) fn n3606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3604);
	s.is_empty()
}

/// Words ending -ντεπεπλευσα.
pub(crate) fn n3607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3605);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3622);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νευσα.
pub(crate) fn n948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 948);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1749);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1750);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1751);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2495);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3056);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3385);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3549);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3606);
													s.is_empty()
												}
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
							super::push(out, 3057);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3386);
									s.is_empty()
								}
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
			super::push(out, 1752);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2496);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3058);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3387);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 3059);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3388);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 3389);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 3060);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3390);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3550);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3607);
													s.is_empty()
												}
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
		['υ', s @ ..] => {
			super::push(out, 1753);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2497);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3061);
							match s {
								[] => true,
								['ι', 'κ', 'ε', 'ν', s @ ..] => {
									super::push(out, 3391);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3551);
											match s {
												[] => true,
												['ξ', '\u{1}', s @ ..] => {
													super::push(out, 3608);
													s.is_empty()
												}
												['σ', s @ ..] => {
													super::push(out, 3609);
													s.is_empty()
												}
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

/// Words ending -πιπευσα.
pub(crate) fn n2498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2498);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 3062);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3392);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σπευσα.
pub(crate) fn n1757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1757);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2499);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3063);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρευσα.
pub(crate) fn n951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 951);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2500);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1760);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2501);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3064);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3393);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3552);
											s.is_empty()
										}
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
		['ρ', s @ ..] => {
			super::push(out, 1761);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2502);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3065);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 3066);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3395);
									s.is_empty()
								}
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
			super::push(out, 1762);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2503);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3067);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 3068);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3396);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3553);
											s.is_empty()
										}
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

/// Words ending -σευσα.
pub(crate) fn n952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 952);
	s.is_empty()
}

/// Words ending -τευσα.
pub(crate) fn n953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 953);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1763);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3069);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1764);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2505);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3070);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3397);
									s.is_empty()
								}
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
			super::push(out, 1765);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2506);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3071);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3398);
									s.is_empty()
								}
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
			super::push(out, 1766);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2507);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 3072);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3399);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3554);
											s.is_empty()
										}
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
			super::push(out, 1767);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2508);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3073);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3400);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3555);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 3401);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3556);
											s.is_empty()
										}
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
					super::push(out, 2509);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3074);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3402);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 3075);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3403);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 3076);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3404);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3557);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3610);
													s.is_empty()
												}
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
		['υ', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2510);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3077);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3405);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3558);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3611);
													s.is_empty()
												}
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

/// Words ending -χευσα.
pub(crate) fn n954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 954);
	s.is_empty()
}

/// Words ending -ψευσα.
pub(crate) fn n955(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 955);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1769);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2511);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -προσεκρουσα.
pub(crate) fn n3614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3612);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3623);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεταρταρωσα.
pub(crate) fn n3615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3613);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3624);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αποκατεστησα.
pub(crate) fn n3622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3620);
	s.is_empty()
}

