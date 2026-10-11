use flexion::Path;

/// Words ending -ρινοσ.
pub(crate) fn n1181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1165);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2041);
			match s {
				[] => true,
				['β', 'α', 'ν', 'ν', 'ι', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2774);
					s.is_empty()
				}
				['ε', '\u{1}', s @ ..] => {
					super::push(out, 2775);
					s.is_empty()
				}
				['λ', '\u{1}', s @ ..] => {
					super::push(out, 2776);
					s.is_empty()
				}
				['μ', 'ρ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2777);
					s.is_empty()
				}
				['φ', '\u{1}', s @ ..] => {
					super::push(out, 2778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 2042);
			match s {
				[] => true,
				['μ', 'υ', 'σ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2779);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', 'α', 'ν', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 2043);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 2044);
			match s {
				[] => true,
				['ν', 'α', 'ξ', 'ε', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2780);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2045);
			match s {
				[] => true,
				['α', '\u{1}', s @ ..] => {
					super::push(out, 2781);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 2782);
					match s {
						[] => true,
						['φ', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 3273);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2783);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2784);
					match s {
						[] => true,
						['ι', 'σ', 'σ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3274);
							s.is_empty()
						}
						['κ', 'υ', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3275);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2046);
			match s {
				[] => true,
				['α', 'β', 'α', 'λ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2785);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2047);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2786);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 2787);
					match s {
						[] => true,
						['κ', 'ο', 'λ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3276);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2788);
					match s {
						[] => true,
						['φ', 'π', 'α', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2048);
			match s {
				[] => true,
				['υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2789);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2049);
			match s {
				[] => true,
				['ι', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2790);
					s.is_empty()
				}
				['σ', 'υ', 'α', 'λ', 'α', 'β', '\u{1}', s @ ..] => {
					super::push(out, 2791);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2050);
			match s {
				[] => true,
				['α', 'λ', 'α', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2792);
					s.is_empty()
				}
				['γ', 'ρ', 'α', 'θ', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2793);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2794);
					match s {
						[] => true,
						['λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3278);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 2795);
					match s {
						[] => true,
						['ι', 'φ', '\u{1}', s @ ..] => {
							super::push(out, 3279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', 'ι', 'λ', 'α', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2796);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2797);
					match s {
						[] => true,
						['α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3280);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2798);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3281);
							s.is_empty()
						}
						['ι', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 3282);
							s.is_empty()
						}
						['υ', 'ο', 'β', '\u{1}', s @ ..] => {
							super::push(out, 3283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2051);
			match s {
				[] => true,
				['π', 'ο', 'ν', 'ι', 'θ', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 2799);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σινοσ.
pub(crate) fn n1182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1166);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2052);
			match s {
				[] => true,
				['π', 'ρ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2800);
					s.is_empty()
				}
				['ρ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 2801);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'υ', 'β', '\u{1}', s @ ..] => {
			super::push(out, 2053);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2054);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3284);
							s.is_empty()
						}
						['ρ', 'α', 'π', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3285);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'β', '\u{1}', s @ ..] => {
					super::push(out, 2802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2055);
			match s {
				[] => true,
				['ρ', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 2803);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τινοσ.
pub(crate) fn n1183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1167);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2056);
			match s {
				[] => true,
				['ε', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2804);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2805);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3286);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2806);
					match s {
						[] => true,
						['α', 'ρ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3287);
							s.is_empty()
						}
						['η', 'λ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3288);
							s.is_empty()
						}
						['ι', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3289);
							s.is_empty()
						}
						['μ', 'α', 'ῥ', '\u{1}', s @ ..] => {
							super::push(out, 3290);
							s.is_empty()
						}
						['ω', s @ ..] => {
							super::push(out, 3291);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2807);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2057);
			match s {
				[] => true,
				['κ', 'υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2808);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2058);
			match s {
				[] => true,
				['λ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2809);
					s.is_empty()
				}
				['μ', 'ι', 'ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2810);
					s.is_empty()
				}
				['σ', '\u{1}', s @ ..] => {
					super::push(out, 2811);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2059);
			match s {
				[] => true,
				['α', 'λ', s @ ..] => {
					super::push(out, 2812);
					match s {
						[] => true,
						['α', 'γ', 'υ', 'ξ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 3292);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2060);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2813);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 3293);
							s.is_empty()
						}
						['ρ', 'α', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3294);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 3295);
							match s {
								[] => true,
								['ε', 'λ', 'ε', 'σ', 'υ', 'ρ', 'χ', '\u{1}', s @ ..] => {
									super::push(out, 3547);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', 'κ', 'α', 'ρ', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 2814);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2061);
			match s {
				[] => true,
				['υ', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 2815);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2062);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2063);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 2064);
			match s {
				[] => true,
				['ν', 'α', 'β', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 2816);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υινοσ.
pub(crate) fn n1184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1168);
	match s {
		[] => true,
		['θ', '\u{1}', s @ ..] => {
			super::push(out, 2065);
			s.is_empty()
		}
		['ξ', 'ο', '\u{1}', s @ ..] => {
			super::push(out, 2066);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2067);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 2817);
					s.is_empty()
				}
				['δ', '\u{1}', s @ ..] => {
					super::push(out, 2818);
					s.is_empty()
				}
				['θ', '\u{1}', s @ ..] => {
					super::push(out, 2819);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2068);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2069);
			match s {
				[] => true,
				['ι', 'π', 'ι', 'α', 'μ', 'α', 'χ', '\u{1}', s @ ..] => {
					super::push(out, 2820);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φινοσ.
pub(crate) fn n1185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1169);
	match s {
		[] => true,
		['α', 'λ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 2070);
			s.is_empty()
		}
		['ι', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 2071);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χινοσ.
pub(crate) fn n1186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1170);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2072);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2073);
			match s {
				[] => true,
				['ν', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 2821);
					s.is_empty()
				}
				['ο', 'ψ', 'μ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2822);
					s.is_empty()
				}
				['ρ', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 2823);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψινοσ.
pub(crate) fn n1187(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1171);
	match s {
		[] => true,
		['υ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 2074);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτεροσ.
pub(crate) fn n2190(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2142);
	match s {
		[] => true,
		['α', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 2865);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2866);
			match s {
				[] => true,
				['δ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 3334);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3335);
					s.is_empty()
				}
				['μ', 'ι', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 3336);
					s.is_empty()
				}
				['ν', 'α', 'π', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3337);
					s.is_empty()
				}
				['ξ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3338);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3339);
					match s {
						[] => true,
						['γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3570);
							s.is_empty()
						}
						['τ', 'ο', 'λ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3571);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2867);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3340);
					match s {
						[] => true,
						['δ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3572);
							s.is_empty()
						}
						['τ', 'ν', 'υ', 'μ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3573);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2868);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2869);
			match s {
				[] => true,
				['ι', s @ ..] => {
					match s {
						[] => true,
						['κ', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3574);
							s.is_empty()
						}
						['λ', 'ε', 'φ', 'ω', '\u{1}', s @ ..] => {
							super::push(out, 3575);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2870);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3341);
					match s {
						[] => true,
						['κ', 'ι', '\u{1}', s @ ..] => {
							super::push(out, 3576);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', 'ε', 'λ', 'α', 'χ', '\u{1}', s @ ..] => {
			super::push(out, 2871);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2872);
			match s {
				[] => true,
				['α', 'θ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3342);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3343);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 3344);
					match s {
						[] => true,
						['π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3577);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'χ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3345);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', 'ι', '\u{1}', s @ ..] => {
			super::push(out, 2873);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2874);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 2875);
			match s {
				[] => true,
				['υ', 'λ', 'γ', 'ο', 'μ', 'α', 'δ', 'ρ', 'α', 'κ', 'ο', 'τ', 'σ', 'ι', 'ρ', 'π', 'ο', 'ν', 'ι', 'μ', 'υ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3346);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρητικοσ.
pub(crate) fn n2645(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2579);
	match s {
		[] => true,
		['υ', 'ο', 'ι', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 3085);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τητικοσ.
pub(crate) fn n2646(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2580);
	match s {
		[] => true,
		['α', 'π', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 3086);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κατηχητικοσ.
pub(crate) fn n2647(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2581);
	s.is_empty()
}

/// Words ending -ιτικοσ.
pub(crate) fn n1911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1868);
	match s {
		[] => true,
		['ζ', 'ε', 'π', 'α', 'ρ', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 2582);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2583);
			match s {
				[] => true,
				['ο', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3087);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2584);
			match s {
				[] => true,
				['κ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3088);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', 'ε', 'λ', '\u{1}', s @ ..] => {
			super::push(out, 2585);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κτικοσ.
pub(crate) fn n1912(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1869);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2586);
			match s {
				[] => true,
				['δ', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3089);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3090);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3091);
					match s {
						[] => true,
						['π', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3600);
									s.is_empty()
								}
								['ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3601);
									s.is_empty()
								}
								['μ', s @ ..] => {
									super::push(out, 3602);
									match s {
										[] => true,
										['υ', 'σ', '\u{1}', s @ ..] => {
											super::push(out, 3635);
											s.is_empty()
										}
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
					super::push(out, 3092);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3448);
							s.is_empty()
						}
						['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3449);
							s.is_empty()
						}
						['ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3450);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2587);
			match s {
				[] => true,
				['λ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3093);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2588);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 3094);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2589);
			match s {
				[] => true,
				['ε', 'δ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3095);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2590);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 2591);
			match s {
				[] => true,
				['ψ', '\u{1}', s @ ..] => {
					super::push(out, 3096);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λτικοσ.
pub(crate) fn n1913(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1870);
	match s {
		[] => true,
		['α', 'ι', 'φ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 2592);
			s.is_empty()
		}
		['ε', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 2593);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ντικοσ.
pub(crate) fn n1914(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1871);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2594);
			match s {
				[] => true,
				['λ', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3097);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 3098);
					s.is_empty()
				}
				['ρ', 'α', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 3099);
					s.is_empty()
				}
				['φ', 'ο', 'κ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3100);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2595);
			s.is_empty()
		}
		['υ', 'μ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 2596);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οτικοσ.
pub(crate) fn n1915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1872);
	s.is_empty()
}

/// Words ending -πτικοσ.
pub(crate) fn n1916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1873);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2597);
			match s {
				[] => true,
				['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3101);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2598);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 2599);
			match s {
				[] => true,
				['λ', 'ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3102);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2600);
			match s {
				[] => true,
				['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3103);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2601);
			match s {
				[] => true,
				['λ', 'α', 'κ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3104);
					s.is_empty()
				}
				['ρ', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3105);
					s.is_empty()
				}
				['τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3106);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρτικοσ.
pub(crate) fn n1917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1874);
	match s {
		[] => true,
		['α', 'θ', 'α', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 2602);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -στικοσ.
pub(crate) fn n1918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1875);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2603);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3107);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 3108);
					s.is_empty()
				}
				['ν', 'μ', 'υ', 'γ', '\u{1}', s @ ..] => {
					super::push(out, 3109);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3110);
					match s {
						[] => true,
						['δ', '\u{1}', s @ ..] => {
							super::push(out, 3451);
							s.is_empty()
						}
						['φ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3452);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'ν', 'α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 3111);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3112);
					match s {
						[] => true,
						['ε', 'κ', 'σ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3453);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2604);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2605);
			match s {
				[] => true,
				['ν', 'ω', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3113);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3114);
					match s {
						[] => true,
						['α', 'χ', 'υ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3454);
							s.is_empty()
						}
						['β', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3455);
							s.is_empty()
						}
						['η', 'τ', 'κ', 'α', 'ρ', 'α', 'χ', '\u{1}', s @ ..] => {
							super::push(out, 3456);
							s.is_empty()
						}
						['υ', 'μ', '\u{1}', s @ ..] => {
							super::push(out, 3457);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 2606);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3115);
					s.is_empty()
				}
				['γ', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3116);
					s.is_empty()
				}
				['ε', 'π', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3117);
					s.is_empty()
				}
				['κ', 'λ', 'ε', 'φ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3118);
					s.is_empty()
				}
				['μ', '\u{1}', s @ ..] => {
					super::push(out, 3119);
					s.is_empty()
				}
				['ο', 'κ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3120);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 2607);
			match s {
				[] => true,
				['ν', 'γ', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3121);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ττικοσ.
pub(crate) fn n1919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1876);
	match s {
		[] => true,
		['α', '\u{1}', s @ ..] => {
			super::push(out, 2608);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υτικοσ.
pub(crate) fn n1920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1877);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2609);
			match s {
				[] => true,
				['κ', 'α', 'μ', 'ρ', 'α', 'φ', '\u{1}', s @ ..] => {
					super::push(out, 3122);
					s.is_empty()
				}
				['π', 'α', 'ρ', 'ε', 'θ', '\u{1}', s @ ..] => {
					super::push(out, 3123);
					s.is_empty()
				}
				['ρ', 'τ', 'α', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3124);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3125);
					match s {
						[] => true,
						['υ', 'φ', 'μ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2610);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3126);
					s.is_empty()
				}
				['ι', 'π', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3127);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', '\u{1}', s @ ..] => {
			super::push(out, 2611);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωτικοσ.
pub(crate) fn n1921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1878);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 2612);
			match s {
				[] => true,
				['δ', 'ι', '\u{1}', s @ ..] => {
					super::push(out, 3128);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3129);
					s.is_empty()
				}
				['σ', 'η', 'ν', '\u{1}', s @ ..] => {
					super::push(out, 3130);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2613);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2614);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3131);
					s.is_empty()
				}
				['π', 'ο', 'κ', 'κ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3132);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ημενοσ.
pub(crate) fn n2025(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1979);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 2662);
			match s {
				[] => true,
				['α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3164);
					s.is_empty()
				}
				['χ', 'η', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3165);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2663);
			match s {
				[] => true,
				['α', 'λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3166);
					s.is_empty()
				}
				['β', 'ε', 'β', 'α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
					super::push(out, 3167);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 2664);
			match s {
				[] => true,
				['ο', 'δ', 'ο', 'κ', 'ω', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
					super::push(out, 3168);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2665);
			match s {
				[] => true,
				['ε', 'γ', 'ε', 'γ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3169);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2666);
			match s {
				[] => true,
				['η', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3170);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μμενοσ.
pub(crate) fn n2027(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1981);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2668);
			match s {
				[] => true,
				['κ', 'σ', 'ε', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3171);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 3172);
					match s {
						[] => true,
						['γ', 'ε', 'γ', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3480);
							s.is_empty()
						}
						['τ', 'σ', 'ε', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 2669);
			match s {
				[] => true,
				['λ', 'ι', 'ε', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3173);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', 'ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 2670);
			s.is_empty()
		}
		['υ', 'λ', 'α', 'κ', 'ε', 'κ', '\u{1}', s @ ..] => {
			super::push(out, 2671);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ομενοσ.
pub(crate) fn n2028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1982);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2672);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3174);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 2673);
			match s {
				[] => true,
				['ι', 'ε', 'μ', 'α', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3175);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 2674);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3176);
					match s {
						[] => true,
						['γ', 'α', 'ν', 'α', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3613);
									s.is_empty()
								}
								['τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3614);
									s.is_empty()
								}
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
			super::push(out, 2675);
			match s {
				[] => true,
				['ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3177);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 3178);
					match s {
						[] => true,
						['ι', 'λ', 'υ', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2676);
			match s {
				[] => true,
				['ι', 'ο', 'π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3179);
					s.is_empty()
				}
				['λ', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3180);
					s.is_empty()
				}
				['τ', 'κ', 'α', 'ν', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3181);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 2677);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3182);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3483);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3484);
							s.is_empty()
						}
						['κ', 'γ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3485);
							s.is_empty()
						}
						['υ', 'ε', 'κ', 'σ', 'α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3486);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3183);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 3487);
							match s {
								[] => true,
								['α', 'κ', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3615);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3488);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3184);
					match s {
						[] => true,
						['γ', 'ο', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 3489);
							s.is_empty()
						}
						['δ', 'ο', 'π', 'α', 'ρ', 'δ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3490);
							s.is_empty()
						}
						['θ', 'α', 'κ', 'ι', 'τ', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3491);
							s.is_empty()
						}
						['λ', 'α', 'ν', 'υ', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3492);
							s.is_empty()
						}
						['μ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3493);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3494);
							s.is_empty()
						}
						['π', 'λ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3495);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3496);
							s.is_empty()
						}
						['χ', 'ι', 'ε', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 3497);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', 'τ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3185);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 2678);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 3186);
					s.is_empty()
				}
				['υ', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3187);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2679);
			match s {
				[] => true,
				['α', 'λ', 'ι', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3188);
					s.is_empty()
				}
				['λ', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3189);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 2680);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3190);
					match s {
						[] => true,
						['ι', 'λ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3498);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 2681);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3191);
					match s {
						[] => true,
						['γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3499);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3192);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3500);
							match s {
								[] => true,
								['β', 'α', 'τ', 'ε', 'μ', '\u{1}', s @ ..] => {
									super::push(out, 3616);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 3501);
							match s {
								[] => true,
								['γ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3617);
									s.is_empty()
								}
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
			super::push(out, 2682);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3193);
					match s {
						[] => true,
						['θ', 'ν', 'υ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3502);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 3194);
					match s {
						[] => true,
						['ι', 'γ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3618);
									s.is_empty()
								}
								['α', 'ρ', 'α', 'π', '\u{1}', s @ ..] => {
									super::push(out, 3619);
									s.is_empty()
								}
								['ι', 'π', 'ε', '\u{1}', s @ ..] => {
									super::push(out, 3620);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3195);
					match s {
						[] => true,
						['γ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3503);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3196);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3197);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 2683);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 2684);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3198);
					match s {
						[] => true,
						['ρ', 'τ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3504);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3199);
					match s {
						[] => true,
						['λ', '\u{1}', s @ ..] => {
							super::push(out, 3505);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 3200);
					match s {
						[] => true,
						['α', 'λ', '\u{1}', s @ ..] => {
							super::push(out, 3506);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2685);
			match s {
				[] => true,
				['γ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3201);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3202);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3203);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['θ', 'α', 'κ', '\u{1}', s @ ..] => {
									super::push(out, 3621);
									s.is_empty()
								}
								['τ', 'ν', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3622);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 3507);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 3204);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 2686);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 3205);
					match s {
						[] => true,
						['γ', 'ο', 'λ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3508);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 3509);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3623);
									match s {
										[] => true,
										['ι', 'ο', 'π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3640);
											s.is_empty()
										}
										['π', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
											super::push(out, 3641);
											s.is_empty()
										}
										_ => false,
									}
								}
								['λ', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3624);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ι', 'ο', 'π', 'ο', 'θ', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3510);
							s.is_empty()
						}
						['λ', 'ε', 'γ', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3511);
							s.is_empty()
						}
						['π', 'α', 'γ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3512);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3513);
							match s {
								[] => true,
								['ρ', 'α', 'μ', 'α', '\u{1}', s @ ..] => super::t1::n3723(s, out),
								['σ', 'α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => super::t1::n3724(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t1::n3292(s, out),
				['σ', 'ι', 'λ', '\u{1}', s @ ..] => super::t1::n3293(s, out),
				['υ', s @ ..] => super::t1::n3294(s, out),
				_ => false,
			}
		}
		['τ', s @ ..] => super::t1::n2756(s, out),
		['υ', s @ ..] => super::t1::n2757(s, out),
		['φ', s @ ..] => super::t1::n2758(s, out),
		['χ', s @ ..] => super::t1::n2759(s, out),
		['ψ', s @ ..] => super::t1::n2760(s, out),
		_ => false,
	}
}

/// Words ending -αρμενοσ.
pub(crate) fn n2761(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 3220);
			match s {
				[] => true,
				['φ', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
					super::push(out, 3528);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', 'σ', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 3221);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σμενοσ.
pub(crate) fn n2030(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1984);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2692);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3222);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 3223);
					match s {
						[] => true,
						['ρ', 'ι', 'ε', 'ν', 'υ', 'ξ', '\u{1}', s @ ..] => {
							super::push(out, 3529);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', 'β', 'ε', 'β', '\u{1}', s @ ..] => {
					super::push(out, 3224);
					s.is_empty()
				}
				['λ', 'ε', 'λ', '\u{1}', s @ ..] => {
					super::push(out, 3225);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 3226);
					match s {
						[] => true,
						['ε', 'κ', 'σ', 'ε', 'ρ', s @ ..] => {
							super::push(out, 3530);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 2693);
			match s {
				[] => true,
				['ε', 'π', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3227);
					s.is_empty()
				}
				['θ', 'ι', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3228);
					s.is_empty()
				}
				['κ', 'ω', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3229);
					s.is_empty()
				}
				['ρ', 'α', 'χ', 'ε', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3230);
					s.is_empty()
				}
				['φ', 'η', 'ψ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3231);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμενοσ.
pub(crate) fn n2031(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1985);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2694);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2695);
			match s {
				[] => true,
				['ε', 'λ', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3531);
							s.is_empty()
						}
						['α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 3532);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', 'ο', '\u{1}', s @ ..] => {
					super::push(out, 3232);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 2696);
			match s {
				[] => true,
				['κ', 'ι', 'ε', 'δ', 'ο', 'π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3233);
					s.is_empty()
				}
				['π', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3234);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2697);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 3235);
					s.is_empty()
				}
				['δ', 'α', 'ν', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3236);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 3237);
					match s {
						[] => true,
						['η', 'ο', 'β', 'ο', 'π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3238);
					match s {
						[] => true,
						['ξ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3534);
							s.is_empty()
						}
						['σ', 'α', 'λ', 'π', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 3535);
							s.is_empty()
						}
						['τ', 'ν', 'α', 'ν', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 3536);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3239);
					match s {
						[] => true,
						['α', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 3537);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3538);
							match s {
								[] => true,
								['δ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3629);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', 'ο', 'ι', 'λ', 'ο', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3539);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 3240);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3540);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 3541);
							match s {
								[] => true,
								['γ', 'γ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3630);
									s.is_empty()
								}
								['μ', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3631);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 3241);
					match s {
						[] => true,
						['κ', 'ι', 'φ', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3542);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3242);
					match s {
						[] => true,
						['ν', 'α', 'ι', 'δ', '\u{1}', s @ ..] => {
							super::push(out, 3543);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 3243);
					match s {
						[] => true,
						['ω', 'μ', 'ι', 'τ', '\u{1}', s @ ..] => {
							super::push(out, 3544);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', 'σ', 'α', 'λ', 'ε', '\u{1}', s @ ..] => {
					super::push(out, 3244);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 3245);
					s.is_empty()
				}
				['χ', 'η', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3246);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', 'τ', 'ε', 'τ', '\u{1}', s @ ..] => {
			super::push(out, 2698);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2699);
			s.is_empty()
		}
		['τ', 'ρ', 'η', 'ξ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 2700);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωμενοσ.
pub(crate) fn n2032(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1986);
	match s {
		[] => true,
		['ζ', 'ε', 'ι', 'δ', '\u{1}', s @ ..] => {
			super::push(out, 2701);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 2702);
			match s {
				[] => true,
				['ξ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 3247);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', 'ι', 'ν', '\u{1}', s @ ..] => {
			super::push(out, 2703);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2704);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 2705);
			match s {
				[] => true,
				['σ', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
					super::push(out, 3248);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2706);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2707);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word αμαρτησομενοσ.
pub(crate) fn n3723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3625);
	s.is_empty()
}

/// The word καταστησομενοσ.
pub(crate) fn n3724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3626);
	s.is_empty()
}

/// Words ending -ισομενοσ.
pub(crate) fn n3292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3206);
	match s {
		[] => true,
		['ν', '\u{1}', s @ ..] => {
			super::push(out, 3514);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word λισσομενοσ.
pub(crate) fn n3293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3207);
	s.is_empty()
}

/// Words ending -υσομενοσ.
pub(crate) fn n3294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3208);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3515);
			s.is_empty()
		}
		['λ', '\u{1}', s @ ..] => {
			super::push(out, 3516);
			s.is_empty()
		}
		['ο', 'κ', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3517);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τομενοσ.
pub(crate) fn n2756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2687);
	match s {
		[] => true,
		['π', 'α', s @ ..] => {
			super::push(out, 3209);
			match s {
				[] => true,
				['θ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3518);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υομενοσ.
pub(crate) fn n2757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2688);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3210);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3211);
			match s {
				[] => true,
				['δ', 'ε', 'π', 'ο', 'τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3519);
					s.is_empty()
				}
				['τ', 'α', 'ρ', 'τ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3520);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 3212);
			match s {
				[] => true,
				['α', 'τ', 'α', 'κ', '\u{1}', s @ ..] => {
					super::push(out, 3521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3213);
			match s {
				[] => true,
				['ρ', 'κ', 'γ', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φομενοσ.
pub(crate) fn n2758(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2689);
	match s {
		[] => true,
		['α', 'ρ', 'γ', '\u{1}', s @ ..] => {
			super::push(out, 3214);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χομενοσ.
pub(crate) fn n2759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2690);
	match s {
		[] => true,
		['γ', 'α', 'π', 'α', '\u{1}', s @ ..] => {
			super::push(out, 3215);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 3216);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 3523);
					s.is_empty()
				}
				['ν', 'υ', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3524);
					s.is_empty()
				}
				['π', 'α', '\u{1}', s @ ..] => {
					super::push(out, 3525);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 3217);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 3218);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3526);
					match s {
						[] => true,
						['π', 'υ', '\u{1}', s @ ..] => {
							super::push(out, 3627);
							s.is_empty()
						}
						['τ', 'α', 'κ', 'ο', 'ρ', 'π', '\u{1}', s @ ..] => {
							super::push(out, 3628);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 3527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψομενοσ.
pub(crate) fn n2760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2691);
	match s {
		[] => true,
		['ο', 'ι', 'ρ', 'ε', 'π', '\u{1}', s @ ..] => {
			super::push(out, 3219);
			s.is_empty()
		}
		_ => false,
	}
}

