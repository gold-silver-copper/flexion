use flexion::Path;

/// Words ending -σμα.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 880);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2466);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2467);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 4426);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 4427);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 4428);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2468);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4429);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5774);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 4430);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4431);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 5775);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 4432);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2469);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2470);
					match s {
						[] => true,
						['ε', 'π', 'μ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 4433);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2472);
					match s {
						[] => true,
						['υ', '\u{1}', s @ ..] => {
							super::push(out, 4434);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2473);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 881);
			match s {
				[] => true,
				['λ', 'ε', 'τ', 'ο', s @ ..] => {
					super::push(out, 2475);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 882);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 883);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2476);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2478);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2479);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2480);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4435);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4436);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4437);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4438);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4439);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2481);
					match s {
						[] => true,
						['α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 4440);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4441);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 4442);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5776);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 5777);
									s.is_empty()
								}
								['γ', s @ ..] => {
									super::push(out, 5778);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5779);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2482);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4443);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2483);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2484);
					match s {
						[] => true,
						['ι', 'ε', 'τ', 'ι', s @ ..] => {
							super::push(out, 4444);
							match s {
								[] => true,
								['τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 5780);
									s.is_empty()
								}
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
			super::push(out, 884);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 885);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2485);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2486);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 2487);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2488);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2489);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2490);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2491);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2492);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2493);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2494);
					match s {
						[] => true,
						['α', 'λ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4445);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 2495);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμα.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 886);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 887);
			match s {
				[] => true,
				['ν', 'ε', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4446);
							s.is_empty()
						}
						['π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4447);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 888);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2496);
					match s {
						[] => true,
						['η', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4448);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2497);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2498);
					match s {
						[] => true,
						['υ', 'π', 'μ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4449);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2499);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4450);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2500);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2501);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2502);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2503);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 890);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 891);
			match s {
				[] => true,
				['δ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 2505);
					s.is_empty()
				}
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 2506);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 892);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 893);
			match s {
				[] => true,
				['ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2507);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 894);
			match s {
				[] => true,
				['γ', 'ε', 'ρ', s @ ..] => {
					super::push(out, 2508);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χμα.
pub(crate) fn n177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 177);
	match s {
		[] => true,
		['α', 'ρ', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωμα.
pub(crate) fn n178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 178);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 896);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 897);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 898);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2509);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2510);
					match s {
						[] => true,
						['λ', 'α', 'χ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 4452);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 899);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2511);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 900);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2513);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2514);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2515);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 2516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 901);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 902);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2517);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 903);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 904);
			match s {
				[] => true,
				['ι', 'κ', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 905);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 906);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2521);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5781);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5782);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5783);
									s.is_empty()
								}
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
			super::push(out, 907);
			match s {
				[] => true,
				['π', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4453);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 4454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 908);
			match s {
				[] => true,
				['α', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τειρα.
pub(crate) fn n2622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2574);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4471);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 4472);
			s.is_empty()
		}
		['υ', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 4473);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ορα.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 965);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 966);
			match s {
				[] => true,
				['φ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2575);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 967);
			match s {
				[] => true,
				['σ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2576);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 968);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2577);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4474);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4475);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2578);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2579);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2580);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 969);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πρα.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['ι', 'π', '\u{1}', s @ ..] => {
			super::push(out, 970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρρα.
pub(crate) fn n205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 205);
	match s {
		[] => true,
		['α', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 971);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τρα.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 972);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2581);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2582);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 974);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 975);
			match s {
				[] => true,
				['λ', '\u{1}', s @ ..] => {
					super::push(out, 2583);
					s.is_empty()
				}
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 2584);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 976);
			match s {
				[] => true,
				['α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2585);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 977);
			match s {
				[] => true,
				['υ', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2586);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 978);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2587);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2588);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2589);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2590);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2591);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2592);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['λ', 'λ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2593);
					s.is_empty()
				}
				['χ', '\u{1}', s @ ..] => {
					super::push(out, 2594);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υρα.
pub(crate) fn n207(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 207);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 980);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 981);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 982);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 983);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 984);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 985);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2595);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2596);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2597);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 986);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2599);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2600);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 4478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2601);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 988);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2602);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 989);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 990);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2604);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2605);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2606);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2607);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χρα.
pub(crate) fn n209(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 209);
	s.is_empty()
}

/// Words ending -ωρα.
pub(crate) fn n210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 210);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 991);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κοθηκη.
pub(crate) fn n4601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4491);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5800);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λοθηκη.
pub(crate) fn n4602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4492);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5801);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 6365);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 5802);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 5803);
			match s {
				[] => true,
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 6366);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μοθηκη.
pub(crate) fn n4603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4493);
	s.is_empty()
}

/// Words ending -νοθηκη.
pub(crate) fn n4604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4494);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5804);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 5805);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 5806);
			match s {
				[] => true,
				['μ', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 6367);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 5807);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ποθηκη.
pub(crate) fn n4605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4495);
	s.is_empty()
}

/// Words ending -ροθηκη.
pub(crate) fn n4606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4496);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 5808);
			match s {
				[] => true,
				['δ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 6368);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 5809);
			match s {
				[] => true,
				['α', 'χ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 6369);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 5810);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 5811);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 6370);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τοθηκη.
pub(crate) fn n4607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4497);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5812);
			match s {
				[] => true,
				['μ', s @ ..] => {
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6532);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 6533);
							match s {
								[] => true,
								['υ', 'δ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 6590);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 6534);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 5813);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 5814);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 5815);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6535);
							s.is_empty()
						}
						['χ', '\u{1}', s @ ..] => {
							super::push(out, 6536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 6372);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 5816);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φοθηκη.
pub(crate) fn n4608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4498);
	match s {
		[] => true,
		['ι', 'ξ', '\u{1}', s @ ..] => {
			super::push(out, 5817);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υθηκη.
pub(crate) fn n2697(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2647);
	s.is_empty()
}

/// Words ending -ικη.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1048);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1049);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2648);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1051);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1052);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2650);
					s.is_empty()
				}
				['ο', 'τ', 'κ', s @ ..] => {
					super::push(out, 2651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1053);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1054);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2652);
					match s {
						[] => true,
						['ε', 'γ', s @ ..] => {
							super::push(out, 4499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2653);
					match s {
						[] => true,
						['μ', 'ο', 'ι', 'ρ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4500);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1055);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2654);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2655);
					match s {
						[] => true,
						['ρ', 'ι', 'α', 'φ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4501);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2656);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2657);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4502);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4503);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'α', 'μ', 'ο', 'ν', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 2658);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2659);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1056);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κκη.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	s.is_empty()
}

/// Words ending -λκη.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['α', s @ ..] => {
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2660);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2661);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οκη.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	s.is_empty()
}

/// Words ending -ρκη.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['α', s @ ..] => {
			match s {
				[] => true,
				['δ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2662);
					s.is_empty()
				}
				['φ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2663);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σκη.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	s.is_empty()
}

/// Words ending -υκη.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 276);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1057);
			match s {
				[] => true,
				['μ', 'α', s @ ..] => {
					match s {
						[] => true,
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 4505);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 4506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1058);
			match s {
				[] => true,
				['λ', 'ι', s @ ..] => {
					super::push(out, 2664);
					s.is_empty()
				}
				['π', 'ι', s @ ..] => {
					super::push(out, 2665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1059);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1060);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1061);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1062);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωκη.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	s.is_empty()
}

/// Words ending -οσυνη.
pub(crate) fn n2772(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2718);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4524);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 4525);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 4526);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5822);
					match s {
						[] => true,
						['α', 'ρ', 'π', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 6374);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 5823);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 4527);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 4528);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5824);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 5825);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6375);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6376);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6377);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 4529);
			match s {
				[] => true,
				['ν', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 5826);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φνη.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1160);
			match s {
				[] => true,
				['δ', 'ι', s @ ..] => {
					super::push(out, 2720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1161);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χνη.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['α', 'ρ', 'α', s @ ..] => {
			super::push(out, 1162);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωνη.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 1163);
			match s {
				[] => true,
				['υ', 'ρ', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2721);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιον.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1293);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1294);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2803);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2804);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4558);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4559);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5836);
									s.is_empty()
								}
								['ρ', 'α', s @ ..] => {
									super::push(out, 5837);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4560);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2805);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4561);
							s.is_empty()
						}
						['ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 4562);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2806);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2807);
					match s {
						[] => true,
						['ε', 'δ', 'ι', s @ ..] => {
							super::push(out, 4563);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4564);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2809);
					match s {
						[] => true,
						['ρ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4565);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2810);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2811);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2812);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2813);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4566);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4567);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4568);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2814);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4569);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4570);
							match s {
								[] => true,
								['π', 'σ', 'ο', s @ ..] => {
									super::push(out, 5838);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 4571);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2815);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2816);
					match s {
						[] => true,
						['λ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4572);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2817);
					match s {
						[] => true,
						['λ', 'ο', 'ν', s @ ..] => {
							super::push(out, 4573);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2818);
					match s {
						[] => true,
						['ζ', 'α', s @ ..] => {
							super::push(out, 4574);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1297);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2819);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4576);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 4577);
							match s {
								[] => true,
								['ε', '\u{1}', s @ ..] => {
									super::push(out, 5839);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 4578);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4579);
							match s {
								[] => true,
								['σ', 'α', 'κ', 'ε', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 5840);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ῥ', s @ ..] => {
							super::push(out, 4580);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2820);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2821);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2822);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4581);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 4582);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 4583);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 5841);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5842);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 5843);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 4584);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5844);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 5845);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 5846);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 4585);
							match s {
								[] => true,
								['α', 'ν', 'ι', 'π', '\u{1}', s @ ..] => {
									super::push(out, 5847);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 4586);
							match s {
								[] => true,
								['α', 'ρ', s @ ..] => {
									match s {
										[] => true,
										['β', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6381);
											s.is_empty()
										}
										['ε', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 6382);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 4587);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4588);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4589);
							match s {
								[] => true,
								['τ', 'υ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 5848);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 4590);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 4591);
							match s {
								[] => true,
								['ν', 'α', 'λ', 'α', 'β', '\u{1}', s @ ..] => {
									super::push(out, 5849);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 4592);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2823);
					match s {
						[] => true,
						['π', s @ ..] => {
							match s {
								[] => true,
								['ο', s @ ..] => super::t1::n6010(s, out),
								['υ', s @ ..] => {
									super::push(out, 5851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2824);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2825);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2826);
					match s {
						[] => true,
						['γ', s @ ..] => super::t1::n4706(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n1324(s, out),
		['ζ', s @ ..] => super::t1::n1325(s, out),
		['η', s @ ..] => super::t1::n1326(s, out),
		['θ', s @ ..] => super::t1::n1327(s, out),
		['κ', s @ ..] => super::t1::n1328(s, out),
		['λ', s @ ..] => super::t1::n1329(s, out),
		['μ', s @ ..] => super::t1::n1330(s, out),
		['ν', s @ ..] => super::t1::n1331(s, out),
		['ξ', s @ ..] => super::t1::n1332(s, out),
		['ο', s @ ..] => super::t1::n1333(s, out),
		['π', s @ ..] => super::t1::n1334(s, out),
		['ρ', s @ ..] => super::t1::n1335(s, out),
		['σ', s @ ..] => super::t1::n1336(s, out),
		['τ', s @ ..] => super::t1::n1337(s, out),
		['υ', 'γ', '\u{1}', s @ ..] => super::t1::n1338(s, out),
		['φ', s @ ..] => super::t1::n1339(s, out),
		['χ', s @ ..] => super::t1::n1340(s, out),
		['ῥ', s @ ..] => super::t1::n1341(s, out),
		_ => false,
	}
}

/// Words ending -λον.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1322);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2939);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 2940);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2941);
					match s {
						[] => true,
						['ν', 'α', 'κ', s @ ..] => {
							super::push(out, 4751);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4752);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2942);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2943);
					match s {
						[] => true,
						['α', 'ψ', '\u{1}', s @ ..] => {
							super::push(out, 4753);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2944);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2945);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2946);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2947);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4754);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5933);
									s.is_empty()
								}
								['π', 'ι', s @ ..] => {
									super::push(out, 5934);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 4755);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4756);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 2948);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2949);
					match s {
						[] => true,
						['ε', 'κ', 'ο', 'ν', s @ ..] => {
							super::push(out, 4757);
							s.is_empty()
						}
						['μ', 'ο', 'κ', s @ ..] => {
							super::push(out, 4758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1323);
			match s {
				[] => true,
				['μ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4759);
							match s {
								[] => true,
								['α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 5935);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4760);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4761);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1324);
			match s {
				[] => true,
				['α', 'τ', s @ ..] => {
					super::push(out, 2950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1325);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2951);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 2952);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1326);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2953);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1327);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2954);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4762);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2955);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2956);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2957);
					match s {
						[] => true,
						['φ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5936);
									match s {
										[] => true,
										['τ', 'ν', s @ ..] => {
											match s {
												[] => true,
												['α', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
													super::push(out, 6549);
													s.is_empty()
												}
												['ε', 'π', '\u{1}', s @ ..] => {
													super::push(out, 6550);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 5937);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5938);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5939);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6409);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 6551);
													s.is_empty()
												}
												['ρ', s @ ..] => {
													super::push(out, 6552);
													match s {
														[] => true,
														['υ', 'μ', '\u{1}', s @ ..] => super::t1::n6786(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 6410);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 6411);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 6412);
											s.is_empty()
										}
										['σ', s @ ..] => {
											super::push(out, 6413);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 6414);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1328);
			match s {
				[] => true,
				['β', 'μ', 'υ', s @ ..] => {
					super::push(out, 2958);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1329);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1330);
			match s {
				[] => true,
				['υ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4763);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 4764);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 5940);
									s.is_empty()
								}
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
			super::push(out, 1331);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2959);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2960);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2961);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2962);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 2963);
					match s {
						[] => true,
						['ο', 'ι', 'ρ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4765);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2964);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2965);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 4766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2966);
					match s {
						[] => true,
						['κ', 'α', 'δ', 'ο', 'μ', 'ρ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 4767);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 2967);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4768);
							s.is_empty()
						}
						['μ', 'ε', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 4769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1332);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μον.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1333);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2968);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2969);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2970);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2971);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2972);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4770);
							s.is_empty()
						}
						['λ', 'α', 'β', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5941);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5942);
									s.is_empty()
								}
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
			super::push(out, 1334);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2973);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2974);
					match s {
						[] => true,
						['ν', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5943);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 5944);
									s.is_empty()
								}
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
			super::push(out, 1335);
			match s {
				[] => true,
				['ν', 'κ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2975);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1336);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1337);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2976);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2977);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1338);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1339);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2978);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2979);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2980);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1340);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1341);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2981);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2982);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1342);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2983);
					match s {
						[] => true,
						['ω', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 5945);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 5946);
									s.is_empty()
								}
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
			super::push(out, 1343);
			match s {
				[] => true,
				['μ', 'α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4771);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4772);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νον.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1344);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2984);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4773);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4774);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2985);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4775);
							match s {
								[] => true,
								['λ', 'ο', 'τ', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 5947);
									s.is_empty()
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 4776);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 4777);
							match s {
								[] => true,
								['π', 'ο', 'ι', 'ρ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 5948);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 4778);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4779);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5949);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5950);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 4780);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4781);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 2986);
					match s {
						[] => true,
						['α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 4782);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4783);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4784);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2987);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2988);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 2989);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2990);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2991);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4785);
							match s {
								[] => true,
								['ρ', 'δ', s @ ..] => {
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 6415);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 6416);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 4786);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4787);
							match s {
								[] => true,
								['κ', 'ι', s @ ..] => {
									super::push(out, 5951);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 4788);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 5952);
									match s {
										[] => true,
										['τ', 'ο', 'ρ', 'η', 'δ', 'ι', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 6417);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 5953);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2992);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 4789);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2993);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2994);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2995);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4790);
							match s {
								[] => true,
								['λ', 'ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 5954);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4791);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 2996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1345);
			match s {
				[] => true,
				['ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2998);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2999);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1347);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3001);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 4792);
							s.is_empty()
						}
						['υ', s @ ..] => {
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 5955);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5956);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3002);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1348);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3003);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3004);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3005);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4793);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1349);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3006);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3007);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3008);
					match s {
						[] => true,
						['α', 'ρ', 'ω', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4794);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3009);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 4795);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4796);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4797);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3010);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3011);
					match s {
						[] => true,
						['φ', 'α', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4798);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3012);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3013);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 4799);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5957);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 5958);
									s.is_empty()
								}
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
			super::push(out, 1350);
			match s {
				[] => true,
				['ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3014);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1351);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3015);
					match s {
						[] => true,
						['τ', 'κ', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 4800);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3016);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3017);
					match s {
						[] => true,
						['δ', 'ε', 'μ', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 4801);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1352);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1353);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3018);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 4802);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4803);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3019);
					s.is_empty()
				}
				['τ', s @ ..] => super::t1::n3083(s, out),
				['φ', s @ ..] => {
					super::push(out, 3021);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t1::n1380(s, out),
		['ρ', s @ ..] => super::t1::n1381(s, out),
		['υ', s @ ..] => {
			super::push(out, 1356);
			s.is_empty()
		}
		['χ', s @ ..] => super::t1::n1383(s, out),
		['ω', s @ ..] => super::t1::n1384(s, out),
		_ => false,
	}
}

/// Words ending -πον.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	match s {
		[] => true,
		['ε', 'λ', 'β', 'ω', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 1361);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1362);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1363);
			match s {
				[] => true,
				['α', 'κ', 'υ', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1364);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3030);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3031);
					match s {
						[] => true,
						['ε', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4811);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1365);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3032);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρον.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 432);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1366);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3033);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3034);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4812);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4813);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4814);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3035);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3036);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3037);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3038);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3039);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4815);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 5963);
									s.is_empty()
								}
								['λ', '\u{1}', s @ ..] => {
									super::push(out, 5964);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 4816);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4817);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3040);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3041);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1367);
			match s {
				[] => true,
				['ν', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4818);
							s.is_empty()
						}
						['ε', 'δ', 'ο', 'λ', s @ ..] => {
							super::push(out, 4819);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1368);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3042);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3043);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3044);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 4820);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1369);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3045);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1370);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3046);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 4821);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 5965);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 5966);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 4822);
							match s {
								[] => true,
								['α', 'π', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 5967);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4823);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3047);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 3048);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3049);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3050);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 4825);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3051);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1371);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3052);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4826);
							s.is_empty()
						}
						['π', 'υ', s @ ..] => {
							super::push(out, 4827);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3053);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1372);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1373);
			match s {
				[] => true,
				['κ', 'η', s @ ..] => {
					super::push(out, 3054);
					s.is_empty()
				}
				['φ', 'ο', 'τ', 'α', 'ρ', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3055);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1374);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1375);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3056);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4828);
							match s {
								[] => true,
								['θ', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 5968);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 4829);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3057);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 4830);
							s.is_empty()
						}
						['μ', 'α', s @ ..] => {
							super::push(out, 4831);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3058);
					match s {
						[] => true,
						['ν', 'ι', s @ ..] => {
							super::push(out, 4832);
							match s {
								[] => true,
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 5969);
									s.is_empty()
								}
								['π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 5970);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3059);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 4833);
							s.is_empty()
						}
						['μ', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 4834);
							s.is_empty()
						}
						['ν', '\u{1}', s @ ..] => {
							super::push(out, 4835);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3060);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4836);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4837);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4838);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3061);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3062);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 4839);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4840);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4841);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 4842);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3063);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3064);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4843);
							match s {
								[] => true,
								['κ', 'σ', 'ι', 'σ', 'υ', 'ρ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 5971);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 4844);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 5972);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 4845);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3065);
					match s {
						[] => true,
						['α', s @ ..] => super::t1::n4969(s, out),
						['ε', s @ ..] => {
							super::push(out, 4847);
							s.is_empty()
						}
						['η', s @ ..] => super::t1::n4971(s, out),
						['ι', s @ ..] => super::t1::n4972(s, out),
						['υ', s @ ..] => super::t1::n4973(s, out),
						_ => false,
					}
				}
				['υ', s @ ..] => super::t1::n3131(s, out),
				['ω', s @ ..] => super::t1::n3132(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n1402(s, out),
		['ω', s @ ..] => super::t1::n1403(s, out),
		_ => false,
	}
}

/// Words ending -σον.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1378);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3081);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3082);
					match s {
						[] => true,
						['π', 'ο', 'ι', s @ ..] => {
							super::push(out, 4864);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1379);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1380);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3083);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3084);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4865);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3085);
					s.is_empty()
				}
				['π', '\u{1}', s @ ..] => {
					super::push(out, 3086);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1381);
			match s {
				[] => true,
				['ω', 'λ', 'γ', 'ο', 'ν', 'υ', s @ ..] => {
					super::push(out, 3087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1382);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τον.
pub(crate) fn n435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 434);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1383);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3088);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4866);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 4867);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3089);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3090);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3091);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3092);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1384);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3093);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3094);
					match s {
						[] => true,
						['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4869);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3095);
					match s {
						[] => true,
						['ω', 'ν', 'ι', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1385);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3096);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 3097);
					match s {
						[] => true,
						['ρ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 4871);
							s.is_empty()
						}
						['υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4872);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3098);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3099);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3100);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1386);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1387);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1388);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4873);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 4874);
							match s {
								[] => true,
								['α', 'τ', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 5982);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 4875);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1389);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1390);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1391);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3102);
					match s {
						[] => true,
						['γ', 'ι', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 4876);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1392);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3104);
					match s {
						[] => true,
						['π', 'α', s @ ..] => {
							super::push(out, 4877);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3105);
					match s {
						[] => true,
						['ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 4878);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3106);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1393);
			match s {
				[] => true,
				['ρ', 'β', '\u{1}', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				['φ', s @ ..] => {
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4880);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1394);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υον.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 435);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1395);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1396);
			match s {
				[] => true,
				['μ', 'ο', 'ρ', 'κ', 'ο', 'ι', 'ρ', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3109);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1397);
			match s {
				[] => true,
				['ο', 'κ', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3110);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1398);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				['κ', 'α', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3112);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1399);
			match s {
				[] => true,
				['κ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1400);
			match s {
				[] => true,
				['ι', '\u{1}', s @ ..] => {
					super::push(out, 3114);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υφον.
pub(crate) fn n1429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1403);
	s.is_empty()
}

/// Words ending -χον.
pub(crate) fn n438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 437);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1404);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1405);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1406);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1407);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψον.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 438);
	s.is_empty()
}

/// Words ending -ωον.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['ζ', 'ι', s @ ..] => {
			super::push(out, 1408);
			s.is_empty()
		}
		['ρ', 'ε', 'π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 1409);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ντοκρατωρ.
pub(crate) fn n6748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6556);
	s.is_empty()
}

/// Words ending -τησ.
pub(crate) fn n568(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 567);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1692);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3309);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 4926);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6002);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 6003);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6004);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3310);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3311);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3312);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4927);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 4928);
							match s {
								[] => true,
								['ω', s @ ..] => {
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 6437);
											s.is_empty()
										}
										['λ', s @ ..] => {
											super::push(out, 6438);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 6439);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 4929);
							match s {
								[] => true,
								['ρ', 'α', 'π', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 6005);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3313);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4930);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4931);
							s.is_empty()
						}
						['η', 'ξ', s @ ..] => {
							super::push(out, 4932);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3314);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3315);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3316);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4933);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 4934);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3317);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3318);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 6006);
									s.is_empty()
								}
								['ο', s @ ..] => {
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 6440);
											s.is_empty()
										}
										['π', 'α', s @ ..] => {
											super::push(out, 6441);
											s.is_empty()
										}
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
					super::push(out, 3319);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1693);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3320);
					match s {
						[] => true,
						['η', 'ν', s @ ..] => {
							super::push(out, 4935);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3321);
					s.is_empty()
				}
				['κ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 3322);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3323);
					match s {
						[] => true,
						['ο', 'δ', 'ν', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 4936);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3324);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3325);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3326);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['γ', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 6007);
									s.is_empty()
								}
								['ρ', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 6008);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3327);
					match s {
						[] => true,
						['η', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 4937);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4938);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1694);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3328);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 3329);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3330);
					match s {
						[] => true,
						['α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 4939);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3331);
					match s {
						[] => true,
						['ι', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 6009);
									s.is_empty()
								}
								['ο', 'ι', s @ ..] => {
									super::push(out, 6010);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3332);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 4940);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4941);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3333);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4942);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3334);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 4943);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 4944);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 4945);
							s.is_empty()
						}
						['ρ', 'ε', 'β', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 4946);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3335);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 4947);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 4948);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 4949);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3336);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n1725(s, out),
		['κ', s @ ..] => {
			super::push(out, 1696);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3349);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 4969);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 4970);
							match s {
								[] => true,
								['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6020);
									s.is_empty()
								}
								['ρ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6021);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3350);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 4971);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3351);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3352);
					match s {
						[] => true,
						['ρ', 'ο', 'τ', 'α', 'ε', 'ρ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 4972);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1697);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3353);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 4973);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 4974);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 4975);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => super::t1::n3429(s, out),
				_ => false,
			}
		}
		['ν', s @ ..] => super::t1::n1728(s, out),
		['ο', s @ ..] => super::t1::n1729(s, out),
		['π', s @ ..] => super::t1::n1730(s, out),
		['ρ', s @ ..] => super::t1::n1731(s, out),
		['σ', s @ ..] => super::t1::n1732(s, out),
		['υ', s @ ..] => super::t1::n1733(s, out),
		['ω', s @ ..] => super::t1::n1734(s, out),
		_ => false,
	}
}

/// Words ending -υλισ.
pub(crate) fn n1778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1747);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3428);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 3429);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 3430);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3431);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 3432);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 3433);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μισ.
pub(crate) fn n582(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 581);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1748);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3434);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3435);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3436);
					match s {
						[] => true,
						['υ', 'δ', 'ο', 'ι', 'λ', 'ι', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 5065);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3437);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5066);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3438);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1749);
			match s {
				[] => true,
				['θ', 'ν', s @ ..] => {
					super::push(out, 3439);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6061);
									s.is_empty()
								}
								['ρ', 'ω', 'ψ', '\u{1}', s @ ..] => {
									super::push(out, 6062);
									s.is_empty()
								}
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
		['θ', s @ ..] => {
			super::push(out, 1751);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1752);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1753);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3440);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1754);
			match s {
				[] => true,
				['ε', 'δ', s @ ..] => {
					super::push(out, 3441);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1755);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1756);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νισ.
pub(crate) fn n583(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 582);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1757);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3442);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3443);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3444);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3445);
					match s {
						[] => true,
						['υ', 'ρ', 'π', 'ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3446);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1758);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1759);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3447);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1760);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1761);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3448);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1762);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1763);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 1764);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1765);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3450);
					s.is_empty()
				}
				['ρ', 'α', s @ ..] => {
					super::push(out, 3451);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξισ.
pub(crate) fn n584(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 583);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1766);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3452);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5068);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5069);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5070);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3453);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3454);
					match s {
						[] => true,
						['α', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 5071);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3455);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5072);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6063);
									match s {
										[] => true,
										['δ', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6453);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 6064);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6065);
									match s {
										[] => true,
										['α', 'π', s @ ..] => {
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 6564);
													s.is_empty()
												}
												['μ', s @ ..] => {
													super::push(out, 6565);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 6066);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 6454);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 6455);
											match s {
												[] => true,
												['μ', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
													super::push(out, 6566);
													s.is_empty()
												}
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
							super::push(out, 5073);
							match s {
								[] => true,
								['τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 6067);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 5074);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5075);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6068);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 6456);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 6069);
									match s {
										[] => true,
										['σ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6457);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5076);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 6070);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6071);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5077);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5078);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6072);
									s.is_empty()
								}
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
			super::push(out, 1767);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3456);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5079);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3457);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5080);
							s.is_empty()
						}
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
		['η', s @ ..] => {
			super::push(out, 1768);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 3458);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1769);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3459);
					match s {
						[] => true,
						['δ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6073);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6074);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3460);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1770);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1771);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1772);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1773);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3461);
					match s {
						[] => true,
						['τ', 'ν', 'ε', 'ν', s @ ..] => {
							super::push(out, 5082);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3462);
					s.is_empty()
				}
				['τ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3463);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3464);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1774);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οισ.
pub(crate) fn n585(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 584);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1775);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1776);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πισ.
pub(crate) fn n586(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 585);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1777);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1778);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1779);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1780);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3465);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1781);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3466);
					match s {
						[] => true,
						['λ', 'ο', 'γ', 'υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 5083);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3467);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1782);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3468);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1783);
			match s {
				[] => true,
				['ο', 'δ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3469);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1784);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3470);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5084);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5085);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5086);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3471);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρισ.
pub(crate) fn n587(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 586);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1785);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3472);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 3473);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5087);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6075);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3474);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3475);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3476);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3477);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5088);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5089);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5090);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3478);
					match s {
						[] => true,
						['α', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 5091);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3479);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3480);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3481);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3482);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3483);
					match s {
						[] => true,
						['α', 'ζ', '\u{1}', s @ ..] => {
							super::push(out, 5092);
							s.is_empty()
						}
						['κ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 5093);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1786);
			match s {
				[] => true,
				['υ', '\u{1}', s @ ..] => {
					super::push(out, 3484);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1787);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1788);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3485);
					s.is_empty()
				}
				['υ', 'ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3486);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1789);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3487);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3488);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5094);
							s.is_empty()
						}
						['π', s @ ..] => {
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 6076);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 6077);
									s.is_empty()
								}
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
			super::push(out, 1790);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3489);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1791);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3490);
					match s {
						[] => true,
						['α', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5095);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1792);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1793);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1794);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1795);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3493);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3494);
					match s {
						[] => true,
						['α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 5096);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3495);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5097);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3496);
					match s {
						[] => true,
						['π', 'π', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 5098);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3497);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5099);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1796);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3499);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3500);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σισ.
pub(crate) fn n588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 587);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1797);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3501);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 3502);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5100);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 5101);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5102);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3503);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5103);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5104);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5105);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6078);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 6079);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 6080);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 6081);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3504);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5107);
							s.is_empty()
						}
						['κ', 'α', s @ ..] => {
							super::push(out, 5108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', 'ν', 'ε', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 3505);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3506);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3507);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3508);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5109);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 5110);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5111);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5112);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 5113);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6083);
									match s {
										[] => true,
										['ρ', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 6458);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3509);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5114);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5115);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5116);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5117);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6085);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 6086);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 6087);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5118);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5119);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6088);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6459);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 6460);
											match s {
												[] => true,
												['α', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n6787(s, out),
														['τ', 'ε', 'μ', '\u{1}', s @ ..] => super::t1::n6788(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 6461);
											match s {
												[] => true,
												['α', 'κ', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n6789(s, out),
														['ο', s @ ..] => super::t1::n6790(s, out),
														_ => false,
													}
												}
												['ε', s @ ..] => {
													super::push(out, 6567);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 6089);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 6090);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6091);
									match s {
										[] => true,
										['π', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6462);
											s.is_empty()
										}
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
					super::push(out, 3510);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1798);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3511);
					match s {
						[] => true,
						['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 5120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3512);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5121);
							match s {
								[] => true,
								['ι', 'δ', '\u{1}', s @ ..] => {
									super::push(out, 6092);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5122);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5123);
							match s {
								[] => true,
								['ε', 'ρ', s @ ..] => {
									super::push(out, 6093);
									s.is_empty()
								}
								['υ', s @ ..] => super::t1::n6263(s, out),
								_ => false,
							}
						}
						['ο', s @ ..] => super::t1::n5258(s, out),
						_ => false,
					}
				}
				['μ', s @ ..] => super::t1::n3588(s, out),
				['ν', s @ ..] => super::t1::n3589(s, out),
				['ρ', s @ ..] => super::t1::n3590(s, out),
				['φ', s @ ..] => super::t1::n3591(s, out),
				['χ', s @ ..] => super::t1::n3592(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t1::n1830(s, out),
		['ι', s @ ..] => super::t1::n1831(s, out),
		['λ', s @ ..] => super::t1::n1832(s, out),
		['ν', s @ ..] => super::t1::n1833(s, out),
		['ο', s @ ..] => super::t1::n1834(s, out),
		['ρ', s @ ..] => super::t1::n1835(s, out),
		['σ', s @ ..] => super::t1::n1836(s, out),
		['υ', s @ ..] => super::t1::n1837(s, out),
		['ω', s @ ..] => super::t1::n1838(s, out),
		_ => false,
	}
}

/// Words ending -τισ.
pub(crate) fn n589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 588);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1808);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3564);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3565);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3566);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3567);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3568);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3569);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3570);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1809);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3571);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3572);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3573);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1810);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3574);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3575);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3576);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5214);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5215);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3577);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3578);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5216);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5217);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5218);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6141);
									match s {
										[] => true,
										['μ', 'η', '\u{1}', s @ ..] => {
											super::push(out, 6469);
											s.is_empty()
										}
										['ρ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 6470);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 5219);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 5220);
							match s {
								[] => true,
								['υ', 'λ', '\u{1}', s @ ..] => {
									super::push(out, 6142);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 5221);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3579);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5222);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6471);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 6472);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5223);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3580);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5224);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 5225);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5226);
							match s {
								[] => true,
								['γ', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 6143);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 5227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3581);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3582);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1811);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3584);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5228);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5229);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5230);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3585);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3586);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3587);
					match s {
						[] => true,
						['ν', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 5231);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1812);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3588);
					match s {
						[] => true,
						['μ', 'ο', 'υ', s @ ..] => {
							super::push(out, 5232);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3589);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1813);
			match s {
				[] => true,
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3590);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1814);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3591);
					match s {
						[] => true,
						['λ', 'λ', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5233);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3592);
					match s {
						[] => true,
						['ν', 'κ', s @ ..] => {
							super::push(out, 5234);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3593);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3594);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 5236);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3595);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 5237);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5238);
							match s {
								[] => true,
								['γ', 'α', 'μ', 'α', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6144);
									s.is_empty()
								}
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
			super::push(out, 1815);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3596);
					s.is_empty()
				}
				['ω', 'λ', 'γ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3597);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1816);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1817);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3598);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3599);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3600);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3601);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3602);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φισ.
pub(crate) fn n590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 589);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1818);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3603);
					match s {
						[] => true,
						['γ', 'ο', s @ ..] => {
							super::push(out, 5239);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3604);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 3605);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1819);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1820);
			match s {
				[] => true,
				['ρ', 'τ', 'σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3606);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1821);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χισ.
pub(crate) fn n591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 590);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1822);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 1823);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1824);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1825);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1826);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1827);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['ν', 'ν', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1829);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ψισ.
pub(crate) fn n592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 591);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1830);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3608);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3609);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1831);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1832);
			match s {
				[] => true,
				['λ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5240);
							match s {
								[] => true,
								['τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6145);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5241);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5242);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5243);
							match s {
								[] => true,
								['π', 'υ', '\u{1}', s @ ..] => {
									super::push(out, 6146);
									s.is_empty()
								}
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
			super::push(out, 1833);
			match s {
				[] => true,
				['ε', 'λ', 'α', s @ ..] => {
					super::push(out, 3610);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3611);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1834);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1835);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1836);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3612);
					s.is_empty()
				}
				['π', 'α', s @ ..] => {
					super::push(out, 3613);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1837);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3614);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3615);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωισ.
pub(crate) fn n593(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 592);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			match s {
				[] => true,
				['ε', 'χ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3617);
					s.is_empty()
				}
				['η', '\u{1}', s @ ..] => {
					super::push(out, 3618);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ῥισ.
pub(crate) fn n595(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 594);
	s.is_empty()
}

/// Words ending -νηγοσ.
pub(crate) fn n3718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3640);
	s.is_empty()
}

/// Words ending -τηγοσ.
pub(crate) fn n3719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3641);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5259);
			match s {
				[] => true,
				['ρ', 'τ', 'σ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
					super::push(out, 6153);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χηγοσ.
pub(crate) fn n3720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3642);
	s.is_empty()
}

/// Words ending -ιγοσ.
pub(crate) fn n1884(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1852);
	match s {
		[] => true,
		['α', 'τ', 'α', 'ρ', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 3643);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λγοσ.
pub(crate) fn n1885(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1853);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3644);
			s.is_empty()
		}
		['ο', 'μ', 'α', s @ ..] => {
			super::push(out, 3645);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ογοσ.
pub(crate) fn n1886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1854);
	match s {
		[] => true,
		['λ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5260);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 6154);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6155);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5261);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5262);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5263);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 6157);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 6158);
							match s {
								[] => true,
								['υ', 'τ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 6473);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 6159);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 6160);
							match s {
								[] => true,
								['ι', 'ν', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6474);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 6161);
							match s {
								[] => true,
								['α', 'ρ', 'ε', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 6475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ργοσ.
pub(crate) fn n1887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1855);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3646);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5264);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 5265);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3647);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3648);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 5266);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6162);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 6163);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6164);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 6165);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 6166);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 6167);
							match s {
								[] => true,
								['ι', 'π', 'α', 'λ', 'ι', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 6476);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 6168);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6477);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 6478);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 6479);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 6169);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6170);
							match s {
								[] => true,
								['α', 'μ', s @ ..] => {
									match s {
										[] => true,
										['α', 'ρ', 'δ', '\u{1}', s @ ..] => {
											super::push(out, 6573);
											s.is_empty()
										}
										['υ', 'α', 'θ', '\u{1}', s @ ..] => {
											super::push(out, 6574);
											s.is_empty()
										}
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
					super::push(out, 5267);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σγοσ.
pub(crate) fn n1888(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1856);
	s.is_empty()
}

/// Words ending -υγοσ.
pub(crate) fn n1889(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1857);
	match s {
		[] => true,
		['ε', 'ζ', '\u{1}', s @ ..] => {
			super::push(out, 3649);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωγοσ.
pub(crate) fn n1890(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1858);
	match s {
		[] => true,
		['γ', 'α', s @ ..] => {
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5268);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5269);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5270);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δοσ.
pub(crate) fn n599(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 598);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1859);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3650);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1860);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3653);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 5271);
							s.is_empty()
						}
						['ο', 'ῥ', '\u{1}', s @ ..] => {
							super::push(out, 5272);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3654);
					match s {
						[] => true,
						['λ', 'ο', 'μ', 'ο', 'κ', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 5273);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1861);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3655);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3656);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3657);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1862);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1863);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1864);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3658);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3659);
					s.is_empty()
				}
				['ο', 'α', s @ ..] => {
					super::push(out, 3660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1865);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1866);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3661);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5274);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3662);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3663);
					match s {
						[] => true,
						['υ', s @ ..] => {
							match s {
								[] => true,
								['ξ', '\u{1}', s @ ..] => {
									super::push(out, 6171);
									s.is_empty()
								}
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 6172);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3664);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3665);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1867);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3666);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3667);
					match s {
						[] => true,
						['χ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1868);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3668);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3669);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1869);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3670);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3671);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3672);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 3673);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εοσ.
pub(crate) fn n600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 599);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1870);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3674);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1871);
			match s {
				[] => true,
				['ι', 'ρ', 'ε', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5276);
							s.is_empty()
						}
						['π', '\u{1}', s @ ..] => {
							super::push(out, 5277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1872);
			match s {
				[] => true,
				['α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 3675);
					s.is_empty()
				}
				['υ', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1873);
			match s {
				[] => true,
				['υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 3677);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζοσ.
pub(crate) fn n601(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 600);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1874);
			s.is_empty()
		}
		['ι', 'π', s @ ..] => {
			super::push(out, 1875);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θοσ.
pub(crate) fn n602(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 601);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1876);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3678);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3679);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3680);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3681);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5278);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3682);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6173);
									s.is_empty()
								}
								['ψ', '\u{1}', s @ ..] => {
									super::push(out, 6174);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3683);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3684);
					match s {
						[] => true,
						['α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 5281);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1877);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1878);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1879);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3687);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3688);
					match s {
						[] => true,
						['ο', s @ ..] => {
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6175);
									s.is_empty()
								}
								['σ', 'υ', s @ ..] => {
									super::push(out, 6176);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3689);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1880);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1881);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3690);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5282);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5283);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6177);
									s.is_empty()
								}
								['α', 'υ', s @ ..] => {
									super::push(out, 6178);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 5284);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3691);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3692);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5285);
							s.is_empty()
						}
						['μ', 'α', s @ ..] => {
							super::push(out, 5286);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5287);
							match s {
								[] => true,
								['η', 'μ', s @ ..] => {
									super::push(out, 6179);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 6180);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3693);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3694);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1882);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1883);
			match s {
				[] => true,
				['α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3695);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1884);
			match s {
				[] => true,
				['ι', 'κ', s @ ..] => {
					super::push(out, 3696);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3697);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1885);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1886);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3699);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3700);
					match s {
						[] => true,
						['κ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5288);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3701);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3702);
					match s {
						[] => true,
						['ι', 'ψ', '\u{1}', s @ ..] => {
							super::push(out, 5289);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3703);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3704);
					s.is_empty()
				}
				['ψ', s @ ..] => {
					super::push(out, 3705);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1887);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3706);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιοσ.
pub(crate) fn n603(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 602);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1888);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 1889);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3707);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3708);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5290);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3709);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3710);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5292);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 5293);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3711);
					match s {
						[] => true,
						['ι', 'ε', 'ζ', 'α', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 5294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3712);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3713);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1890);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 3714);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1891);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 1892);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3715);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3716);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1893);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3717);
					s.is_empty()
				}
				['δ', 'η', 'τ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3718);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3719);
					match s {
						[] => true,
						['ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 5295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3720);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3721);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1894);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 3722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1895);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1896);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3723);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 3724);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5296);
							s.is_empty()
						}
						['μ', 'α', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 5297);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3725);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3726);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3727);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5298);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3728);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', 'ε', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
			super::push(out, 1897);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1898);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3729);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3730);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3731);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 3732);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1899);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3733);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3735);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5300);
							s.is_empty()
						}
						['σ', 'σ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 5301);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3736);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1901);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3737);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5302);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5303);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5304);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 3738);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5305);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3739);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3740);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3741);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3742);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3743);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5306);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5307);
							s.is_empty()
						}
						['σ', 'σ', 'ε', s @ ..] => {
							super::push(out, 5308);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3744);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1902);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3745);
					match s {
						[] => true,
						['ν', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 5309);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3746);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5310);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5311);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5312);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3747);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3748);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3749);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1903);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3750);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3751);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3752);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3753);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5313);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5314);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5315);
							match s {
								[] => true,
								['ο', 'ζ', 'ε', 'π', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 6181);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3754);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3755);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3756);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1904);
			match s {
				[] => true,
				['μ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3757);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1905);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3758);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3759);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3760);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 1906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κοσ.
pub(crate) fn n604(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 603);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3761);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3762);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3763);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5316);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6182);
									s.is_empty()
								}
								['ρ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 6183);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3764);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5317);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5318);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3765);
					match s {
						[] => true,
						['ρ', 'α', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 5319);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3766);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5320);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6184);
									s.is_empty()
								}
								['β', '\u{1}', s @ ..] => {
									super::push(out, 6185);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3767);
					match s {
						[] => true,
						['ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5321);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3768);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5322);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5323);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5324);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3769);
					s.is_empty()
				}
				['ψ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3770);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1908);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3771);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5325);
							s.is_empty()
						}
						['ρ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 5326);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'κ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3772);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1909);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1910);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3774);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3775);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1911);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3776);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 5327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 3777);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3778);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3779);
					match s {
						[] => true,
						['ρ', 'α', s @ ..] => {
							super::push(out, 5328);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3780);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3781);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3782);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3783);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5329);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5330);
							s.is_empty()
						}
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5331);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5332);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5333);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3784);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3785);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5335);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5336);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3786);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5337);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3787);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1912);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3788);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1913);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3789);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3790);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3791);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3792);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1914);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1915);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3793);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3794);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1916);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3795);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 5339);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3796);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3797);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5340);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5341);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6186);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5342);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6187);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 6480);
											s.is_empty()
										}
										['δ', s @ ..] => {
											super::push(out, 6481);
											s.is_empty()
										}
										['ζ', s @ ..] => {
											super::push(out, 6482);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 6188);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 6189);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 6190);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 5343);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5344);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6191);
									match s {
										[] => true,
										['ε', 'ν', '\u{1}', s @ ..] => {
											super::push(out, 6483);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 5345);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5346);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6192);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 6193);
									s.is_empty()
								}
								['η', s @ ..] => super::t1::n6366(s, out),
								['υ', s @ ..] => super::t1::n6367(s, out),
								['φ', s @ ..] => super::t1::n6368(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => super::t1::n3879(s, out),
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n1949(s, out),
		_ => false,
	}
}

/// Words ending -λοσ.
pub(crate) fn n605(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 604);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1918);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3804);
					match s {
						[] => true,
						['ο', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5348);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3805);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5349);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 5350);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5351);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 3806);
					match s {
						[] => true,
						['ν', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6197);
									match s {
										[] => true,
										['υ', 'ο', '\u{1}', s @ ..] => {
											super::push(out, 6484);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6198);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 6199);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5352);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3807);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3808);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 5353);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3809);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3810);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3811);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5354);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5355);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5356);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3812);
					match s {
						[] => true,
						['ε', 'κ', 'ο', s @ ..] => {
							super::push(out, 5357);
							match s {
								[] => true,
								['ν', 'υ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5358);
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
				['μ', s @ ..] => {
					super::push(out, 3813);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1920);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1921);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3815);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3816);
					match s {
						[] => true,
						['γ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6201);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 6202);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3817);
					match s {
						[] => true,
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 5359);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3818);
					match s {
						[] => true,
						['α', 'φ', 'σ', s @ ..] => {
							super::push(out, 5360);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5361);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 3819);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5362);
							match s {
								[] => true,
								['α', 'σ', 'σ', 'ι', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6203);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5363);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3820);
					match s {
						[] => true,
						['π', '\u{1}', s @ ..] => {
							super::push(out, 5364);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 3821);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1922);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3822);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 3823);
					match s {
						[] => true,
						['ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 5365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3824);
					match s {
						[] => true,
						['α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 5366);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3825);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3826);
					match s {
						[] => true,
						['α', 'κ', s @ ..] => {
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6204);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6205);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3827);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3828);
					match s {
						[] => true,
						['α', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 5367);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1923);
			match s {
				[] => true,
				['υ', '\u{1}', s @ ..] => {
					super::push(out, 3829);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1924);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3830);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3831);
					s.is_empty()
				}
				['δ', 'ο', 'κ', 'ο', 'ρ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3832);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3833);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3834);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3835);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5368);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3836);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3837);
					s.is_empty()
				}
				['τ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5369);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5370);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', '\u{1}', s @ ..] => {
					super::push(out, 3838);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1925);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3839);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3840);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3841);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3842);
					match s {
						[] => true,
						['κ', 'α', s @ ..] => {
							super::push(out, 5371);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1926);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3843);
					match s {
						[] => true,
						['β', s @ ..] => super::t1::n5513(s, out),
						['γ', s @ ..] => {
							super::push(out, 5373);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 5374);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 5375);
							s.is_empty()
						}
						['ι', s @ ..] => super::t1::n5517(s, out),
						['κ', s @ ..] => super::t1::n5518(s, out),
						['μ', s @ ..] => super::t1::n5519(s, out),
						['τ', s @ ..] => super::t1::n5520(s, out),
						['φ', s @ ..] => super::t1::n5521(s, out),
						_ => false,
					}
				}
				['ε', s @ ..] => super::t1::n3926(s, out),
				['ι', s @ ..] => super::t1::n3927(s, out),
				['υ', s @ ..] => super::t1::n3928(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n1959(s, out),
		['τ', s @ ..] => super::t1::n1960(s, out),
		['υ', s @ ..] => super::t1::n1961(s, out),
		['χ', s @ ..] => super::t1::n1962(s, out),
		['ω', s @ ..] => super::t1::n1963(s, out),
		_ => false,
	}
}

/// Words ending -μοσ.
pub(crate) fn n606(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 605);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1932);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3864);
					match s {
						[] => true,
						['ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3865);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5398);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3866);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 5399);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3867);
					match s {
						[] => true,
						['ο', 'π', 'ο', s @ ..] => {
							super::push(out, 5400);
							match s {
								[] => true,
								['π', 'π', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 6215);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3868);
					match s {
						[] => true,
						['κ', 'σ', 'ο', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 5401);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1933);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3869);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5402);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5403);
							match s {
								[] => true,
								['α', 'τ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 6216);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5404);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3870);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3871);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3872);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 3873);
					match s {
						[] => true,
						['ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 5406);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1934);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 3874);
					s.is_empty()
				}
				['ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1935);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 1936);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3876);
					s.is_empty()
				}
				['ι', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3877);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3878);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 5407);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3879);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3880);
					match s {
						[] => true,
						['ῥ', '\u{1}', s @ ..] => {
							super::push(out, 5408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 1937);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3881);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3882);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5409);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3883);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 3884);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 3885);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1938);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5410);
							match s {
								[] => true,
								['φ', 'ο', 'υ', s @ ..] => {
									super::push(out, 6217);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 5411);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3886);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1939);
			match s {
				[] => true,
				['α', 'ψ', '\u{1}', s @ ..] => {
					super::push(out, 3887);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1940);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3888);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3889);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5412);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5413);
							match s {
								[] => true,
								['κ', 'ι', 'α', s @ ..] => {
									super::push(out, 6218);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5414);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3890);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5415);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6219);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 6220);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6221);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 3891);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5416);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6222);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 6223);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 6224);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5417);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5418);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1941);
			match s {
				[] => true,
				['υ', 'φ', 'μ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => super::t1::n1974(s, out),
		['τ', s @ ..] => {
			super::push(out, 1943);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1944);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3898);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3899);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3900);
					match s {
						[] => true,
						['ε', '\u{1}', s @ ..] => {
							super::push(out, 5450);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3901);
					match s {
						[] => true,
						['ω', 'ι', s @ ..] => {
							super::push(out, 5451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3902);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3903);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1945);
			match s {
				[] => true,
				['υ', 'μ', s @ ..] => {
					super::push(out, 3904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1946);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νοσ.
pub(crate) fn n607(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 606);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1947);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 3905);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5452);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6259);
									match s {
										[] => true,
										['ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
											super::push(out, 6499);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 6260);
									match s {
										[] => true,
										['τ', 'ε', 'λ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6500);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5453);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3906);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5454);
							match s {
								[] => true,
								['ρ', 'τ', '\u{1}', s @ ..] => {
									super::push(out, 6261);
									s.is_empty()
								}
								['χ', '\u{1}', s @ ..] => {
									super::push(out, 6262);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 3907);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3908);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3909);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5455);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5456);
							match s {
								[] => true,
								['ι', 'α', 'κ', 'ο', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6263);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5457);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6501);
											s.is_empty()
										}
										['φ', '\u{1}', s @ ..] => {
											super::push(out, 6502);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5458);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3910);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5459);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5460);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3911);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5461);
							match s {
								[] => true,
								['β', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6503);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 6504);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['γ', s @ ..] => {
							super::push(out, 5462);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 5463);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5464);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 5465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3912);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3913);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3914);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5466);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 5467);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5468);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3916);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5469);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3917);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3918);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5470);
							match s {
								[] => true,
								['λ', 'π', 'ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 6264);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5471);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3919);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5473);
							s.is_empty()
						}
						['α', 'ρ', s @ ..] => {
							super::push(out, 5474);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 3921);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1948);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5475);
							s.is_empty()
						}
						['φ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 5476);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1949);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1950);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 3922);
					match s {
						[] => true,
						['ρ', 'α', 'π', 'ο', s @ ..] => {
							super::push(out, 5477);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3923);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5478);
							s.is_empty()
						}
						['υ', 'ο', 'δ', s @ ..] => {
							super::push(out, 5479);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 3924);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1951);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3925);
					match s {
						[] => true,
						['α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 5480);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 3926);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3927);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5481);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 6265);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5482);
							match s {
								[] => true,
								['ζ', 'υ', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6266);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 5483);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3928);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3929);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5484);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3930);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n1984(s, out),
		['κ', s @ ..] => {
			super::push(out, 1953);
			match s {
				[] => true,
				['υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3946);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1954);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 3948);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3949);
					match s {
						[] => true,
						['δ', 'ε', 'μ', 'ο', s @ ..] => {
							super::push(out, 5506);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3950);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3951);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5508);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => super::t1::n1987(s, out),
		['ο', s @ ..] => super::t1::n1988(s, out),
		['π', s @ ..] => super::t1::n1989(s, out),
		['ρ', s @ ..] => super::t1::n1990(s, out),
		['υ', s @ ..] => super::t1::n1991(s, out),
		['χ', s @ ..] => super::t1::n1992(s, out),
		['ω', s @ ..] => super::t1::n1993(s, out),
		_ => false,
	}
}

/// Words ending -ξοσ.
pub(crate) fn n608(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 607);
	match s {
		[] => true,
		['α', '\u{1}', s @ ..] => {
			super::push(out, 1962);
			s.is_empty()
		}
		['ο', 'τ', 'ο', 'ρ', 'υ', 'γ', 'ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 1963);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1964);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οοσ.
pub(crate) fn n609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 608);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1965);
			match s {
				[] => true,
				['π', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5518);
							s.is_empty()
						}
						['κ', 'ε', 'ι', s @ ..] => {
							super::push(out, 5519);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1966);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3980);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 3981);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1967);
			match s {
				[] => true,
				['ρ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3982);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ποσ.
pub(crate) fn n610(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 609);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1968);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1969);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3983);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1970);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 1971);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3985);
					match s {
						[] => true,
						['κ', 'ο', s @ ..] => {
							super::push(out, 5520);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3986);
					match s {
						[] => true,
						['π', 'ο', 'χ', s @ ..] => {
							super::push(out, 5521);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1972);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3987);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 3988);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5522);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5523);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6281);
									s.is_empty()
								}
								['ο', 'ν', 'α', s @ ..] => {
									super::push(out, 6282);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3989);
					match s {
						[] => true,
						['τ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6283);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 6284);
									s.is_empty()
								}
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
			super::push(out, 1973);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3990);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3991);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5524);
							s.is_empty()
						}
						['σ', 'ρ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 5525);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1974);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1975);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 3992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1976);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3993);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3994);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 3995);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ροσ.
pub(crate) fn n611(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 610);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1977);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3996);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 3997);
					match s {
						[] => true,
						['ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 5526);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 3998);
					match s {
						[] => true,
						['ν', 'α', 'κ', 'ο', s @ ..] => {
							super::push(out, 5527);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3999);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4000);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5528);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4001);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5530);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 5531);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4002);
					match s {
						[] => true,
						['θ', '\u{1}', s @ ..] => {
							super::push(out, 5532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 4003);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 4004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 1978);
			match s {
				[] => true,
				['μ', 'ι', s @ ..] => {
					super::push(out, 4005);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1979);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4006);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5533);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 4007);
					match s {
						[] => true,
						['ι', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 5535);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1980);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4008);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5536);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5537);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 4009);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5539);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5540);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5541);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4010);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1981);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4011);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 4012);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4013);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4014);
					match s {
						[] => true,
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 5544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4015);
					match s {
						[] => true,
						['υ', '\u{1}', s @ ..] => {
							super::push(out, 5545);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4016);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5546);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5547);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5548);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1982);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4017);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1983);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1984);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4018);
					match s {
						[] => true,
						['κ', 'ο', s @ ..] => {
							super::push(out, 5549);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4019);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 5550);
							match s {
								[] => true,
								['α', 'μ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6510);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 6511);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6285);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5551);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 5552);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4020);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1985);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1986);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4021);
					match s {
						[] => true,
						['η', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5553);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 4022);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5554);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 4023);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4024);
					match s {
						[] => true,
						['α', 'ζ', s @ ..] => {
							super::push(out, 5555);
							match s {
								[] => true,
								['ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 6286);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4025);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5556);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4026);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4027);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5557);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 5558);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6287);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 6288);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5559);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6289);
									s.is_empty()
								}
								['δ', s @ ..] => {
									super::push(out, 6290);
									s.is_empty()
								}
								['ζ', s @ ..] => {
									super::push(out, 6291);
									s.is_empty()
								}
								['θ', s @ ..] => {
									super::push(out, 6292);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 6293);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 6294);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5560);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 5561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1987);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4028);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4029);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1988);
			match s {
				[] => true,
				['α', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 4030);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4031);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => super::t2::n2021(s, out),
		['υ', s @ ..] => super::t2::n2022(s, out),
		['φ', s @ ..] => {
			super::push(out, 1991);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4046);
					match s {
						[] => true,
						['\u{1}', s @ ..] => super::t2::n5726(s, out),
						['τ', '\u{1}', s @ ..] => super::t2::n5727(s, out),
						_ => false,
					}
				}
				['ε', s @ ..] => super::t2::n4132(s, out),
				['ι', s @ ..] => super::t2::n4133(s, out),
				_ => false,
			}
		}
		['χ', s @ ..] => super::t2::n2024(s, out),
		['ω', s @ ..] => super::t2::n2025(s, out),
		_ => false,
	}
}

/// Words ending -σοσ.
pub(crate) fn n612(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 611);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1994);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4051);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4052);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4053);
					match s {
						[] => true,
						['ε', 'κ', 'ι', s @ ..] => {
							super::push(out, 5581);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5582);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4054);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1995);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1996);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1997);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4055);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 4056);
					match s {
						[] => true,
						['β', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5583);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4057);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4058);
					match s {
						[] => true,
						['α', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 5584);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4059);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5585);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4060);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1998);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1999);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2000);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4061);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5586);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4062);
					s.is_empty()
				}
				['ι', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 4063);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4064);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2001);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4065);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4066);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5587);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6305);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5588);
							match s {
								[] => true,
								['α', 'π', 'υ', 'κ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6516);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 6517);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 6306);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4067);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4068);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5589);
							s.is_empty()
						}
						['β', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6307);
									s.is_empty()
								}
								['α', '\u{1}', s @ ..] => {
									super::push(out, 6308);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'β', '\u{1}', s @ ..] => {
							super::push(out, 5590);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 4069);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2002);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4070);
					s.is_empty()
				}
				['λ', s @ ..] => {
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
						['α', s @ ..] => {
							super::push(out, 5591);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 5592);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 5593);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6309);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τοσ.
pub(crate) fn n613(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 612);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2003);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4074);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 5594);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5595);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4075);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4076);
					match s {
						[] => true,
						['α', 'θ', '\u{1}', s @ ..] => {
							super::push(out, 5596);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4077);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5597);
							s.is_empty()
						}
						['υ', 'θ', s @ ..] => {
							super::push(out, 5598);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2004);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4078);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5599);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5600);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 4079);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4080);
					match s {
						[] => true,
						['α', 'ι', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5601);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 4081);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5602);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4082);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4083);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4084);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2005);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4085);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4086);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2006);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4088);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4089);
					match s {
						[] => true,
						['ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 5603);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4090);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4091);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 5604);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2007);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4092);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 5605);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4093);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4094);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4095);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 4096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2008);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4097);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4098);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4099);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2009);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2011);
			match s {
				[] => true,
				['α', 'β', 'ι', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 4101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2012);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4102);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 5606);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4103);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4104);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4105);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5608);
							s.is_empty()
						}
						['β', 'α', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5609);
							s.is_empty()
						}
						['κ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5610);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4106);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5611);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5612);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5613);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4108);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4109);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 5614);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 5615);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 4110);
					s.is_empty()
				}
				['ω', 'λ', 'γ', 'ο', s @ ..] => {
					super::push(out, 4111);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2015);
			match s {
				[] => true,
				['α', 'ι', 'ν', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 4112);
					s.is_empty()
				}
				['κ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5617);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 5618);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4113);
					match s {
						[] => true,
						['υ', 'ο', 'β', '\u{1}', s @ ..] => {
							super::push(out, 5619);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4114);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 5620);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 5621);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', 'ο', 'ε', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 4115);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2016);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4116);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 4117);
					s.is_empty()
				}
				['λ', 'α', 'μ', 'χ', 'ι', 'α', '\u{1}', s @ ..] => {
					super::push(out, 4118);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4119);
					match s {
						[] => true,
						['α', 'β', 'ι', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 5622);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υοσ.
pub(crate) fn n614(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 613);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2017);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 2018);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 2019);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2020);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φοσ.
pub(crate) fn n615(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 614);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2021);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4120);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5623);
							s.is_empty()
						}
						['ε', 'γ', s @ ..] => {
							super::push(out, 5624);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4121);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6310);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6311);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 6518);
											match s {
												[] => true,
												['μ', 'α', 'ρ', 'υ', 'θ', 'ι', 'δ', '\u{1}', s @ ..] => {
													super::push(out, 6582);
													s.is_empty()
												}
												_ => false,
											}
										}
										['θ', s @ ..] => {
											super::push(out, 6519);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 6520);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 6583);
													match s {
														[] => true,
														['ω', 'γ', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => super::t2::n6791(s, out),
														_ => false,
													}
												}
												['λ', s @ ..] => {
													super::push(out, 6584);
													s.is_empty()
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 6521);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 6522);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 6585);
													s.is_empty()
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 6523);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 6524);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ω', s @ ..] => {
									super::push(out, 6312);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 5625);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4122);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2022);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 2023);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2024);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5626);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 5627);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2025);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4123);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4124);
					match s {
						[] => true,
						['δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4125);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2026);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4126);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2027);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 4128);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2028);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2029);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4129);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4130);
					s.is_empty()
				}
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 4131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2030);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2031);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4132);
					match s {
						[] => true,
						['γ', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6313);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6314);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 5629);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4133);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4134);
					match s {
						[] => true,
						['ο', 'κ', 'γ', 'α', 'λ', 'ε', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 5630);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χοσ.
pub(crate) fn n616(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 615);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2032);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4136);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 4137);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4138);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4139);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4140);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4141);
					match s {
						[] => true,
						['τ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 5631);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4142);
					s.is_empty()
				}
				['ῥ', s @ ..] => {
					super::push(out, 4143);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2033);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4144);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4145);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4146);
					match s {
						[] => true,
						['ρ', 'ρ', 'υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 5632);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2034);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 2035);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2036);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4147);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4148);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5633);
							match s {
								[] => true,
								['τ', '\u{1}', s @ ..] => {
									super::push(out, 6315);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5634);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4149);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2037);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2038);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4150);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4151);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4152);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5635);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5636);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4153);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2039);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5637);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 5638);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5639);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 6316);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 6317);
									match s {
										[] => true,
										['α', s @ ..] => {
											match s {
												[] => true,
												['ν', 'μ', 'υ', 'γ', '\u{1}', s @ ..] => {
													super::push(out, 6586);
													s.is_empty()
												}
												['τ', 'σ', '\u{1}', s @ ..] => {
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
						['λ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 5640);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5641);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5642);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5643);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5644);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5645);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2040);
			match s {
				[] => true,
				['ι', 'μ', s @ ..] => {
					super::push(out, 4156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2041);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4157);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4158);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4159);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5646);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 6318);
									match s {
										[] => true,
										['α', 'ρ', 'ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6525);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 6319);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5647);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 5648);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5649);
							match s {
								[] => true,
								['υ', s @ ..] => {
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6526);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 6527);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 6320);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5650);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4160);
					match s {
						[] => true,
						['τ', '\u{1}', s @ ..] => {
							super::push(out, 5651);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψοσ.
pub(crate) fn n617(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 616);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2042);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4161);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 4162);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 4163);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4164);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2043);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2044);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2045);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4165);
					s.is_empty()
				}
				['γ', '\u{1}', s @ ..] => {
					super::push(out, 4166);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωοσ.
pub(crate) fn n618(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 617);
	s.is_empty()
}

/// The word αρκυσ.
pub(crate) fn n5817(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5665);
	s.is_empty()
}

/// Words ending -μαρκυσ.
pub(crate) fn n5818(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5666);
	s.is_empty()
}

/// Words ending -ορκυσ.
pub(crate) fn n4295(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4204);
	s.is_empty()
}

/// Words ending -λυσ.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2068);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 2069);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2070);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 2071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μυσ.
pub(crate) fn n629(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 628);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2072);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 2073);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 2074);
			s.is_empty()
		}
		['λ', 'α', 'π', '\u{1}', s @ ..] => {
			super::push(out, 2075);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 2076);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νυσ.
pub(crate) fn n630(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 629);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2077);
			match s {
				[] => true,
				['ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 4205);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξυσ.
pub(crate) fn n631(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 630);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2079);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ουσ.
pub(crate) fn n632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 631);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 2081);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4206);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4207);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5667);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2082);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2083);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2084);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2085);
			match s {
				[] => true,
				['ι', 'χ', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 4208);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2086);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5669);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5670);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5671);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4210);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4211);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4212);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5672);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5673);
							match s {
								[] => true,
								['ι', 'χ', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 6325);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5674);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4213);
					match s {
						[] => true,
						['σ', 'α', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 5676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 4214);
					match s {
						[] => true,
						['γ', 'α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 5677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 2087);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πυσ.
pub(crate) fn n633(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 632);
	s.is_empty()
}

/// Words ending -ρυσ.
pub(crate) fn n634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 633);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 2088);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 2089);
			match s {
				[] => true,
				['ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 4215);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2090);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2091);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2092);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 2093);
			match s {
				[] => true,
				['ο', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 4216);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 2094);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συσ.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τυσ.
pub(crate) fn n636(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 635);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2096);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4217);
					s.is_empty()
				}
				['π', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5678);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2097);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2098);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2099);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φυσ.
pub(crate) fn n637(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 636);
	match s {
		[] => true,
		['α', 'ῥ', '\u{1}', s @ ..] => {
			super::push(out, 2100);
			s.is_empty()
		}
		['σ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 2101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χυσ.
pub(crate) fn n638(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 637);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2102);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4218);
					match s {
						[] => true,
						['β', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5680);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4219);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2103);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2104);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεια.
pub(crate) fn n723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 717);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2211);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4269);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4270);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4271);
					match s {
						[] => true,
						['υ', 'θ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 5709);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4272);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5710);
							match s {
								[] => true,
								['α', '\u{1}', s @ ..] => {
									super::push(out, 6342);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 5711);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2212);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 2213);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2214);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2215);
			match s {
				[] => true,
				['α', 'μ', 'ο', s @ ..] => {
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5712);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5713);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5714);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 5715);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6343);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 6344);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 6345);
									s.is_empty()
								}
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
			super::push(out, 2216);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2217);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2218);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4273);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 4274);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4275);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αυτοκρατια.
pub(crate) fn n6782(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6588);
	s.is_empty()
}

/// Words ending -ουτοκρατια.
pub(crate) fn n6783(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6589);
	s.is_empty()
}

/// Words ending -οποδιον.
pub(crate) fn n6010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5850);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 6383);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 6384);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 6385);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 6386);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 6387);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γωδιον.
pub(crate) fn n4706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4593);
	s.is_empty()
}

/// Words ending -ειον.
pub(crate) fn n1324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1298);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2827);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4594);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4595);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 5852);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5853);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4596);
					match s {
						[] => true,
						['υ', 'ο', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 6388);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 6389);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6390);
									s.is_empty()
								}
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
			super::push(out, 2828);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4597);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 4598);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2829);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2830);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 2831);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2832);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4599);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4600);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4601);
					match s {
						[] => true,
						['β', 'ο', 'λ', 'υ', 'φ', 'α', 'τ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 5854);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 4602);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2833);
			match s {
				[] => true,
				['ο', 'κ', 'ο', 'ρ', 'υ', s @ ..] => {
					super::push(out, 4603);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2834);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4604);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 5855);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 5856);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5857);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4605);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4606);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 4607);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2835);
			match s {
				[] => true,
				['υ', 'τ', 'ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 4608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2836);
			match s {
				[] => true,
				['α', 'χ', 'σ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 4609);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4610);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4611);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4612);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 5858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2837);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2838);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4613);
					s.is_empty()
				}
				['σ', 'ι', 'ρ', s @ ..] => {
					super::push(out, 4614);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 2839);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4615);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4616);
					match s {
						[] => true,
						['ρ', 'τ', 'ο', 'γ', s @ ..] => {
							super::push(out, 5859);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 2840);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4617);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζιον.
pub(crate) fn n1325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1299);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2841);
			match s {
				[] => true,
				['λ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 4618);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2842);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2843);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηιον.
pub(crate) fn n1326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1300);
	match s {
		[] => true,
		['ν', 'ν', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 2844);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θιον.
pub(crate) fn n1327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1301);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2845);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2846);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4619);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4620);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2847);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4621);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4622);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4623);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κιον.
pub(crate) fn n1328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1302);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2848);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4624);
					s.is_empty()
				}
				['ζ', s @ ..] => {
					super::push(out, 4625);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4626);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4627);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4628);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5860);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5861);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 5862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4629);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5863);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5864);
							match s {
								[] => true,
								['ι', 'π', '\u{1}', s @ ..] => {
									super::push(out, 6391);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', 'ι', 'π', '\u{1}', s @ ..] => {
							super::push(out, 5865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2849);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2850);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4630);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4631);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5866);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 4633);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2851);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2852);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4634);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2853);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2854);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4635);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λιον.
pub(crate) fn n1329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1303);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2855);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4636);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5867);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4637);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4638);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4639);
					match s {
						[] => true,
						['ε', 'κ', 'ο', 'ν', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 5868);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 4640);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 2856);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4641);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4642);
					match s {
						[] => true,
						['β', '\u{1}', s @ ..] => {
							super::push(out, 5869);
							s.is_empty()
						}
						['ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 5870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2857);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 2858);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4643);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2859);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4644);
					s.is_empty()
				}
				['μ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5871);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5872);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2860);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 2861);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2862);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4645);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5873);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 5874);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 4646);
					match s {
						[] => true,
						['γ', 'α', 'ρ', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 5875);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4647);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4648);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5876);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2863);
			match s {
				[] => true,
				['π', 'ι', 'ρ', s @ ..] => {
					super::push(out, 4649);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2864);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4650);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 4651);
					s.is_empty()
				}
				['δ', 'ν', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 4652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2865);
			match s {
				[] => true,
				['π', 'ο', 'χ', s @ ..] => {
					super::push(out, 4653);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μιον.
pub(crate) fn n1330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1304);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 2866);
			s.is_empty()
		}
		['ο', 'τ', s @ ..] => {
			super::push(out, 2867);
			match s {
				[] => true,
				['σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 4654);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νιον.
pub(crate) fn n1331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1305);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2868);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4655);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 5877);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5878);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 5879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 4656);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4657);
					match s {
						[] => true,
						['σ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 5880);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 4658);
					match s {
						[] => true,
						['κ', '\u{1}', s @ ..] => {
							super::push(out, 5881);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4659);
					match s {
						[] => true,
						['ε', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 5882);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5883);
							s.is_empty()
						}
						['υ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 5884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2869);
			match s {
				[] => true,
				['μ', 'ι', s @ ..] => {
					super::push(out, 4660);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2870);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 4661);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2871);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4662);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 4663);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4664);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4665);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4666);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2872);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2873);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2874);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4667);
					match s {
						[] => true,
						['υ', 'ρ', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 5885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 4668);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2875);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2876);
			match s {
				[] => true,
				['ε', 'τ', s @ ..] => {
					super::push(out, 4669);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4670);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2877);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4671);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 2878);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4672);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4673);
					match s {
						[] => true,
						['λ', 'ο', 'κ', 'σ', 'ι', 'λ', 'ε', 'β', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 5886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2879);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4674);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 5887);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 4675);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 5888);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 4676);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξιον.
pub(crate) fn n1332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1306);
	s.is_empty()
}

/// Words ending -οιον.
pub(crate) fn n1333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1307);
	s.is_empty()
}

/// Words ending -πιον.
pub(crate) fn n1334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1308);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2880);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2881);
			match s {
				[] => true,
				['κ', 'σ', 'ο', 'ι', s @ ..] => {
					super::push(out, 4677);
					s.is_empty()
				}
				['ρ', 'τ', 'ο', s @ ..] => {
					super::push(out, 4678);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2882);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2883);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4679);
					match s {
						[] => true,
						['κ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 5889);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4680);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2884);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 2885);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ριον.
pub(crate) fn n1335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1309);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2886);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4681);
					match s {
						[] => true,
						['υ', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 5890);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'λ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 4682);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4683);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5891);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 5892);
							match s {
								[] => true,
								['λ', 'ω', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 6392);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 5893);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6393);
									s.is_empty()
								}
								['ῥ', '\u{1}', s @ ..] => {
									super::push(out, 6394);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 5894);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 4684);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4685);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5895);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5896);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 4686);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5897);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 2887);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 2888);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4687);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4688);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4689);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4690);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2889);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4691);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4692);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 5898);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5899);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5900);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2890);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 4693);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4694);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4695);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4696);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 4697);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 4698);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5901);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6395);
									s.is_empty()
								}
								['λ', s @ ..] => {
									super::push(out, 6396);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6397);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6398);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 5902);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5903);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5904);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6399);
									match s {
										[] => true,
										['λ', 'υ', 'φ', '\u{1}', s @ ..] => {
											super::push(out, 6537);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', s @ ..] => {
									super::push(out, 6400);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 5905);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 5906);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 5907);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 5908);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5909);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6401);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 6538);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 6539);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 6540);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 6402);
									s.is_empty()
								}
								['η', s @ ..] => {
									super::push(out, 6403);
									match s {
										[] => true,
										['θ', 'λ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6541);
											s.is_empty()
										}
										['ρ', 'χ', '\u{1}', s @ ..] => {
											super::push(out, 6542);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6404);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 6543);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 6544);
											match s {
												[] => true,
												['α', 'χ', '\u{1}', s @ ..] => {
													super::push(out, 6591);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 6545);
											s.is_empty()
										}
										['χ', s @ ..] => {
											super::push(out, 6546);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 6405);
									match s {
										[] => true,
										['ε', 'γ', '\u{1}', s @ ..] => {
											super::push(out, 6547);
											s.is_empty()
										}
										['μ', '\u{1}', s @ ..] => {
											super::push(out, 6548);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 5910);
							match s {
								[] => true,
								['λ', 'α', s @ ..] => {
									super::push(out, 6406);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 5911);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2891);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4699);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2892);
			match s {
				[] => true,
				['α', 'φ', 'σ', 'ι', 'μ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 4700);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2893);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2894);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 4701);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 5912);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5913);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5914);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => super::t2::n4816(s, out),
				['χ', s @ ..] => super::t2::n4817(s, out),
				_ => false,
			}
		}
		['π', s @ ..] => super::t2::n2955(s, out),
		['ρ', s @ ..] => super::t2::n2956(s, out),
		['τ', s @ ..] => super::t2::n2957(s, out),
		['υ', s @ ..] => super::t2::n2958(s, out),
		['ω', s @ ..] => super::t2::n2959(s, out),
		_ => false,
	}
}

/// Words ending -σιον.
pub(crate) fn n1336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1310);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2900);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4715);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4716);
					match s {
						[] => true,
						['μ', 'υ', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 5917);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4717);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 5918);
							s.is_empty()
						}
						['π', 'μ', s @ ..] => {
							super::push(out, 5919);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2901);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 4718);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4719);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 4720);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2902);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 4721);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4722);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2903);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4723);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2904);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4724);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4725);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4726);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2905);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τιον.
pub(crate) fn n1337(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1311);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2906);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 4727);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 4728);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 4729);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4730);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 5920);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 5921);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 5922);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 5923);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 5924);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 4731);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 5925);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 5926);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 5927);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2907);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2908);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 2909);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2910);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2911);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4732);
					match s {
						[] => true,
						['λ', 'λ', 'α', 'β', '\u{1}', s @ ..] => {
							super::push(out, 5928);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4733);
					match s {
						[] => true,
						['κ', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 6407);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6408);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 5929);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2912);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2913);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2914);
			match s {
				[] => true,
				['υ', 'α', 'λ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 4734);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2915);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 2916);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 4735);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 4736);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word γυιον.
pub(crate) fn n1338(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1312);
	s.is_empty()
}

/// Words ending -φιον.
pub(crate) fn n1339(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1313);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2917);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2918);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2919);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2920);
			match s {
				[] => true,
				['ρ', 'τ', 'σ', 'ο', 'ν', 'η', 'γ', 'α', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 4737);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χιον.
pub(crate) fn n1340(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1314);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2921);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 2922);
			match s {
				[] => true,
				['α', 'ρ', 'β', '\u{1}', s @ ..] => {
					super::push(out, 4738);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2923);
			match s {
				[] => true,
				['ι', '\u{1}', s @ ..] => {
					super::push(out, 4739);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2924);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ῥιον.
pub(crate) fn n1341(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1315);
	s.is_empty()
}

/// The word μυριοφυλλον.
pub(crate) fn n6786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6592);
	s.is_empty()
}

/// Words ending -τονον.
pub(crate) fn n3083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3020);
	match s {
		[] => true,
		['κ', 'ο', s @ ..] => {
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5959);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5960);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πνον.
pub(crate) fn n1380(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1354);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3022);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4804);
					s.is_empty()
				}
				['ε', 'δ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 5961);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 5962);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3023);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρνον.
pub(crate) fn n1381(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1355);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3024);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4805);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4806);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3025);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χνον.
pub(crate) fn n1383(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1357);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 3026);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 3027);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3028);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωνον.
pub(crate) fn n1384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1358);
	match s {
		[] => true,
		['γ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4808);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4809);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4810);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αστρον.
pub(crate) fn n4969(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4846);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 5973);
			s.is_empty()
		}
		['β', 'α', 'λ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 5974);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 5975);
			match s {
				[] => true,
				['υ', 'ζ', '\u{1}', s @ ..] => {
					super::push(out, 6418);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', 'π', 'μ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 5976);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 5977);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηστρον.
pub(crate) fn n4971(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4848);
	match s {
		[] => true,
		['λ', 'β', 'ι', 'φ', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 5978);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιστρον.
pub(crate) fn n4972(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4849);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 5979);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 5980);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 5981);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υστρον.
pub(crate) fn n4973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4850);
	s.is_empty()
}

/// Words ending -υτρον.
pub(crate) fn n3131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3066);
	match s {
		[] => true,
		['λ', '\u{1}', s @ ..] => {
			super::push(out, 4851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτρον.
pub(crate) fn n3132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3067);
	match s {
		[] => true,
		['σ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 4852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υρον.
pub(crate) fn n1402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1376);
	match s {
		[] => true,
		['α', 'τ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 3068);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3069);
			match s {
				[] => true,
				['λ', 'α', s @ ..] => {
					super::push(out, 4853);
					s.is_empty()
				}
				['ν', 'υ', s @ ..] => {
					super::push(out, 4854);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 3070);
			match s {
				[] => true,
				['ι', 'σ', 'υ', 'α', 'λ', 'κ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 4855);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 3071);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 3072);
			s.is_empty()
		}
		['ξ', s @ ..] => {
			super::push(out, 3073);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3074);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4856);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4857);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3075);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4858);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4859);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4860);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3076);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4861);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4862);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4863);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3077);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 3078);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωρον.
pub(crate) fn n1403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1377);
	match s {
		[] => true,
		['μ', 'υ', s @ ..] => {
			super::push(out, 3079);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 3080);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιτησ.
pub(crate) fn n1725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1695);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3337);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 3338);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 3339);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4950);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3340);
			match s {
				[] => true,
				['ρ', 'α', s @ ..] => {
					super::push(out, 4951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 3341);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 3342);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 3343);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4952);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 4953);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4954);
					match s {
						[] => true,
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6442);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 6557);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 6558);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 6559);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 6443);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 6444);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 6445);
									match s {
										[] => true,
										['λ', 'α', 'γ', 'ε', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 6560);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 6446);
									match s {
										[] => true,
										['ο', 'ν', 'ι', 'τ', 'ν', 'α', 'τ', 'σ', 'ν', 'ω', 'κ', '\u{1}', s @ ..] => {
											super::push(out, 6561);
											s.is_empty()
										}
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
					super::push(out, 4955);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3344);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 4956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3345);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4957);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 6011);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 6012);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 6013);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 4958);
					match s {
						[] => true,
						['σ', 'ρ', 'υ', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 6014);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 4959);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 4960);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3346);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3347);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4961);
					match s {
						[] => true,
						['β', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 6015);
							s.is_empty()
						}
						['μ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 6016);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 4962);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 4963);
					match s {
						[] => true,
						['δ', 'ι', s @ ..] => {
							super::push(out, 6017);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 4964);
					match s {
						[] => true,
						['ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 6018);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4965);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6019);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 3348);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4966);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4967);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 4968);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ελτησ.
pub(crate) fn n3429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3354);
	match s {
		[] => true,
		['π', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 4976);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ντησ.
pub(crate) fn n1728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1698);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3355);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4977);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 4978);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3356);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οτησ.
pub(crate) fn n1729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1699);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3357);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 3358);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3359);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4979);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 6022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 4980);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 4981);
					match s {
						[] => true,
						['δ', '\u{1}', s @ ..] => {
							super::push(out, 6023);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 4982);
					s.is_empty()
				}
				['ρ', 'η', s @ ..] => {
					super::push(out, 4983);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4984);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 3360);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 3361);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 4985);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4986);
					match s {
						[] => true,
						['ψ', s @ ..] => {
							super::push(out, 6024);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 4987);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3362);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 4988);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3363);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4989);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 4990);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 4991);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 3364);
			match s {
				[] => true,
				['ο', 'τ', 'ο', s @ ..] => {
					super::push(out, 4992);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3365);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4993);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 4994);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 4995);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 4996);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 4997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3366);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4998);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 4999);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3367);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5000);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3368);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πτησ.
pub(crate) fn n1730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1700);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3369);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3370);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3371);
			match s {
				[] => true,
				['λ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 5001);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρτησ.
pub(crate) fn n1731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1701);
	match s {
		[] => true,
		['α', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 3372);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -στησ.
pub(crate) fn n1732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1702);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3373);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5002);
					match s {
						[] => true,
						['ν', 'π', 'υ', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 6025);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6026);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5003);
					match s {
						[] => true,
						['ι', 'δ', 'ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6027);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 5004);
					match s {
						[] => true,
						['ε', 'γ', 'α', s @ ..] => {
							super::push(out, 6028);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 5005);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6030);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 6031);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 6032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5006);
					match s {
						[] => true,
						['ε', 'λ', s @ ..] => {
							super::push(out, 6033);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 5007);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 5008);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 5009);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3374);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5010);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5011);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 3375);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3376);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5012);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 5013);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 5014);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5015);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6034);
							match s {
								[] => true,
								['ο', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 6447);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 6035);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 5016);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6036);
							s.is_empty()
						}
						['β', s @ ..] => {
							super::push(out, 6037);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6038);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 5017);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5018);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 6039);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 6040);
							match s {
								[] => true,
								['γ', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6562);
											s.is_empty()
										}
										['τ', 'ν', s @ ..] => {
											super::push(out, 6563);
											s.is_empty()
										}
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
					super::push(out, 5019);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6041);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 6042);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 6043);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 6044);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 5020);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 5021);
					match s {
						[] => true,
						['ο', 'σ', 'ο', 'ν', 'μ', 'υ', 'γ', '\u{1}', s @ ..] => {
							super::push(out, 6045);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3377);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5022);
					match s {
						[] => true,
						['ρ', 'υ', s @ ..] => {
							super::push(out, 6046);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 5023);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5024);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5025);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 3378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υτησ.
pub(crate) fn n1733(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1703);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3379);
			match s {
				[] => true,
				['ν', 'ο', 'γ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 5026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 3380);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 3381);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5027);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3382);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5028);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5029);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5030);
					match s {
						[] => true,
						['η', s @ ..] => {
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 6448);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 6449);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 6047);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5031);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5032);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5033);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3383);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5034);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6048);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 6049);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6050);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5035);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 5036);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 3384);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3385);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 3386);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 3387);
			match s {
				[] => true,
				['α', 'ρ', 'τ', s @ ..] => {
					super::push(out, 5037);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωτησ.
pub(crate) fn n1734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1704);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 3388);
			match s {
				[] => true,
				['υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5038);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3389);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5039);
					s.is_empty()
				}
				['δ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 5040);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 5041);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 5042);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5043);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5044);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6051);
							match s {
								[] => true,
								['τ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 6450);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 6052);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5045);
					match s {
						[] => true,
						['τ', 'α', 'π', 'μ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 6053);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 5046);
					match s {
						[] => true,
						['ε', 'ρ', 'ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6054);
							s.is_empty()
						}
						['η', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 6055);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5047);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3390);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3391);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αναστασισ.
pub(crate) fn n6787(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6593);
	s.is_empty()
}

/// The word μεταναστασισ.
pub(crate) fn n6788(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6594);
	s.is_empty()
}

/// The word καταστασισ.
pub(crate) fn n6789(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6595);
	s.is_empty()
}

/// Words ending -οκαταστασισ.
pub(crate) fn n6790(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6596);
	s.is_empty()
}

/// Words ending -υνθεσισ.
pub(crate) fn n6263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6094);
	s.is_empty()
}

/// Words ending -οθεσισ.
pub(crate) fn n5258(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5124);
	match s {
		[] => true,
		['π', 'υ', '\u{1}', s @ ..] => {
			super::push(out, 6095);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μεσισ.
pub(crate) fn n3588(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3513);
	s.is_empty()
}

/// Words ending -νεσισ.
pub(crate) fn n3589(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3514);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5125);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 5126);
			match s {
				[] => true,
				['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 6096);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 5127);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρεσισ.
pub(crate) fn n3590(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3515);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 5128);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 5129);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6463);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 6464);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 6465);
							match s {
								[] => true,
								['υ', '\u{1}', s @ ..] => {
									super::push(out, 6568);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φεσισ.
pub(crate) fn n3591(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3516);
	s.is_empty()
}

/// Words ending -χεσισ.
pub(crate) fn n3592(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3517);
	match s {
		[] => true,
		['σ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 6097);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 6098);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ησισ.
pub(crate) fn n1830(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1799);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 3518);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 5130);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6099);
							s.is_empty()
						}
						['ξ', s @ ..] => {
							super::push(out, 6100);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 6101);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 5131);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 3519);
			match s {
				[] => true,
				['ι', 'ε', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5132);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 3520);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 3521);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5133);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 5134);
					match s {
						[] => true,
						['ι', 'α', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 6102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3522);
			match s {
				[] => true,
				['ρ', 'υ', 'τ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5135);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 3523);
			match s {
				[] => true,
				['α', 'κ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 5136);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5137);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 6103);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 6466);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 6467);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 5138);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3524);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5139);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6104);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6105);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 6106);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5140);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3525);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5141);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5142);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5143);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 5144);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5145);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3526);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5146);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5147);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5148);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 5149);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5150);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5151);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 6107);
							match s {
								[] => true,
								['φ', 'ι', s @ ..] => {
									super::push(out, 6468);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 6108);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5152);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 5153);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 5154);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3527);
			match s {
				[] => true,
				['ω', 'ι', 'σ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 5155);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3528);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5156);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5157);
					match s {
						[] => true,
						['ε', 'χ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 6109);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5158);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 6110);
							s.is_empty()
						}
						['φ', 'α', s @ ..] => {
							super::push(out, 6111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 5159);
					match s {
						[] => true,
						['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 6112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 5160);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3529);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5161);
					match s {
						[] => true,
						['ρ', 'κ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 6113);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 5162);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 5163);
					match s {
						[] => true,
						['ζ', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 6114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5164);
					match s {
						[] => true,
						['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 6115);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5165);
					match s {
						[] => true,
						['α', 'ν', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6116);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 5166);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 3530);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ισισ.
pub(crate) fn n1831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1800);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 3531);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3532);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 3533);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 3534);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 3535);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3536);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3537);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5168);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5169);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6117);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 6118);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6119);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6120);
							match s {
								[] => true,
								['π', s @ ..] => {
									match s {
										[] => true,
										['α', 'τ', s @ ..] => {
											super::push(out, 6569);
											s.is_empty()
										}
										['υ', s @ ..] => {
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
				['χ', s @ ..] => {
					super::push(out, 5170);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 6121);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 6122);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 6123);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6124);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3538);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 3539);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 3540);
			match s {
				[] => true,
				['ι', 'ε', 'τ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 5171);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λσισ.
pub(crate) fn n1832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1801);
	s.is_empty()
}

/// Words ending -νσισ.
pub(crate) fn n1833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1802);
	s.is_empty()
}

/// Words ending -οσισ.
pub(crate) fn n1834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1803);
	match s {
		[] => true,
		['δ', s @ ..] => {
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5172);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5173);
					match s {
						[] => true,
						['π', 'α', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6125);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρσισ.
pub(crate) fn n1835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1804);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3541);
			match s {
				[] => true,
				['θ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 5174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3542);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σσισ.
pub(crate) fn n1836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1805);
	match s {
		[] => true,
		['ω', 'λ', 'γ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 3543);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υσισ.
pub(crate) fn n1837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1806);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3544);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 3545);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3546);
			match s {
				[] => true,
				['τ', 'υ', s @ ..] => {
					super::push(out, 5175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3547);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5176);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6126);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 6127);
							match s {
								[] => true,
								['α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 6571);
											s.is_empty()
										}
										['π', 'ε', '\u{1}', s @ ..] => {
											super::push(out, 6572);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 6128);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 6129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 5177);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5178);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5179);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5180);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3548);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5181);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5182);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5183);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3549);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 5184);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3550);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 5186);
					s.is_empty()
				}
				['ο', 'π', 'υ', s @ ..] => {
					super::push(out, 5187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 3551);
			match s {
				[] => true,
				['γ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5188);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ῥ', s @ ..] => {
			super::push(out, 3552);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωσισ.
pub(crate) fn n1838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1807);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3553);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3554);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 5189);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 5190);
					s.is_empty()
				}
				['λ', 'ο', 'κ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5191);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 5192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3555);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5193);
					match s {
						[] => true,
						['τ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 6130);
							s.is_empty()
						}
						['ψ', '\u{1}', s @ ..] => {
							super::push(out, 6131);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5194);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 5195);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 5196);
					match s {
						[] => true,
						['κ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6132);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3556);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5197);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 5198);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 3557);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5199);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6133);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 5200);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5201);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3558);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3559);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5202);
					match s {
						[] => true,
						['δ', 'α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 6135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['β', s @ ..] => {
					super::push(out, 5203);
					match s {
						[] => true,
						['α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 5204);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 5205);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5206);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5207);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 5208);
					match s {
						[] => true,
						['υ', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 6137);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 5209);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6138);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6139);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 3560);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 3561);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 5210);
					s.is_empty()
				}
				['π', 'ι', s @ ..] => {
					super::push(out, 5211);
					match s {
						[] => true,
						['τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 6140);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3562);
			match s {
				[] => true,
				['ρ', 'ο', 'μ', 'α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 5212);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 3563);
			match s {
				[] => true,
				['υ', 'ψ', '\u{1}', s @ ..] => {
					super::push(out, 5213);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηρισκοσ.
pub(crate) fn n6366(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6194);
	s.is_empty()
}

/// Words ending -υρισκοσ.
pub(crate) fn n6367(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6195);
	s.is_empty()
}

/// Words ending -φρισκοσ.
pub(crate) fn n6368(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 6196);
	s.is_empty()
}

/// Words ending -οσκοσ.
pub(crate) fn n3879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3798);
	s.is_empty()
}

/// Words ending -υκοσ.
pub(crate) fn n1949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1917);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3799);
			s.is_empty()
		}
		['ε', 'λ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 3800);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 3801);
			match s {
				[] => true,
				['ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 5347);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', 'ρ', 'β', '\u{1}', s @ ..] => {
			super::push(out, 3802);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3803);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -βαλλοσ.
pub(crate) fn n5513(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5372);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 6206);
			match s {
				[] => true,
				['ρ', 'ε', 'π', 'ω', 'ν', 'α', 'τ', 'α', 'μ', 'ι', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 6485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 6207);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 6208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιαλλοσ.
pub(crate) fn n5517(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5376);
	s.is_empty()
}

/// Words ending -καλλοσ.
pub(crate) fn n5518(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5377);
	s.is_empty()
}

/// Words ending -μαλλοσ.
pub(crate) fn n5519(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5378);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 6209);
			match s {
				[] => true,
				['θ', 'ι', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 6486);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταλλοσ.
pub(crate) fn n5520(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5379);
	s.is_empty()
}

/// Words ending -φαλλοσ.
pub(crate) fn n5521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5380);
	match s {
		[] => true,
		['υ', 'θ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 6210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ελλοσ.
pub(crate) fn n3926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3844);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 5381);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιλλοσ.
pub(crate) fn n3927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3845);
	s.is_empty()
}

/// Words ending -υλλοσ.
pub(crate) fn n3928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3846);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 5382);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 6211);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 5383);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ολοσ.
pub(crate) fn n1959(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1927);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 3847);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5384);
					match s {
						[] => true,
						['ζ', '\u{1}', s @ ..] => {
							super::push(out, 6212);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 5385);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5386);
					match s {
						[] => true,
						['ν', 'η', 'γ', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 6213);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3848);
			match s {
				[] => true,
				['σ', 'α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 5387);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3849);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 5388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3850);
			match s {
				[] => true,
				['σ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 5389);
					s.is_empty()
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 3851);
			match s {
				[] => true,
				['ι', 'σ', 'α', 'ρ', 'κ', 'γ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 5390);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τλοσ.
pub(crate) fn n1960(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1928);
	match s {
		[] => true,
		['ν', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3852);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υλοσ.
pub(crate) fn n1961(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1929);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3853);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 5391);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 3854);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 3855);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 3856);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 3857);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3858);
			match s {
				[] => true,
				['β', 'μ', s @ ..] => {
					super::push(out, 5392);
					s.is_empty()
				}
				['δ', 'ο', s @ ..] => {
					super::push(out, 5393);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'η', s @ ..] => {
			super::push(out, 3859);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 3860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5394);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5395);
					match s {
						[] => true,
						['π', '\u{1}', s @ ..] => {
							super::push(out, 6214);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5396);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3861);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 3862);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χλοσ.
pub(crate) fn n1962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1930);
	s.is_empty()
}

/// Words ending -ωλοσ.
pub(crate) fn n1963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1931);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3863);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σμοσ.
pub(crate) fn n1974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1942);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3893);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 5419);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 5420);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 5421);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 5422);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6225);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 6226);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 6227);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 5423);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5424);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 5425);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5426);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 5427);
					match s {
						[] => true,
						['σ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 6487);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 6488);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 5428);
					match s {
						[] => true,
						['α', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 6228);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 5429);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 5430);
					match s {
						[] => true,
						['υ', 'σ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 6229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3894);
			match s {
				[] => true,
				['δ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6230);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 6231);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6232);
							match s {
								[] => true,
								['τ', 'ν', 'α', 'μ', 'ι', '\u{1}', s @ ..] => {
									super::push(out, 6489);
									s.is_empty()
								}
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
			super::push(out, 3895);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3896);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5431);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 5432);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 5433);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 6233);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6234);
							match s {
								[] => true,
								['λ', s @ ..] => {
									match s {
										[] => true,
										['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
											super::push(out, 6575);
											s.is_empty()
										}
										['λ', 'υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 6576);
											s.is_empty()
										}
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
					super::push(out, 5434);
					match s {
						[] => true,
						['ο', 'ῥ', '\u{1}', s @ ..] => {
							super::push(out, 6235);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 5435);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 5436);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5437);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6236);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6490);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 6491);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6492);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 6237);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6238);
							match s {
								[] => true,
								['ο', s @ ..] => {
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 6577);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 6578);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 6493);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 5438);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6239);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 6240);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 6241);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 5439);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5440);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6242);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 6243);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 6244);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 6245);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 6494);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 6495);
									match s {
										[] => true,
										['χ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
											super::push(out, 6579);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 6246);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 6247);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 6248);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 5441);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 5442);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6249);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 6250);
							match s {
								[] => true,
								['τ', 'σ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 6496);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 6251);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 6497);
									s.is_empty()
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 6252);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 6253);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 5443);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6254);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6498);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 6255);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 6256);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 6257);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 6258);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 5444);
					s.is_empty()
				}
				['φ', s @ ..] => {
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
		['υ', s @ ..] => {
			super::push(out, 3897);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 5447);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 5448);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 5449);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ινοσ.
pub(crate) fn n1984(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1952);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3931);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 3932);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 3933);
			match s {
				[] => true,
				['ρ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 5485);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3934);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 3935);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5486);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 6267);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 6268);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 5487);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 5488);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6269);
							s.is_empty()
						}
						['ι', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 6270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3936);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5489);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 6271);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 6272);
							s.is_empty()
						}
						['χ', s @ ..] => {
							super::push(out, 6273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 5490);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 5491);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 5492);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 3937);
			match s {
				[] => true,
				['α', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 6274);
							match s {
								[] => true,
								['υ', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 6505);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 6275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 3938);
			match s {
				[] => true,
				['υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 5493);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3939);
			match s {
				[] => true,
				['χ', 'σ', 'υ', s @ ..] => {
					super::push(out, 5494);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 3940);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3941);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 5495);
					match s {
						[] => true,
						['τ', 'κ', 'υ', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 6276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 5496);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 5497);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 6277);
							match s {
								[] => true,
								['κ', '\u{1}', s @ ..] => {
									super::push(out, 6506);
									s.is_empty()
								}
								['λ', 'η', 'θ', '\u{1}', s @ ..] => {
									super::push(out, 6507);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 5498);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3942);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 5499);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 5500);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 5501);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 6278);
							s.is_empty()
						}
						['ο', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 6508);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 6509);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 5502);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 5503);
					match s {
						[] => true,
						['ε', 'ν', 'ι', 'α', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 6279);
							s.is_empty()
						}
						['ι', 'α', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 6280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 5504);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3943);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 3944);
			s.is_empty()
		}
		['ῥ', s @ ..] => {
			super::push(out, 3945);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ννοσ.
pub(crate) fn n1987(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1955);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3952);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3953);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3954);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 5509);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 3955);
			match s {
				[] => true,
				['θ', '\u{1}', s @ ..] => {
					super::push(out, 5510);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ονοσ.
pub(crate) fn n1988(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1956);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 3956);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3957);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 3958);
			match s {
				[] => true,
				['α', 'ι', 'δ', 'ι', 'χ', 'ρ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 5511);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3959);
			match s {
				[] => true,
				['κ', 'ο', 'ν', s @ ..] => {
					super::push(out, 5512);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 3960);
			match s {
				[] => true,
				['ο', 'σ', s @ ..] => {
					super::push(out, 5513);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πνοσ.
pub(crate) fn n1989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1957);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3961);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 3962);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3963);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρνοσ.
pub(crate) fn n1990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1958);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3964);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 5514);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 3965);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3966);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3967);
			match s {
				[] => true,
				['β', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 5515);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνοσ.
pub(crate) fn n1991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1959);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3968);
			match s {
				[] => true,
				['ρ', 'υ', s @ ..] => {
					super::push(out, 5516);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 3969);
			s.is_empty()
		}
		['δ', 'ν', 'ι', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 3970);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 3971);
			match s {
				[] => true,
				['ι', 'β', '\u{1}', s @ ..] => {
					super::push(out, 5517);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3972);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 3973);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3974);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χνοσ.
pub(crate) fn n1992(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1960);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3975);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3976);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3977);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωνοσ.
pub(crate) fn n1993(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1961);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 3978);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 3979);
			s.is_empty()
		}
		_ => false,
	}
}

