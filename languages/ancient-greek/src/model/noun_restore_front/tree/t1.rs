use flexion::Path;

/// Words ending -ισ.
pub(crate) fn n213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 213);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2285);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2286);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2287);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2288);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 3098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['η', s @ ..] => {
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3099);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3100);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2289);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2290);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2291);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1094);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2292);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2293);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2294);
					match s {
						[] => true,
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 3101);
							s.is_empty()
						}
						['τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3102);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1095);
			match s {
				[] => true,
				['δ', 'ω', s @ ..] => {
					super::push(out, 2295);
					s.is_empty()
				}
				['τ', 'ο', s @ ..] => {
					super::push(out, 2296);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1096);
			match s {
				[] => true,
				['τ', 'α', 'χ', 'ο', 'ρ', 'α', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2297);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1097);
			match s {
				[] => true,
				['α', 'κ', s @ ..] => {
					super::push(out, 2298);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2299);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1098);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2300);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2301);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2302);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3103);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2303);
					match s {
						[] => true,
						['η', '\u{1}', s @ ..] => {
							super::push(out, 3104);
							s.is_empty()
						}
						['υ', 'β', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3105);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 2304);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1100);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1101);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κσ.
pub(crate) fn n214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 214);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['ι', 'ω', 'ρ', 'η', 'μ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2305);
					s.is_empty()
				}
				['λ', s @ ..] => {
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3106);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 3107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2306);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['δ', s @ ..] => {
							match s {
								[] => true,
								['α', 'λ', s @ ..] => {
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3460);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3461);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 3378);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', 'ε', 'ρ', 'δ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2308);
					s.is_empty()
				}
				['ρ', 'φ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				['φ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3108);
							s.is_empty()
						}
						['η', '\u{1}', s @ ..] => {
							super::push(out, 3109);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1103);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1104);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1105);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2310);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3110);
							s.is_empty()
						}
						['γ', s @ ..] => {
							super::push(out, 3111);
							s.is_empty()
						}
						['δ', s @ ..] => {
							super::push(out, 3112);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 3113);
							s.is_empty()
						}
						['σ', '\u{1}', s @ ..] => {
							super::push(out, 3114);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 2311);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 2312);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 2313);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2314);
					match s {
						[] => true,
						['ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3115);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1106);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1107);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2315);
					match s {
						[] => true,
						['π', 'ι', 'ο', s @ ..] => {
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3379);
									s.is_empty()
								}
								['κ', s @ ..] => {
									super::push(out, 3380);
									s.is_empty()
								}
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
			super::push(out, 1108);
			match s {
				[] => true,
				['λ', 'α', '\u{1}', s @ ..] => {
					super::push(out, 2316);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2317);
					s.is_empty()
				}
				['ρ', 'ο', s @ ..] => {
					super::push(out, 2318);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2319);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3116);
							match s {
								[] => true,
								['λ', 'ι', s @ ..] => {
									super::push(out, 3381);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 3117);
							match s {
								[] => true,
								['σ', '\u{1}', s @ ..] => {
									super::push(out, 3382);
									s.is_empty()
								}
								['τ', 'ο', 'μ', 'ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3383);
									s.is_empty()
								}
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
			super::push(out, 1109);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σιφοσ.
pub(crate) fn n2410(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['μ', 'ο', s @ ..] => {
			super::push(out, 3118);
			s.is_empty()
		}
		['τ', 'η', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 3119);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πσ.
pub(crate) fn n217(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 217);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2324);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3120);
							s.is_empty()
						}
						_ => false,
					}
				}
				['θ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2325);
					s.is_empty()
				}
				['ρ', 'α', s @ ..] => {
					super::push(out, 2326);
					s.is_empty()
				}
				['σ', 'μ', s @ ..] => {
					super::push(out, 2327);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1116);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1117);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 2328);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1120);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1121);
			match s {
				[] => true,
				['ρ', 'ι', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2330);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τσ.
pub(crate) fn n218(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 218);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1122);
			match s {
				[] => true,
				['γ', 'ε', s @ ..] => {
					super::push(out, 2331);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 2332);
					s.is_empty()
				}
				['θ', 'μ', 'ο', 'ν', s @ ..] => {
					super::push(out, 2333);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2334);
					s.is_empty()
				}
				['λ', 'ι', s @ ..] => {
					super::push(out, 2335);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2336);
					s.is_empty()
				}
				['σ', s @ ..] => {
					super::push(out, 2337);
					match s {
						[] => true,
						['ι', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3121);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', 'ω', s @ ..] => {
					super::push(out, 2338);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2339);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 2340);
					match s {
						[] => true,
						['υ', 'λ', 'η', s @ ..] => {
							super::push(out, 3122);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1123);
			match s {
				[] => true,
				['γ', 'α', s @ ..] => {
					super::push(out, 2341);
					match s {
						[] => true,
						['σ', 'τ', 'ρ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3123);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1124);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1125);
			match s {
				[] => true,
				['β', s @ ..] => {
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 3124);
							s.is_empty()
						}
						['ι', '\u{1}', s @ ..] => {
							super::push(out, 3125);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 2342);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1126);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1127);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2343);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3126);
							match s {
								[] => true,
								['γ', 'ο', 'υ', 'ρ', 'ι', 'α', '\u{1}', s @ ..] => {
									super::push(out, 3384);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							match s {
								[] => true,
								['ι', 'α', s @ ..] => {
									super::push(out, 3385);
									s.is_empty()
								}
								['ο', 'σ', '\u{1}', s @ ..] => {
									super::push(out, 3386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2344);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 2345);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 2346);
					match s {
						[] => true,
						['μ', 'α', 'τ', s @ ..] => {
							super::push(out, 3127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1128);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2347);
					match s {
						[] => true,
						['ι', 'τ', 'ι', s @ ..] => {
							super::push(out, 3128);
							s.is_empty()
						}
						['ο', 'σ', s @ ..] => {
							super::push(out, 3129);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 2348);
					s.is_empty()
				}
				['π', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3130);
							s.is_empty()
						}
						['π', 'η', s @ ..] => {
							super::push(out, 3131);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 3132);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2349);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υσ.
pub(crate) fn n219(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 219);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 1129);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2350);
					s.is_empty()
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1130);
			match s {
				[] => true,
				['γ', s @ ..] => {
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3133);
							s.is_empty()
						}
						['ρ', 'α', 'φ', s @ ..] => {
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3387);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3388);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', 'ρ', 'η', s @ ..] => {
					super::push(out, 2351);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 2352);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1131);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2353);
					match s {
						[] => true,
						['μ', 'ι', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3134);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 2354);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3135);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2355);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 1132);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2356);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1133);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2357);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 2358);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 2359);
					match s {
						[] => true,
						['α', 'θ', s @ ..] => {
							super::push(out, 3136);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2360);
					match s {
						[] => true,
						['α', s @ ..] => {
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3389);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 3390);
									s.is_empty()
								}
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
			super::push(out, 1134);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2361);
					match s {
						[] => true,
						['γ', 'ω', 'γ', 'η', '\u{1}', s @ ..] => {
							super::push(out, 3137);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3138);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 3139);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 3140);
							match s {
								[] => true,
								['α', 'σ', 'τ', 'ρ', 'ο', 'φ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 3391);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 2362);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2363);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2364);
					match s {
						[] => true,
						['α', 'γ', s @ ..] => {
							super::push(out, 3141);
							match s {
								[] => true,
								['μ', 'α', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3462);
											s.is_empty()
										}
										['τ', s @ ..] => {
											super::push(out, 3463);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 3142);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2365);
					match s {
						[] => true,
						['α', '\u{1}', s @ ..] => {
							super::push(out, 3143);
							s.is_empty()
						}
						['κ', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1136);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2366);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φσ.
pub(crate) fn n220(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 220);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1137);
			match s {
				[] => true,
				['γ', s @ ..] => {
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3145);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 3146);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 3147);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2367);
					match s {
						[] => true,
						['ρ', 'ι', s @ ..] => {
							super::push(out, 3148);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2368);
					s.is_empty()
				}
				['λ', s @ ..] => {
					super::push(out, 2369);
					s.is_empty()
				}
				['ξ', '\u{1}', s @ ..] => {
					super::push(out, 2370);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1138);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 1139);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1140);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1141);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 1142);
			match s {
				[] => true,
				['ρ', 'α', s @ ..] => {
					super::push(out, 2371);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3149);
							s.is_empty()
						}
						['ι', 'ν', 'α', '\u{1}', s @ ..] => {
							super::push(out, 3150);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χσ.
pub(crate) fn n221(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 221);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1143);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1144);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 1145);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2372);
					match s {
						[] => true,
						['ε', 'λ', 'α', 'ι', 'ο', 'ν', '\u{1}', s @ ..] => {
							super::push(out, 3151);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2373);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1146);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωσ.
pub(crate) fn n222(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 222);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1147);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1148);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2374);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1149);
			match s {
				[] => true,
				['ρ', 'ο', 'ν', 'ι', 'σ', 'τ', s @ ..] => {
					super::push(out, 2375);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word σοπιογυρτ.
pub(crate) fn n3565(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3395);
	s.is_empty()
}

/// The word νοινογυρτ.
pub(crate) fn n3566(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3396);
	s.is_empty()
}

/// The word νωγυρτ.
pub(crate) fn n3310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3163);
	s.is_empty()
}

/// Words ending -πυρτ.
pub(crate) fn n2502(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2410);
	match s {
		[] => true,
		['α', '\u{1}', s @ ..] => {
			super::push(out, 3164);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τυρτ.
pub(crate) fn n2503(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2411);
	s.is_empty()
}

/// Words ending -φυρτ.
pub(crate) fn n2504(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2412);
	match s {
		[] => true,
		['ο', 'σ', '\u{1}', s @ ..] => {
			super::push(out, 3165);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωρτ.
pub(crate) fn n1189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1176);
	s.is_empty()
}

/// Words ending -υτ.
pub(crate) fn n232(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 232);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1177);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1178);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2413);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1179);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1180);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 1181);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2414);
					s.is_empty()
				}
				['β', s @ ..] => {
					super::push(out, 2415);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2416);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 2417);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2418);
					s.is_empty()
				}
				['ρ', s @ ..] => {
					super::push(out, 2419);
					s.is_empty()
				}
				['σ', 'η', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 2420);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1182);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 1183);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2421);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωτ.
pub(crate) fn n233(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 233);
	s.is_empty()
}

/// Words ending -ψυ.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1208);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1209);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 1210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υφ.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1240);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 1241);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2469);
					match s {
						[] => true,
						['κ', 'τ', s @ ..] => {
							super::push(out, 3192);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2470);
					s.is_empty()
				}
				['λ', 'α', s @ ..] => {
					super::push(out, 2471);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 2472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1242);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1243);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2473);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 3193);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 3194);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 2474);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 2475);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 3195);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 3196);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 3197);
							s.is_empty()
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 2476);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 2477);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1244);
			match s {
				[] => true,
				['λ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 2478);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ατνα.
pub(crate) fn n1521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1493);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 2640);
			s.is_empty()
		}
		['π', 'ο', 'δ', s @ ..] => {
			super::push(out, 2641);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιτνα.
pub(crate) fn n1522(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1494);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2642);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 2643);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 2644);
			s.is_empty()
		}
		['κ', 'υ', s @ ..] => {
			super::push(out, 2645);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 2646);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2647);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 2648);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 2649);
			match s {
				[] => true,
				['ν', '\u{1}', s @ ..] => {
					super::push(out, 3254);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2650);
			match s {
				[] => true,
				['α', 'ρ', s @ ..] => {
					super::push(out, 3255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 2651);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2652);
			match s {
				[] => true,
				['τ', s @ ..] => {
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 3426);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 3427);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 2653);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ολτνα.
pub(crate) fn n2764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2654);
	s.is_empty()
}

/// Words ending -οτνα.
pub(crate) fn n1524(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1496);
	s.is_empty()
}

/// The word νορτνα.
pub(crate) fn n1525(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1497);
	s.is_empty()
}

/// Words ending -υτνα.
pub(crate) fn n1526(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1498);
	s.is_empty()
}

/// Words ending -ωτνα.
pub(crate) fn n1527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1499);
	s.is_empty()
}

/// Words ending -ωνα.
pub(crate) fn n375(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 375);
	s.is_empty()
}

/// Words ending -ηχρα.
pub(crate) fn n1597(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1560);
	match s {
		[] => true,
		['γ', 'ε', s @ ..] => {
			super::push(out, 2696);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιχρα.
pub(crate) fn n1598(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1561);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2697);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 2698);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 2699);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3262);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 3263);
					match s {
						[] => true,
						['α', 'κ', 'ο', 'ν', 'ο', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3435);
							s.is_empty()
						}
						['κ', 'α', 'σ', 'τ', 'η', 'σ', '\u{1}', s @ ..] => {
							super::push(out, 3436);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 2700);
			s.is_empty()
		}
		['ζ', s @ ..] => {
			super::push(out, 2701);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 2702);
			match s {
				[] => true,
				['α', 'γ', 'ε', 'ι', 'ρ', 'ο', 'σ', '\u{1}', s @ ..] => {
					super::push(out, 3264);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 2703);
			s.is_empty()
		}
		['ρ', s @ ..] => {
			super::push(out, 2704);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 2705);
			s.is_empty()
		}
		['τ', s @ ..] => {
			super::push(out, 2706);
			match s {
				[] => true,
				['ε', 'λ', s @ ..] => {
					super::push(out, 3265);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωχρα.
pub(crate) fn n1600(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1563);
	s.is_empty()
}

/// Words ending -ωρα.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	s.is_empty()
}

