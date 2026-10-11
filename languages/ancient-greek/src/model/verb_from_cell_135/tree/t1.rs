use flexion::Path;

/// Words ending -ρησω.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 854);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1615);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2325);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2872);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3216);
											s.is_empty()
										}
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
					super::push(out, 1616);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2326);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2873);
									s.is_empty()
								}
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
			super::push(out, 855);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1617);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 2327);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2874);
									s.is_empty()
								}
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
			super::push(out, 856);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1619);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 857);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1620);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1621);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2330);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2875);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3217);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3404);
													s.is_empty()
												}
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
					super::push(out, 1622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1623);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2331);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2332);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2876);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3218);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2333);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2877);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 2334);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2878);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3219);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 2335);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2879);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2880);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3220);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 2336);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2881);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 2882);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3221);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3405);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3478(s, out),
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
					super::push(out, 1624);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2337);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2883);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2884);
									s.is_empty()
								}
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
			super::push(out, 859);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1625);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2338);
							match s {
								[] => true,
								['ρ', 'κ', s @ ..] => {
									super::push(out, 2885);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3222);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3406);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3479(s, out),
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
					super::push(out, 1626);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2339);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2886);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3223);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3407);
													s.is_empty()
												}
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
					super::push(out, 1627);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2340);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2887);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1628);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2341);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2888);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2342);
							match s {
								[] => true,
								['ζ', s @ ..] => {
									super::push(out, 2889);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3224);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3408);
													s.is_empty()
												}
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
					super::push(out, 1629);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2343);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t1::n860(s, out),
		['ρ', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['ε', s @ ..] => super::t1::n1632(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t1::n862(s, out),
		['υ', s @ ..] => super::t1::n863(s, out),
		['χ', s @ ..] => super::t1::n864(s, out),
		['ω', s @ ..] => super::t1::n865(s, out),
		_ => false,
	}
}

/// Words ending -σησω.
pub(crate) fn n319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 319);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1645);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2362);
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
			super::push(out, 867);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1646);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τησω.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 868);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1647);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1648);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2363);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2906);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3233);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3412);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3482(s, out),
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
		['ε', s @ ..] => {
			super::push(out, 869);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1649);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1650);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2365);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2908);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3234);
											s.is_empty()
										}
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
					super::push(out, 1651);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 870);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 871);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1653);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2367);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2368);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2909);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 2369);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2910);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2370);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2911);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3235);
											s.is_empty()
										}
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
					super::push(out, 1654);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2371);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2912);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3236);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3413);
													s.is_empty()
												}
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
		['κ', s @ ..] => {
			super::push(out, 872);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1655);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2372);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2913);
									match s {
										[] => true,
										['γ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3237);
											s.is_empty()
										}
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
					super::push(out, 1656);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2373);
							match s {
								[] => true,
								['ο', s @ ..] => {
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
		['ν', s @ ..] => {
			super::push(out, 873);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1657);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2374);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 2375);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2915);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3238);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2376);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2916);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1658);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2377);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2917);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3239);
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
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1659);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2379);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2918);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3240);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3415);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3483(s, out),
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
		['ο', s @ ..] => {
			super::push(out, 874);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1660);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1661);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2381);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 875);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1662);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2382);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 876);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1663);
					match s {
						[] => true,
						['μ', s @ ..] => super::t1::n2384(s, out),
						['ξ', s @ ..] => super::t1::n2385(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t1::n877(s, out),
		['υ', s @ ..] => super::t1::n878(s, out),
		['ω', s @ ..] => super::t1::n879(s, out),
		_ => false,
	}
}

/// Words ending -υησω.
pub(crate) fn n321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 321);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 880);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 881);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 882);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1672);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2404);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2942);
									s.is_empty()
								}
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

/// Words ending -φησω.
pub(crate) fn n322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 322);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 883);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 884);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1673);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 1674);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2405);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1675);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1676);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2406);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -χησω.
pub(crate) fn n323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 323);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1677);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2407);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 2408);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2944);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3261);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3428);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n3491(s, out),
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
				['ν', s @ ..] => {
					super::push(out, 1678);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2409);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2945);
									s.is_empty()
								}
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
			super::push(out, 889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1679);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 890);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1681);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2410);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2946);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3262);
											s.is_empty()
										}
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
			super::push(out, 891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1682);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 1683);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2411);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 892);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1684);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2412);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2947);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3263);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3429);
													s.is_empty()
												}
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
			super::push(out, 894);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1686);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψησω.
pub(crate) fn n324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 324);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 895);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1687);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2413);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2948);
									s.is_empty()
								}
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
			super::push(out, 896);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1689);
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
		_ => false,
	}
}

/// Words ending -περικλεισω.
pub(crate) fn n3434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3430);
	s.is_empty()
}

/// Words ending -σκανδαλισω.
pub(crate) fn n3435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3431);
	s.is_empty()
}

/// Words ending -ποισω.
pub(crate) fn n917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 917);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1709);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1710);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2429);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ροισω.
pub(crate) fn n918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 918);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1711);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2430);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2957);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1712);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2431);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σοισω.
pub(crate) fn n919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 919);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1713);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2432);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2959);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3268);
									s.is_empty()
								}
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

/// Words ending -τοισω.
pub(crate) fn n920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 920);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1714);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2433);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πισω.
pub(crate) fn n331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 331);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 921);
			match s {
				[] => true,
				['ῥ', s @ ..] => {
					super::push(out, 1715);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 922);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρισω.
pub(crate) fn n332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 332);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 923);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2434);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2961);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3269);
											s.is_empty()
										}
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
			super::push(out, 924);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1717);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 925);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2435);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2962);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3270);
											s.is_empty()
										}
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
					super::push(out, 1719);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2436);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1720);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2437);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2963);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2438);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2964);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3271);
											s.is_empty()
										}
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

/// Words ending -τισω.
pub(crate) fn n333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 333);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 926);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1721);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2439);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2965);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3272);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3432);
													s.is_empty()
												}
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
		['κ', s @ ..] => {
			super::push(out, 928);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 929);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1722);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2440);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2966);
									s.is_empty()
								}
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
			super::push(out, 930);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1723);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1724);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2441);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χισω.
pub(crate) fn n334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 334);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1725);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2442);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2967);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3273);
											s.is_empty()
										}
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

/// The word αποθραυσω.
pub(crate) fn n3437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3433);
	s.is_empty()
}

/// Words ending -ευσω.
pub(crate) fn n346(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 346);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 953);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2455);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2456);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2975);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3277);
											s.is_empty()
										}
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
					super::push(out, 1746);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2457);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2976);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3278);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3434);
													s.is_empty()
												}
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
		['γ', s @ ..] => {
			super::push(out, 954);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1747);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 955);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2458);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2459);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2978);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3279);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3435);
													s.is_empty()
												}
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
					super::push(out, 1749);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2460);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2979);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3280);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3436);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3492(s, out),
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
				['η', s @ ..] => {
					super::push(out, 1750);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1751);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2461);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2980);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 2462);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2981);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3281);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3437);
													s.is_empty()
												}
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
			super::push(out, 956);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1752);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1753);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 957);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1754);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2464);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1756);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2466);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 958);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1757);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 1758);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2467);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2982);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3282);
											s.is_empty()
										}
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
					super::push(out, 1759);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2468);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2983);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1760);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2469);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 959);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1761);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1762);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2470);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2984);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2471);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2985);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3283);
											s.is_empty()
										}
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
					super::push(out, 1764);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2472);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2986);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3284);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2987);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3285);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => super::t1::n2474(s, out),
						_ => false,
					}
				}
				['υ', s @ ..] => super::t1::n1765(s, out),
				['ω', s @ ..] => super::t1::n1766(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n960(s, out),
		['ν', s @ ..] => super::t1::n961(s, out),
		['ξ', s @ ..] => super::t1::n962(s, out),
		['π', s @ ..] => super::t1::n963(s, out),
		['ρ', s @ ..] => super::t1::n964(s, out),
		['σ', s @ ..] => super::t1::n965(s, out),
		['τ', s @ ..] => super::t1::n966(s, out),
		['φ', s @ ..] => super::t1::n967(s, out),
		['χ', s @ ..] => super::t1::n968(s, out),
		['ψ', s @ ..] => super::t1::n969(s, out),
		_ => false,
	}
}

/// Words ending -επιδιαλυσω.
pub(crate) fn n3452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3448);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3493);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κλυσω.
pub(crate) fn n978(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 978);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1815);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1816);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2525);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3022);
							match s {
								[] => true,
								['κ', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 1817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ολυσω.
pub(crate) fn n980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 980);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1819);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2526);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3023);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3024);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3313);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3025);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3314);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3449);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3494);
													s.is_empty()
												}
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
							super::push(out, 3026);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3315);
									s.is_empty()
								}
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
			super::push(out, 1820);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωλυσω.
pub(crate) fn n982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 982);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1822);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2528);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3027);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3316);
									s.is_empty()
								}
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

/// Words ending -μυσω.
pub(crate) fn n350(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 350);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 983);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 984);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1823);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2529);
							match s {
								[] => true,
								['κ', s @ ..] => {
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
		['μ', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1824);
					match s {
						[] => true,
						['κ', s @ ..] => {
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

/// Words ending -νυσω.
pub(crate) fn n351(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 351);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1825);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1826);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2531);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1827);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2532);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2533);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3029);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3317);
											s.is_empty()
										}
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

/// Words ending -ουσω.
pub(crate) fn n352(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 352);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1828);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1829);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1830);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2534);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3030);
									s.is_empty()
								}
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
			super::push(out, 990);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1831);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2535);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 2536);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3031);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2537);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 2539);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3032);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3318);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3450);
													s.is_empty()
												}
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
					super::push(out, 1832);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2540);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2541);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3033);
									s.is_empty()
								}
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

/// Words ending -πυσω.
pub(crate) fn n353(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 353);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρυσω.
pub(crate) fn n354(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 354);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 992);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1833);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2542);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3034);
									s.is_empty()
								}
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
			super::push(out, 993);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1834);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2543);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3035);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 994);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1835);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2544);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3036);
									s.is_empty()
								}
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
			super::push(out, 995);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1836);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2545);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1837);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -συσω.
pub(crate) fn n355(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 355);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 997);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1838);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2546);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τυσω.
pub(crate) fn n356(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 356);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 998);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1839);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1840);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2547);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3037);
									s.is_empty()
								}
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
			super::push(out, 999);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1841);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2548);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3038);
									s.is_empty()
								}
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
			super::push(out, 1000);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φυσω.
pub(crate) fn n357(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 357);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1001);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1002);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1842);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1843);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2550);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3039);
									s.is_empty()
								}
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

/// Words ending -χυσω.
pub(crate) fn n358(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 358);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1844);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1845);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2552);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2553);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3040);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3320);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3451);
													s.is_empty()
												}
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

/// Words ending -ῥαφανιδωσω.
pub(crate) fn n3456(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3452);
	s.is_empty()
}

/// Words ending -ανταποδωσω.
pub(crate) fn n3457(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3453);
	s.is_empty()
}

/// Words ending -αταπροδωσω.
pub(crate) fn n3458(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3454);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3495);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τοιχειωσω.
pub(crate) fn n3329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3325);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 3455);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οιωσω.
pub(crate) fn n1019(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1019);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2566);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3051);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3326);
									s.is_empty()
								}
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

/// Words ending -ριωσω.
pub(crate) fn n1020(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1020);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1865);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2567);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3052);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3327);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3456);
											s.is_empty()
										}
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

/// Words ending -κωσω.
pub(crate) fn n365(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 365);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1866);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2568);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λωσω.
pub(crate) fn n366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 366);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1022);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1867);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1023);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2570);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3053);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3328);
											s.is_empty()
										}
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
					super::push(out, 1870);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3054);
									s.is_empty()
								}
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
			super::push(out, 1024);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1871);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μωσω.
pub(crate) fn n367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 367);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1872);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2572);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -νωσω.
pub(crate) fn n368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 368);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1026);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1873);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1874);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2573);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3056);
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
						['τ', s @ ..] => {
							super::push(out, 2574);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3057);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3330);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3457);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3500(s, out),
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
		['ε', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1876);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2575);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3058);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3331);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3458);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3501(s, out),
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
			super::push(out, 1030);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1877);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1878);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1032);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1879);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρωσω.
pub(crate) fn n369(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 369);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1880);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2577);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1034);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1881);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2578);
							match s {
								[] => true,
								['ε', 'λ', 'ε', 'π', s @ ..] => {
									super::push(out, 3059);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3332);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3459);
													s.is_empty()
												}
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
		['η', s @ ..] => {
			super::push(out, 1035);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1882);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1883);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2579);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3060);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
		['τ', s @ ..] => {
			super::push(out, 1037);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1885);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1038);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1886);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2582);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3061);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3334);
											s.is_empty()
										}
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

/// Words ending -σωσω.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 1039);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1887);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τωσω.
pub(crate) fn n371(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 371);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 1040);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φωσω.
pub(crate) fn n372(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 372);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1888);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χωσω.
pub(crate) fn n373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 373);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1042);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψωσω.
pub(crate) fn n374(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 374);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1889);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥωσω.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1044);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -στρεψω.
pub(crate) fn n1925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1925);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2611);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3343);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3460);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 3461);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3498);
											match s {
												[] => true,
												['σ', s @ ..] => {
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
				['τ', s @ ..] => {
					super::push(out, 3085);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3344);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3462);
									s.is_empty()
								}
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
			super::push(out, 2612);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3086);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2613);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3087);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3346);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3347);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3463);
									s.is_empty()
								}
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

/// Words ending -στεψω.
pub(crate) fn n1075(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1075);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1926);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2614);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3088);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φεψω.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1076);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1927);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥεψω.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1077);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word παραλειψω.
pub(crate) fn n3468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3464);
	s.is_empty()
}

/// Words ending -ενδιατριψω.
pub(crate) fn n3469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3465);
	s.is_empty()
}

/// Words ending -αμφικαλυψω.
pub(crate) fn n3470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3466);
	s.is_empty()
}

/// Words ending -ωθησω.
pub(crate) fn n802(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 802);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1520);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1521);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2253);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1522);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2822);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word υπερνικησω.
pub(crate) fn n3476(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3472);
	s.is_empty()
}

/// Words ending -ροσκυνησω.
pub(crate) fn n3216(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3212);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3401);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξυνησω.
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1596);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συνησω.
pub(crate) fn n1597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1597);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2313);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωνησω.
pub(crate) fn n843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 843);
	match s {
		[] => true,
		['φ', s @ ..] => {
			super::push(out, 1598);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2314);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word επισκοπησω.
pub(crate) fn n3477(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3473);
	s.is_empty()
}

/// The word συγχαιρησω.
pub(crate) fn n3478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3474);
	s.is_empty()
}

/// The word μακρηγορησω.
pub(crate) fn n3479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3475);
	s.is_empty()
}

/// Words ending -πρησω.
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 860);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1630);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1631);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2344);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2890);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ερρησω.
pub(crate) fn n1632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1632);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2345);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τρησω.
pub(crate) fn n862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 862);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1633);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2346);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υρησω.
pub(crate) fn n863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 863);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1634);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2347);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 2348);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2891);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2349);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2892);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 1635);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1636);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2350);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2893);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3225);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3409);
											s.is_empty()
										}
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
					super::push(out, 2351);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2894);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3226);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2352);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2895);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1637);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2353);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2896);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3227);
									s.is_empty()
								}
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

/// Words ending -χρησω.
pub(crate) fn n864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 864);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1638);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1639);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωρησω.
pub(crate) fn n865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 865);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1640);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1641);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1642);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2356);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2898);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1643);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1644);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2358);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2899);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3228);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3410);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3476);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3504(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 3477);
													s.is_empty()
												}
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
				['γ', s @ ..] => {
					super::push(out, 2359);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2900);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3229);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2360);
					match s {
						[] => true,
						['ν', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 2901);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2902);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3230);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2903);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3231);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2361);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2904);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3232);
									match s {
										[] => true,
										['π', s @ ..] => {
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
		_ => false,
	}
}

/// The word επικρατησω.
pub(crate) fn n3482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3478);
	s.is_empty()
}

/// Words ending -ναισχυντησω.
pub(crate) fn n3483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3479);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3501);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μαρτησω.
pub(crate) fn n2384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2383);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2919);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3241);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3242);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3416);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2920);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3243);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξαρτησω.
pub(crate) fn n2385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2384);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2921);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -στησω.
pub(crate) fn n877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 877);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1664);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2385);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2922);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2386);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2923);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3244);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3245);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3417);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3480);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3506(s, out),
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
				['ν', s @ ..] => {
					super::push(out, 2387);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2924);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3246);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3418);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3481);
													s.is_empty()
												}
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
					super::push(out, 2388);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2925);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3247);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2926);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3248);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3419);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3482);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3507(s, out),
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
					super::push(out, 2389);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2927);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3249);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 3420);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3483);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 3421);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3484);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3508(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3422);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3485);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3509(s, out),
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
							super::push(out, 2928);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3250);
									s.is_empty()
								}
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
			super::push(out, 1665);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2390);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2929);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3251);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2391);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2930);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3252);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2392);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2931);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3253);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2393);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2932);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3254);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3423);
											s.is_empty()
										}
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
			super::push(out, 1666);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2394);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2933);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2934);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3255);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3424);
											s.is_empty()
										}
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
					super::push(out, 2395);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2935);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3256);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3425);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2936);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2396);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2937);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1667);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 2397);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2938);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t1::n2399(s, out),
				['σ', s @ ..] => {
					super::push(out, 2399);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υτησω.
pub(crate) fn n878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 878);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1668);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2400);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2940);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3259);
									s.is_empty()
								}
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
			super::push(out, 1669);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2401);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2941);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3260);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3427);
											s.is_empty()
										}
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
			super::push(out, 1670);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2402);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωτησω.
pub(crate) fn n879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 879);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1671);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2403);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνναυμαχησω.
pub(crate) fn n3491(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3487);
	s.is_empty()
}

/// Words ending -ρατοπεδευσω.
pub(crate) fn n3492(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3488);
	s.is_empty()
}

/// Words ending -μπλευσω.
pub(crate) fn n2474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2473);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 2988);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3286);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υλευσω.
pub(crate) fn n1765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1765);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2474);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2475);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2990);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3288);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3438);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3489);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 3289);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3439);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3490);
											s.is_empty()
										}
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
					super::push(out, 2991);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωλευσω.
pub(crate) fn n1766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1766);
	match s {
		[] => true,
		['φ', s @ ..] => {
			super::push(out, 2476);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μευσω.
pub(crate) fn n960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 960);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1767);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νευσω.
pub(crate) fn n961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 961);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2478);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1769);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2479);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2992);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1770);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2480);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1771);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2481);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1772);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2993);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3290);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3440);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3491);
													s.is_empty()
												}
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
							super::push(out, 2994);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3291);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3441);
											s.is_empty()
										}
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
					super::push(out, 2483);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2995);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1773);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2484);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2996);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3292);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2485);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2997);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3293);
									s.is_empty()
								}
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
			super::push(out, 1774);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1775);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2487);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2998);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3294);
									match s {
										[] => true,
										['κ', 'γ', s @ ..] => {
											super::push(out, 3442);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3492);
													match s {
														[] => true,
														['ξ', '\u{1}', s @ ..] => super::t1::n3510(s, out),
														['σ', s @ ..] => super::t1::n3511(s, out),
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

/// Words ending -ξευσω.
pub(crate) fn n962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 962);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1776);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1777);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2489);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πευσω.
pub(crate) fn n963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 963);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1778);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2490);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2999);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1779);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2491);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3000);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρευσω.
pub(crate) fn n964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 964);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1782);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2494);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3001);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3295);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1783);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1784);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1785);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2496);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1786);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2497);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3002);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3296);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1787);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2499);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3003);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3297);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3004);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3298);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3443);
											s.is_empty()
										}
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
			super::push(out, 1788);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2500);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3005);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3299);
									s.is_empty()
								}
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
			super::push(out, 1789);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -σευσω.
pub(crate) fn n965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 965);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1790);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τευσω.
pub(crate) fn n966(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 966);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2503);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3007);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3300);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 3008);
							match s {
								[] => true,
								['σ', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 1792);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2504);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3009);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2505);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3010);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3302);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3444);
											s.is_empty()
										}
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
			super::push(out, 1793);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2506);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3011);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3303);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2507);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3012);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3304);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3445);
											s.is_empty()
										}
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
			super::push(out, 1794);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2508);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3013);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		['κ', s @ ..] => {
			super::push(out, 1795);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2509);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3014);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1796);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2510);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3015);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3306);
									s.is_empty()
								}
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
			super::push(out, 1797);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2511);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3016);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3307);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2512);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3017);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3018);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3308);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3446);
											s.is_empty()
										}
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
			super::push(out, 1798);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2513);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1799);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2514);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3019);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3309);
									s.is_empty()
								}
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

/// Words ending -φευσω.
pub(crate) fn n967(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 967);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1800);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χευσω.
pub(crate) fn n968(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 968);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1801);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2516);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3020);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3310);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3447);
											s.is_empty()
										}
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
		['γ', s @ ..] => {
			super::push(out, 1802);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1803);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2518);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψευσω.
pub(crate) fn n969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 969);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1804);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ροκαταγνωσω.
pub(crate) fn n3500(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3496);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3508);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κατασκηνωσω.
pub(crate) fn n3501(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3497);
	s.is_empty()
}

/// The word επαναχωρησω.
pub(crate) fn n3504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3500);
	s.is_empty()
}

/// The word αναβλαστησω.
pub(crate) fn n3506(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3502);
	s.is_empty()
}

/// Words ending -παιδεραστησω.
pub(crate) fn n3507(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3503);
	s.is_empty()
}

/// Words ending -ντικαταστησω.
pub(crate) fn n3508(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3504);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3510);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αποκαταστησω.
pub(crate) fn n3509(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3505);
	s.is_empty()
}

/// Words ending -ουστησω.
pub(crate) fn n2399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2398);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2939);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3258);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3426);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3486);
									s.is_empty()
								}
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

/// The word ξυγκινδυνευσω.
pub(crate) fn n3510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3506);
	s.is_empty()
}

/// Words ending -συγκινδυνευσω.
pub(crate) fn n3511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3507);
	s.is_empty()
}

