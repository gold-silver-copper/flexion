use flexion::Path;

/// Words ending -υσμαι.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1932);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2703);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3338);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3802);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n4083(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1125);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1933);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2704);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3339);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3803);
													s.is_empty()
												}
												['ν', s @ ..] => {
													super::push(out, 3804);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n4084(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 3805);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4085(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1934);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2705);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3340);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 437);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1935);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2706);
									s.is_empty()
								}
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
			super::push(out, 438);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1936);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2707);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3341);
											match s {
												[] => true,
												['κ', s @ ..] => {
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4086(s, out),
														['α', s @ ..] => super::t1::n4087(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1937);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2708);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3342);
											s.is_empty()
										}
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
					super::push(out, 1128);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1938);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2709);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3343);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1939);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2710);
									s.is_empty()
								}
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
			super::push(out, 439);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1940);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2711);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3344);
											s.is_empty()
										}
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
					super::push(out, 1131);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1941);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2712);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3345);
											s.is_empty()
										}
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
					super::push(out, 1132);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1942);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2713);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3346);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3806);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 440);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1133);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1943);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2714);
									match s {
										[] => true,
										['μ', s @ ..] => {
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
				['λ', s @ ..] => {
					super::push(out, 1134);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1944);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2715);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t1::n444(s, out),
		['ν', s @ ..] => {
			super::push(out, 442);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1137);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1947);
							match s {
								[] => true,
								['δ', s @ ..] => super::t1::n2728(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t1::n446(s, out),
		['ρ', s @ ..] => super::t1::n447(s, out),
		['σ', s @ ..] => super::t1::n448(s, out),
		['τ', s @ ..] => super::t1::n449(s, out),
		['χ', s @ ..] => super::t1::n450(s, out),
		_ => false,
	}
}

/// Words ending -ωσμαι.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 448);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1144);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1955);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1956);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 449);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1146);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1957);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2729);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3358);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3815);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4089(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 450);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1147);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1958);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2730);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2731);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3359);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 451);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1148);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1959);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2732);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3360);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 452);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1149);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ανακεκαυμαι.
pub(crate) fn n3833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3816);
	s.is_empty()
}

/// Words ending -αναπεπαυμαι.
pub(crate) fn n3834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3817);
	s.is_empty()
}

/// Words ending -επενδεδυμαι.
pub(crate) fn n3835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3818);
	s.is_empty()
}

/// Words ending -ευμαι.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1154);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1967);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1968);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2741);
									s.is_empty()
								}
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
			super::push(out, 458);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1969);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1970);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2742);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 3364);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3819);
													s.is_empty()
												}
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
							super::push(out, 1971);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2743);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3365);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3820);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1158);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1972);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2744);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1973);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2745);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3366);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3821);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1159);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1160);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1974);
							match s {
								[] => true,
								['π', 'ε', 'π', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3367);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 3368);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3822);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3369);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3823);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4090(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1975);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2746);
									s.is_empty()
								}
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
			super::push(out, 460);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1161);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1976);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2747);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3370);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3824);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1162);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1163);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1978);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2748);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3371);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3825);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1164);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1979);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
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
			super::push(out, 461);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['ο', s @ ..] => super::t1::n1986(s, out),
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1982);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2752);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3374);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n466(s, out),
		['ν', s @ ..] => super::t1::n467(s, out),
		['ξ', s @ ..] => super::t1::n468(s, out),
		['π', s @ ..] => super::t1::n469(s, out),
		['ρ', s @ ..] => super::t1::n470(s, out),
		['σ', s @ ..] => super::t1::n471(s, out),
		['τ', s @ ..] => super::t1::n472(s, out),
		['φ', s @ ..] => super::t1::n473(s, out),
		['χ', s @ ..] => super::t1::n474(s, out),
		_ => false,
	}
}

/// Words ending -καθειλκυμαι.
pub(crate) fn n3862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3845);
	s.is_empty()
}

/// Words ending -λελυμαι.
pub(crate) fn n1203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1200);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2010);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2777);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3399);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3846);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3847);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2778);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3400);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 3848);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4080);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 3849);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 4081);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3850);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 4082);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4175);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2779);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3401);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3851);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2780);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3402);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3852);
									s.is_empty()
								}
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
			super::push(out, 2011);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2781);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3403);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3853);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 4083);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4176);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2782);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3404);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3854);
									s.is_empty()
								}
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
			super::push(out, 2012);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2783);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2013);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2784);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2785);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3406);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3855);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επλυμαι.
pub(crate) fn n1204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1201);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 2014);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2786);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3407);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3856);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 4084);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κωλυμαι.
pub(crate) fn n1205(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1202);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2015);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2787);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μυμαι.
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2016);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2788);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3408);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3857);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 478);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1204);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2017);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2018);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2789);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νυμαι.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 480);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2019);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2790);
									s.is_empty()
								}
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
			super::push(out, 481);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1207);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2020);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2791);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3410);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3858);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n4102(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3411);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3859);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4103(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 482);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2021);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ουμαι.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 483);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2022);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2023);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2792);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3412);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 484);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1211);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2024);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2793);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 3413);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3860);
													s.is_empty()
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 3414);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3861);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4104(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3415);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3862);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n4105(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3863);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4106(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυμαι.
pub(crate) fn n101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 485);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2025);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2794);
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
			super::push(out, 486);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2026);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2795);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
				['ε', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2027);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2796);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3418);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3864);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4107(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3419);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3865);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n4108(s, out),
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
									super::push(out, 2797);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2798);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3420);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3866);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4109(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3421);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3867);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4110(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['φ', s @ ..] => {
											super::push(out, 3422);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2799);
									match s {
										[] => true,
										['π', s @ ..] => {
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
		['τ', s @ ..] => {
			super::push(out, 487);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2028);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 488);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2029);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -συμαι.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 101);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 489);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2030);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 490);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2031);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2800);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				['ε', s @ ..] => {
					super::push(out, 1219);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2032);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2801);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2033);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2802);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3425);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τυμαι.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1220);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2034);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2803);
									s.is_empty()
								}
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
			super::push(out, 492);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1221);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χυμαι.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1222);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2035);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2036);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2804);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3426);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2805);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3427);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3868);
													s.is_empty()
												}
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
							super::push(out, 2037);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2806);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2038);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2807);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3428);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 494);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1223);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2039);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2808);
									match s {
										[] => true,
										['τ', s @ ..] => {
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

/// Words ending -κεκλειδωμαι.
pub(crate) fn n3886(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3869);
	s.is_empty()
}

/// Words ending -ραφανιδωμαι.
pub(crate) fn n3887(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3870);
	s.is_empty()
}

/// Words ending -απηξιωμαι.
pub(crate) fn n2832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2820);
	s.is_empty()
}

/// Words ending -οιωμαι.
pub(crate) fn n510(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 507);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1239);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 2055);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ριωμαι.
pub(crate) fn n511(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 508);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 1240);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2056);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2821);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3435);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3871);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τιωμαι.
pub(crate) fn n512(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 509);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1241);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2057);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2822);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3436);
									s.is_empty()
								}
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
			super::push(out, 1242);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2058);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2823);
							match s {
								[] => true,
								['η', s @ ..] => {
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

/// Words ending -κωμαι.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 510);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1243);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 511);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2059);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2824);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3438);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 512);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2060);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2825);
									s.is_empty()
								}
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
			super::push(out, 513);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2061);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2826);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λωμαι.
pub(crate) fn n113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 514);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2062);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2827);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3439);
											s.is_empty()
										}
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
					super::push(out, 1248);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1249);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 515);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 1250);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2063);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 516);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1251);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2064);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2828);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3440);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 517);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1252);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2065);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2829);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3441);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3872);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 518);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2066);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2067);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1255);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2068);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2830);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3442);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 520);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2069);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1257);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2070);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2832);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -μωμαι.
pub(crate) fn n114(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 521);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1258);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2071);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1259);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2072);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1260);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1261);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νωμαι.
pub(crate) fn n115(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 113);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1262);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2075);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2833);
									match s {
										[] => true,
										['σ', s @ ..] => {
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
		['γ', s @ ..] => {
			super::push(out, 526);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2076);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2834);
									s.is_empty()
								}
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
			super::push(out, 527);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2077);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2835);
									s.is_empty()
								}
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
			super::push(out, 528);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1265);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2078);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2836);
									s.is_empty()
								}
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
			super::push(out, 529);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1266);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2079);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2837);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				['ο', s @ ..] => {
					super::push(out, 1267);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2080);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2838);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
		['π', s @ ..] => {
			super::push(out, 530);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1268);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1269);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2081);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2839);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ξωμαι.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 114);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1270);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2082);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2840);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πωμαι.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 115);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 533);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1271);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2083);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2841);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3448);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρωμαι.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 534);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1272);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2084);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2842);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3449);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3873);
													match s {
														[] => true,
														['β', s @ ..] => super::t1::n4111(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1273);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2085);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2843);
									s.is_empty()
								}
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
			super::push(out, 535);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1274);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 536);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1275);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2087);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1276);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2088);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2844);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3450);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 3874);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1277);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2089);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2845);
									s.is_empty()
								}
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
			super::push(out, 538);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1278);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['ε', 'π', 'μ', s @ ..] => {
									super::push(out, 2846);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3451);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3875);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 539);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1279);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2091);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2847);
									s.is_empty()
								}
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
			super::push(out, 540);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2092);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2848);
									s.is_empty()
								}
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
			super::push(out, 541);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1281);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2093);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2849);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3452);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3876);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n4112(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1282);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2095);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1283);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2096);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2850);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3453);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3877);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4113(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2097);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2851);
									s.is_empty()
								}
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
			super::push(out, 542);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1284);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2098);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2852);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2099);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2853);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1286);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2100);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2854);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σωμαι.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1287);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2101);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2855);
									s.is_empty()
								}
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
			super::push(out, 544);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1288);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2102);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2856);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τωμαι.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 545);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1289);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2103);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2857);
									s.is_empty()
								}
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
			super::push(out, 546);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1290);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2104);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 2858);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3454);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 547);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1291);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2105);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2859);
									s.is_empty()
								}
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
			super::push(out, 548);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1292);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2106);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2860);
									s.is_empty()
								}
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
			super::push(out, 549);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1293);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2107);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1294);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2108);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φωμαι.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1295);
					match s {
						[] => true,
						['μ', 'ε', 'μ', 'α', s @ ..] => {
							super::push(out, 2109);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2861);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3455);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3878);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χωμαι.
pub(crate) fn n122(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1296);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2110);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2862);
									s.is_empty()
								}
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
			super::push(out, 552);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 1297);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2111);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2863);
									match s {
										[] => true,
										['ε', s @ ..] => {
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

/// Words ending -γκατειλεγμαι.
pub(crate) fn n3903(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3886);
	s.is_empty()
}

/// The word επενηνεγμαι.
pub(crate) fn n3904(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3887);
	s.is_empty()
}

/// The word υπενηνεγμαι.
pub(crate) fn n3905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3888);
	s.is_empty()
}

/// Words ending -ψεγμαι.
pub(crate) fn n161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 601);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συρρηρρηγμαι.
pub(crate) fn n3906(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3889);
	s.is_empty()
}

/// Words ending -αποδεδειγμαι.
pub(crate) fn n3907(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3890);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4098);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υποδεδειγμαι.
pub(crate) fn n3908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3891);
	s.is_empty()
}

/// Words ending -συμμεμειγμαι.
pub(crate) fn n3909(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3892);
	s.is_empty()
}

/// Words ending -καταμεμιγμαι.
pub(crate) fn n3910(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3893);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 4099);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4182);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αταδεδιωγμαι.
pub(crate) fn n3911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3894);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 4100);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λελητουργημαι.
pub(crate) fn n4120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4103);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 4184);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 4214);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -προμεμηθημαι.
pub(crate) fn n3919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3902);
	s.is_empty()
}

/// Words ending -ραβεβοηθημαι.
pub(crate) fn n3920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3903);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4104);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 4185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιβεβοηθημαι.
pub(crate) fn n3921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3904);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4105);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οσβεβοηθημαι.
pub(crate) fn n3922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3905);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 4106);
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

/// Words ending -εριπεποιημαι.
pub(crate) fn n3923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3906);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4107);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ντιπεποιημαι.
pub(crate) fn n3924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3907);
	s.is_empty()
}

/// Words ending -ροσπεποιημαι.
pub(crate) fn n3925(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3908);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4108);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παραβεβλημαι.
pub(crate) fn n3926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3909);
	s.is_empty()
}

/// Words ending -καταβεβλημαι.
pub(crate) fn n3927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3910);
	s.is_empty()
}

/// Words ending -μεταβεβλημαι.
pub(crate) fn n3928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3911);
	s.is_empty()
}

/// Words ending -υπερβεβλημαι.
pub(crate) fn n3929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3912);
	s.is_empty()
}

/// Words ending -επιμεμελημαι.
pub(crate) fn n3930(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3913);
	s.is_empty()
}

/// Words ending -δεδηλημαι.
pub(crate) fn n2262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2255);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2970);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κηλημαι.
pub(crate) fn n724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 721);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1469);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2256);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιλημαι.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 243);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 722);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1470);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1471);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 723);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1472);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 724);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1473);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2258);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κλημαι.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 244);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 725);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1474);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2259);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2971);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3525);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2972);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3526);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3914);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2973);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3527);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3915);
													s.is_empty()
												}
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
							super::push(out, 2261);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2974);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2975);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3528);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2262);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2976);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3529);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2263);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2977);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2264);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2978);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3530);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2979);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3531);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3916);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λλημαι.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 726);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1475);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2266);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2980);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ολημαι.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 727);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1476);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2267);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2981);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3532);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3917);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4126(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 728);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1477);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 729);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1478);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2269);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υλημαι.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 730);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1479);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2270);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωλημαι.
pub(crate) fn n250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 731);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1480);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2271);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2983);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3533);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3918);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -απεπολεμημαι.
pub(crate) fn n3936(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3919);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 4110);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 4187);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 4215);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μπεπολεμημαι.
pub(crate) fn n3937(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3920);
	s.is_empty()
}

/// Words ending -εβλασφημημαι.
pub(crate) fn n3938(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3921);
	s.is_empty()
}

/// Words ending -τακεκοιμημαι.
pub(crate) fn n3939(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3922);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4111);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4188);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -προτετιμημαι.
pub(crate) fn n3940(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3923);
	s.is_empty()
}

/// Words ending -τολμημαι.
pub(crate) fn n1497(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1494);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2283);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2992);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ομημαι.
pub(crate) fn n259(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 746);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2284);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2993);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1496);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2285);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2994);
									s.is_empty()
								}
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
			super::push(out, 747);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2286);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2995);
									s.is_empty()
								}
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
			super::push(out, 748);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1498);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2287);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2996);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1499);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2288);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2289);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2998);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3538);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3924);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4129(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1500);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2290);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2999);
									s.is_empty()
								}
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
			super::push(out, 749);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2291);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3000);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3539);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3925);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n4130(s, out),
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
									super::push(out, 3001);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3540);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3926);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n4131(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1502);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2292);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3002);
									s.is_empty()
								}
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
			super::push(out, 750);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2293);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3003);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 3541);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3927);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1504);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2294);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3004);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3542);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3928);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρμημαι.
pub(crate) fn n260(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 258);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 751);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1505);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σμημαι.
pub(crate) fn n261(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 259);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 752);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2295);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3005);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3543);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τμημαι.
pub(crate) fn n262(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 260);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 753);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1507);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2296);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3006);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3544);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2297);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3007);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3545);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 3008);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3546);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3929);
													s.is_empty()
												}
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
							super::push(out, 2298);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3009);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2299);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3010);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3547);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2300);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3011);
									match s {
										[] => true,
										['α', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -υμημαι.
pub(crate) fn n263(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 261);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 754);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1508);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2301);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3012);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3549);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 3013);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3550);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3930);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1509);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word μεμηχανημαι.
pub(crate) fn n3948(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3931);
	s.is_empty()
}

/// Words ending -διαγεγενημαι.
pub(crate) fn n3949(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3932);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4115);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συγγεγενημαι.
pub(crate) fn n3950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3933);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 4116);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 4191);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εριγεγενημαι.
pub(crate) fn n3951(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3934);
	s.is_empty()
}

/// Words ending -απογεγενημαι.
pub(crate) fn n3952(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3935);
	s.is_empty()
}

/// Words ending -υπογεγενημαι.
pub(crate) fn n3953(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3936);
	s.is_empty()
}

/// Words ending -προγεγενημαι.
pub(crate) fn n3954(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3937);
	s.is_empty()
}

/// Words ending -βινημαι.
pub(crate) fn n767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 764);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1519);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 2309);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δινημαι.
pub(crate) fn n768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 765);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1520);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2310);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3023);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3560);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3938);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κινημαι.
pub(crate) fn n769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 766);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1521);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2311);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3024);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 3025);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3561);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κνημαι.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 266);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1522);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μνημαι.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 267);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2312);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3026);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3562);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2313);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3027);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3563);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 769);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1524);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2314);
							match s {
								[] => true,
								['μ', s @ ..] => {
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
		['υ', s @ ..] => {
			super::push(out, 770);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ννημαι.
pub(crate) fn n270(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1525);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ονημαι.
pub(crate) fn n271(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 773);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1527);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2317);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3029);
									s.is_empty()
								}
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
			super::push(out, 774);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1528);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2318);
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
		['κ', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1529);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2319);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3031);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3564);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3939);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 776);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1530);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2320);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3032);
									s.is_empty()
								}
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
			super::push(out, 777);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1531);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2321);
							match s {
								[] => true,
								['η', s @ ..] => {
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
		['π', s @ ..] => {
			super::push(out, 778);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1532);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2322);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['ρ', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1533);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2323);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3035);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3565);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3940);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n4134(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 3566);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3941);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4135(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2324);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3036);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3567);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 780);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1534);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2325);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 3037);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3568);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3942);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n4136(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 781);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1535);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2326);
							match s {
								[] => true,
								['ο', s @ ..] => {
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
		['χ', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1536);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2327);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3039);
									match s {
										[] => true,
										['α', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πνημαι.
pub(crate) fn n272(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 270);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1537);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2328);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3040);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3570);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', 'ρ', 'γ', 'η', '\u{1}', s @ ..] => {
			super::push(out, 784);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρνημαι.
pub(crate) fn n273(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 271);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 785);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1538);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνημαι.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 272);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 786);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1539);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2329);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1540);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 788);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1541);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2331);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1542);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2332);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3041);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3571);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3943);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', '\u{1}', s @ ..] => {
			super::push(out, 789);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 790);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1543);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χνημαι.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 273);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 791);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1544);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2333);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3042);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωνημαι.
pub(crate) fn n276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 792);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 793);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1545);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2334);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3043);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3572);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 794);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1546);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2335);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3044);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3573);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3944);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 795);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1547);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -καταβεβοημαι.
pub(crate) fn n3962(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3945);
	s.is_empty()
}

/// Words ending -πεπροτερημαι.
pub(crate) fn n3963(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3946);
	s.is_empty()
}

/// Words ending -ελαγοθηρημαι.
pub(crate) fn n3964(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3947);
	s.is_empty()
}

/// Words ending -εγκεχειρημαι.
pub(crate) fn n3965(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3948);
	s.is_empty()
}

/// Words ending -ορημαι.
pub(crate) fn n290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 288);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 822);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1578);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3063);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3588);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3949);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['β', s @ ..] => {
									super::push(out, 3064);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3589);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3950);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4137(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2363);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3065);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3590);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 823);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1579);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2364);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3066);
									s.is_empty()
								}
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
			super::push(out, 824);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1580);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2365);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3067);
									s.is_empty()
								}
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
			super::push(out, 825);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1581);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2366);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 826);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1582);
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
		['φ', s @ ..] => {
			super::push(out, 827);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1583);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2368);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3068);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3069);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3591);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3951);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1584);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2369);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1585);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υρημαι.
pub(crate) fn n292(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 290);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 829);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1588);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1589);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2372);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1590);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2373);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 830);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1591);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1592);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2374);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3071);
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
		['τ', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1593);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2375);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3072);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χρημαι.
pub(crate) fn n293(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 291);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1594);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2376);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3073);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3593);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3952);
													s.is_empty()
												}
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
							super::push(out, 2377);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3074);
									match s {
										[] => true,
										['ε', s @ ..] => {
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

/// Words ending -ωρημαι.
pub(crate) fn n294(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 292);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1595);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 835);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1596);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2379);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3075);
									s.is_empty()
								}
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
			super::push(out, 836);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1597);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2380);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3076);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3595);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3953);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n4138(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 837);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1598);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2381);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3077);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3596);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 838);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1599);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2382);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3078);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3597);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 3079);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3598);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 3599);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3954);
													s.is_empty()
												}
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
							super::push(out, 2383);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3080);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3600);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3955);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1600);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εριπεπατημαι.
pub(crate) fn n3973(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3956);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4122);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ελογοθετημαι.
pub(crate) fn n3974(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3957);
	s.is_empty()
}

/// Words ending -ετοποθετημαι.
pub(crate) fn n3975(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3958);
	s.is_empty()
}

/// Words ending -εντημαι.
pub(crate) fn n859(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 856);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1620);
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

/// Words ending -ηντημαι.
pub(crate) fn n860(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 857);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 2404);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οτημαι.
pub(crate) fn n305(s: &[char], out: &mut Path<'_>) -> bool {
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 858);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2405);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1623);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2406);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3092);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3607);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3959);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 859);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1624);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2407);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πτημαι.
pub(crate) fn n306(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 303);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 860);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρτημαι.
pub(crate) fn n307(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 304);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 861);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2408);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 862);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1627);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2409);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -στημαι.
pub(crate) fn n308(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 305);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 863);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1628);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2410);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3094);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3608);
											match s {
												[] => true,
												['α', 'π', 'ε', 'π', 'α', s @ ..] => {
													super::push(out, 3960);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n4140(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 864);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2411);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3095);
									match s {
										[] => true,
										['π', s @ ..] => {
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
		['ι', s @ ..] => {
			super::push(out, 865);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1630);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2412);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 3096);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3610);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 3961);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υτημαι.
pub(crate) fn n309(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 306);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 866);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1631);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2413);
							match s {
								[] => true,
								['γ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ωτημαι.
pub(crate) fn n310(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 307);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 867);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1632);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word εζωγραφημαι.
pub(crate) fn n3979(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3962);
	s.is_empty()
}

/// Words ending -αμπαδαρχημαι.
pub(crate) fn n3980(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3963);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 4124);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αμπαδουχημαι.
pub(crate) fn n3981(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3964);
	s.is_empty()
}

/// Words ending -δεδυστυχημαι.
pub(crate) fn n3982(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3965);
	s.is_empty()
}

/// Words ending -παραγεγραμμαι.
pub(crate) fn n4142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4125);
	s.is_empty()
}

/// Words ending -καταγεγραμμαι.
pub(crate) fn n4143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4126);
	s.is_empty()
}

/// Words ending -περιγεγραμμαι.
pub(crate) fn n4144(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4127);
	s.is_empty()
}

/// Words ending -προσγεγραμμαι.
pub(crate) fn n4145(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4128);
	s.is_empty()
}

/// Words ending -ετραμμαι.
pub(crate) fn n1682(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1678);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 2461);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3134);
					s.is_empty()
				}
				['α', s @ ..] => {
					super::push(out, 3135);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 3644);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3981);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 3136);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3645);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 3137);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3646);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3983);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 4129);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4197);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 3138);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3647);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 3139);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3648);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3984);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 3649);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3985);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εστραμμαι.
pub(crate) fn n2471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2462);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 3140);
			match s {
				[] => true,
				['α', 'ν', s @ ..] => {
					super::push(out, 3650);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 3986);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 4130);
									s.is_empty()
								}
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
			super::push(out, 3141);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3651);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 3652);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3142);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3653);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3987);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμπροπεπεμμαι.
pub(crate) fn n4148(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4131);
	s.is_empty()
}

/// Words ending -επανειλημμαι.
pub(crate) fn n4009(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3992);
	s.is_empty()
}

/// Words ending -προσειλημμαι.
pub(crate) fn n4010(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3993);
	s.is_empty()
}

/// Words ending -αραλελειμμαι.
pub(crate) fn n4011(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3994);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4132);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -απολελειμμαι.
pub(crate) fn n4012(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3995);
	s.is_empty()
}

/// Words ending -υπολελειμμαι.
pub(crate) fn n4013(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3996);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4133);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προλελειμμαι.
pub(crate) fn n4014(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3997);
	s.is_empty()
}

/// Words ending -συντεθλιμμαι.
pub(crate) fn n4015(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3998);
	s.is_empty()
}

/// Words ending -αποτεθλιμμαι.
pub(crate) fn n4016(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3999);
	s.is_empty()
}

/// Words ending -εριτεθαλμμαι.
pub(crate) fn n4017(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4000);
	s.is_empty()
}

/// Words ending -ποκεκαλυμμαι.
pub(crate) fn n4018(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4001);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4134);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4198);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιβεβιβασμαι.
pub(crate) fn n4021(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4004);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4135);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4199);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κατειργασμαι.
pub(crate) fn n4022(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4005);
	s.is_empty()
}

/// Words ending -κεκραυγασμαι.
pub(crate) fn n4023(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4006);
	s.is_empty()
}

/// The word κεκερδασμαι.
pub(crate) fn n4024(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4007);
	s.is_empty()
}

/// Words ending -ιασμαι.
pub(crate) fn n394(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 973);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1734);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2516);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 974);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1735);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2517);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3187);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3690);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 975);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1736);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2518);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3188);
									s.is_empty()
								}
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
			super::push(out, 976);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1737);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2519);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3189);
									s.is_empty()
								}
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
			super::push(out, 977);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1738);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2520);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3190);
									s.is_empty()
								}
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
			super::push(out, 978);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1739);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2521);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3191);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3691);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4008);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 979);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2522);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3192);
									s.is_empty()
								}
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
			super::push(out, 980);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1741);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2523);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2524);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3194);
									s.is_empty()
								}
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
			super::push(out, 981);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2525);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3195);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3692);
											s.is_empty()
										}
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
					super::push(out, 1743);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2526);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3196);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3693);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4009);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1744);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2527);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3197);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3694);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4010);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4153(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 982);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2528);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 983);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2529);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3198);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3695);
											s.is_empty()
										}
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
					super::push(out, 1747);
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

/// Words ending -κεκασμαι.
pub(crate) fn n1753(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1749);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 2532);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ικασμαι.
pub(crate) fn n989(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 986);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1750);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2533);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3200);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3696);
									s.is_empty()
								}
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
			super::push(out, 1751);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2534);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3201);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3697);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3698);
									s.is_empty()
								}
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
			super::push(out, 1752);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρκασμαι.
pub(crate) fn n990(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 987);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1753);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2535);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3202);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υκασμαι.
pub(crate) fn n991(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 988);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1754);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2536);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3203);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λασμαι.
pub(crate) fn n396(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 989);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2537);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3204);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 1756);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2538);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1757);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2539);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3205);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3700);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 990);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1758);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2540);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 991);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1759);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2541);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3206);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1760);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2542);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3207);
									s.is_empty()
								}
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
			super::push(out, 992);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1761);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2543);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μασμαι.
pub(crate) fn n397(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 394);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 993);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1762);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2544);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3208);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3701);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 994);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1763);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2545);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3209);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1764);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2546);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3210);
									s.is_empty()
								}
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
			super::push(out, 995);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1765);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2547);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3211);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3702);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 4011);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4154(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1766);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2548);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3212);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3703);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 4012);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1767);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2549);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3213);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3704);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4013);
													s.is_empty()
												}
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
							super::push(out, 2550);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3214);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3705);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3706);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 4014);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4155(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1768);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 996);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1769);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2552);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3215);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 3216);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3707);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 997);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1770);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2553);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3217);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3708);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 998);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1771);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νασμαι.
pub(crate) fn n398(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 395);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 999);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1772);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2554);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1773);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2555);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3218);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3709);
											s.is_empty()
										}
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
					super::push(out, 1774);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2556);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3219);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 3710);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1001);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1775);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2557);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3220);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3711);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4015);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1776);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2558);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3221);
									s.is_empty()
								}
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
			super::push(out, 1002);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1777);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2559);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3222);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3712);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξασμαι.
pub(crate) fn n399(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 396);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1003);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1778);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1779);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2560);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3223);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οασμαι.
pub(crate) fn n400(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 397);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1005);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1780);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2561);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πασμαι.
pub(crate) fn n401(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 398);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1006);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1781);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2562);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3224);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1782);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2563);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2564);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3225);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3713);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1007);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1783);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2565);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3226);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3714);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2566);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3227);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3715);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1008);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1784);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2567);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 1009);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρασμαι.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 399);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1010);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1785);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2568);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3228);
									s.is_empty()
								}
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
			super::push(out, 1011);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1786);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2569);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1012);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1787);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2570);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3229);
									s.is_empty()
								}
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
			super::push(out, 1013);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1788);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2571);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3230);
									s.is_empty()
								}
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
			super::push(out, 1014);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2572);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3231);
									s.is_empty()
								}
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
			super::push(out, 1015);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2573);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3232);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3716);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1016);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2574);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1017);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2575);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3233);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3717);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1018);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2576);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3234);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3718);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1019);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1795);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2577);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3235);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3719);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4016);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n4156(s, out),
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
									super::push(out, 3236);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3720);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 4017);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4157(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τασμαι.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 400);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1020);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2578);
							match s {
								[] => true,
								['α', s @ ..] => {
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
		['ν', s @ ..] => {
			super::push(out, 1021);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1797);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2579);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3238);
									s.is_empty()
								}
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
			super::push(out, 1022);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1798);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2580);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3239);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3721);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1023);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1799);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2581);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3240);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3722);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1024);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2582);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3241);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3723);
											s.is_empty()
										}
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
					super::push(out, 1801);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2583);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1025);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1802);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2584);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3242);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3724);
											s.is_empty()
										}
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
					super::push(out, 1803);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2585);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3243);
									s.is_empty()
								}
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
			super::push(out, 1026);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1804);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2586);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3244);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3725);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υασμαι.
pub(crate) fn n404(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 401);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1027);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1805);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2587);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3245);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3726);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 3727);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 4018);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4158(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 3728);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4019);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n4159(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3729);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4020);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4160(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3730);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4021);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n4161(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1806);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2588);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3246);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3731);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4022);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φασμαι.
pub(crate) fn n405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1028);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2589);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2590);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3247);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3732);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1029);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1808);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χασμαι.
pub(crate) fn n406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1030);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1809);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2591);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3248);
									s.is_empty()
								}
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
			super::push(out, 1031);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1810);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2592);
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
				['σ', s @ ..] => {
					super::push(out, 1811);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2593);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3249);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωασμαι.
pub(crate) fn n407(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 404);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1033);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1812);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιαλελογισμαι.
pub(crate) fn n4044(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4027);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 4149);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εκλελογισμαι.
pub(crate) fn n4045(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4028);
	s.is_empty()
}

/// Words ending -υλλελογισμαι.
pub(crate) fn n4046(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4029);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 4150);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 4201);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υπλελογισμαι.
pub(crate) fn n4047(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4030);
	s.is_empty()
}

/// Words ending -νδραποδισμαι.
pub(crate) fn n4048(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4031);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 4151);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εμπεποδισμαι.
pub(crate) fn n4049(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4032);
	s.is_empty()
}

/// Words ending -εισμαι.
pub(crate) fn n424(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 421);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1059);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1839);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2613);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3262);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3749);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 4033);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4169(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1060);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1840);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2614);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3263);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3264);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3750);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4034);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1841);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2615);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 1061);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1842);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1062);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1843);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θισμαι.
pub(crate) fn n425(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 422);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1063);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1844);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2618);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3265);
									s.is_empty()
								}
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
			super::push(out, 1064);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1845);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1065);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1066);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1847);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2619);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3266);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3751);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4035);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n4170(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κισμαι.
pub(crate) fn n426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 423);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1067);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2620);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 3267);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3752);
											s.is_empty()
										}
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
					super::push(out, 1849);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2621);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 3268);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3753);
											s.is_empty()
										}
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
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2622);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3269);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3754);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 2623);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 3270);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3755);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 4036);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1068);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1850);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2624);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3271);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3756);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1069);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1851);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2625);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2626);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3273);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3757);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4037);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n4171(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1070);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1853);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1071);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1854);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2627);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3274);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3758);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1072);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1855);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λισμαι.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 424);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1073);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1856);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1857);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2628);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3275);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3759);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 4038);
													match s {
														[] => true,
														['ο', s @ ..] => super::t1::n4172(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2629);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3276);
									s.is_empty()
								}
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
			super::push(out, 1074);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1858);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2630);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3277);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3760);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4039);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1075);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1859);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2631);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3278);
									s.is_empty()
								}
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
			super::push(out, 1076);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1860);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2632);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3279);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3761);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1077);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1861);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2633);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3280);
									s.is_empty()
								}
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
			super::push(out, 1078);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1862);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1079);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1863);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2634);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3281);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μισμαι.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 425);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1080);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1864);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2635);
							match s {
								[] => true,
								['γ', s @ ..] => {
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
		['η', s @ ..] => {
			super::push(out, 1081);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1865);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2636);
							match s {
								[] => true,
								['η', s @ ..] => {
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
		['ο', s @ ..] => {
			super::push(out, 1082);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1866);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2637);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3284);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3762);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 4040);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n4173(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 4041);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4174(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 3763);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 4042);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n4175(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1867);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2638);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3285);
									s.is_empty()
								}
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
			super::push(out, 1083);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1868);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νισμαι.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 426);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1869);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2639);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3286);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3764);
											s.is_empty()
										}
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
					super::push(out, 1870);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2640);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3287);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1871);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2641);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 3288);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3765);
											s.is_empty()
										}
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
					super::push(out, 1872);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2642);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3289);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3766);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4043);
													s.is_empty()
												}
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
							super::push(out, 2643);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 1085);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1873);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2644);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1874);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1875);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1087);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1876);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2645);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3290);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3767);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1088);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1877);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2646);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1878);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2647);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3291);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3768);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4044);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1879);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2648);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3292);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3769);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 4045);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1880);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2649);
							match s {
								[] => true,
								['ε', s @ ..] => {
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3770);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4046);
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 4047);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n4176(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 3771);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 4048);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n4177(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1881);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2650);
							match s {
								[] => true,
								['η', s @ ..] => {
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
		['π', s @ ..] => {
			super::push(out, 1090);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1882);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2651);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3294);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3772);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1091);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1883);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2652);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3295);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 3296);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3773);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3297);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3774);
											s.is_empty()
										}
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
					super::push(out, 1884);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2653);
							match s {
								[] => true,
								['λ', s @ ..] => super::t1::n3313(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οισμαι.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1092);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1885);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2654);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πισμαι.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1093);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1886);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2655);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 3299);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3776);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1094);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1887);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2656);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3300);
									s.is_empty()
								}
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
			super::push(out, 1095);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1888);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2657);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2658);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3301);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3777);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1096);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1889);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2659);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3302);
									s.is_empty()
								}
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
			super::push(out, 1097);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1890);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2660);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3303);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3778);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 4050);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4178(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2661);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3304);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3779);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρισμαι.
pub(crate) fn n432(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1098);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1891);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2662);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3305);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3780);
											s.is_empty()
										}
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
					super::push(out, 1892);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2663);
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
				['χ', s @ ..] => {
					super::push(out, 1893);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2664);
							match s {
								[] => true,
								['κ', s @ ..] => {
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
		['β', s @ ..] => {
			super::push(out, 1099);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1894);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2665);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1100);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1895);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2666);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3308);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3781);
											s.is_empty()
										}
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
					super::push(out, 1896);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2667);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3309);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3782);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2668);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 3310);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3783);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4051);
													s.is_empty()
												}
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
							super::push(out, 2669);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3311);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3784);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1101);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1897);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2670);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3312);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3785);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4052);
													match s {
														[] => true,
														['χ', s @ ..] => super::t1::n4179(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2671);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3313);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3786);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 4053);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1102);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1898);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2672);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3314);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3787);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4054);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n4180(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1103);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1899);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2673);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3315);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3788);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4055);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n4181(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1900);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2674);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3316);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3789);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4056);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1104);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1901);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2675);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1902);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2676);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1903);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2677);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1105);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1904);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => super::t1::n1109(s, out),
		_ => false,
	}
}

/// Words ending -τισμαι.
pub(crate) fn n433(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1107);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1910);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2682);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3321);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3322);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3791);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 3323);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3792);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4058);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1108);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1911);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2683);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3324);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3793);
											s.is_empty()
										}
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
					super::push(out, 1912);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2684);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1913);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1109);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1914);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2686);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3325);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3794);
											s.is_empty()
										}
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
					super::push(out, 1915);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2687);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1916);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2688);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3326);
									match s {
										[] => true,
										['ῥ', s @ ..] => {
											super::push(out, 3795);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 4059);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1917);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2689);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 3327);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3796);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1111);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1918);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2690);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		['π', s @ ..] => {
			super::push(out, 1112);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1919);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2691);
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
				['ε', s @ ..] => {
					super::push(out, 1920);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2692);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1113);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1921);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2693);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3330);
									s.is_empty()
								}
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
			super::push(out, 1114);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1922);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2694);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3331);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3797);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 4060);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φισμαι.
pub(crate) fn n434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 1923);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2695);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1116);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1924);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2696);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3332);
									s.is_empty()
								}
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
			super::push(out, 1117);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1925);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2697);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3333);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3798);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4061);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n4183(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χισμαι.
pub(crate) fn n435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 432);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1118);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1926);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2698);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3334);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3799);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 4062);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n4184(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 4063);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n4185(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1119);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1927);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2699);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωισμαι.
pub(crate) fn n436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 433);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1928);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2700);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3335);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3800);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 4064);
													match s {
														[] => true,
														['η', s @ ..] => super::t1::n4186(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1121);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1929);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2701);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ιαλελοιπσμαι.
pub(crate) fn n4082(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4065);
	s.is_empty()
}

/// Words ending -απολελαυσμαι.
pub(crate) fn n4083(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4066);
	s.is_empty()
}

/// Words ending -υντεθραυσμαι.
pub(crate) fn n4084(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4067);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 4170);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ποτεθραυσμαι.
pub(crate) fn n4085(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4068);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4171);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word κεκελευσμαι.
pub(crate) fn n4086(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4069);
	s.is_empty()
}

/// Words ending -ακεκελευσμαι.
pub(crate) fn n4087(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4070);
	s.is_empty()
}

/// Words ending -λυσμαι.
pub(crate) fn n444(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1135);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1945);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2716);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3348);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3807);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 4071);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n4189(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1136);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1946);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2717);
							match s {
								[] => true,
								['μ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -διηνυσμαι.
pub(crate) fn n2728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2718);
	s.is_empty()
}

/// Words ending -πυσμαι.
pub(crate) fn n446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1138);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1948);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2719);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2720);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3350);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3808);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2721);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3351);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3809);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυσμαι.
pub(crate) fn n447(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 444);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1139);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1949);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2722);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3352);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3810);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -συσμαι.
pub(crate) fn n448(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 445);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1140);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1950);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2723);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3353);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3811);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τυσμαι.
pub(crate) fn n449(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 446);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1141);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1951);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2724);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3354);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3812);
											s.is_empty()
										}
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
					super::push(out, 1952);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2725);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3355);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3813);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1142);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1953);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2726);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3356);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χυσμαι.
pub(crate) fn n450(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 447);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1143);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2727);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3357);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επικεκλωσμαι.
pub(crate) fn n4089(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4072);
	s.is_empty()
}

/// Words ending -προπεπαιδευμαι.
pub(crate) fn n4090(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4073);
	s.is_empty()
}

/// Words ending -ουλευμαι.
pub(crate) fn n1986(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1981);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 2750);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3372);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 3826);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 4074);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 4173);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 4212);
													match s {
														[] => true,
														['ο', s @ ..] => super::t1::n4235(s, out),
														['τ', s @ ..] => super::t1::n4236(s, out),
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
									super::push(out, 4075);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 4174);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 4213);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 2751);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3373);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3827);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μευμαι.
pub(crate) fn n466(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 463);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1168);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1983);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2753);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3375);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3828);
											s.is_empty()
										}
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
					super::push(out, 1984);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2754);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3376);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3829);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νευμαι.
pub(crate) fn n467(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 464);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1169);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1985);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2755);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3377);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3830);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 4076);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1170);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1171);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1986);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1172);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1173);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 1174);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1987);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2756);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1175);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1988);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2757);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3378);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3831);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1176);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1989);
					match s {
						[] => true,
						['ν', 'ι', 'κ', 'ε', 'κ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3379);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3380);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3832);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 4077);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['γ', s @ ..] => {
									super::push(out, 3381);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3833);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 4078);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 3382);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3834);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 4079);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξευμαι.
pub(crate) fn n468(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 465);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1177);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1990);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2758);
							match s {
								[] => true,
								['λ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -πευμαι.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 466);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1178);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1179);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρευμαι.
pub(crate) fn n470(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 467);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1180);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1991);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2759);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3384);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3835);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1181);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1992);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2760);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1182);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1993);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2761);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1994);
					s.is_empty()
				}
				['χ', s @ ..] => {
					super::push(out, 1995);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2762);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3385);
									s.is_empty()
								}
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
			super::push(out, 1183);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1996);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1184);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1997);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2763);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3386);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3836);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σευμαι.
pub(crate) fn n471(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 468);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1185);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1998);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2764);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3387);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τευμαι.
pub(crate) fn n472(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 469);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1186);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2765);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3388);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3837);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2766);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3389);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3838);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1187);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1999);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2767);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3390);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3839);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1188);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2000);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2768);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3391);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3840);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1189);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2001);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2769);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3392);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3841);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1190);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1191);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 1192);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2002);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2770);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3393);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3842);
											s.is_empty()
										}
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
					super::push(out, 2003);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2771);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3394);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3843);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1193);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2004);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2772);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		['ω', s @ ..] => {
			super::push(out, 1194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φευμαι.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 470);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1195);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2005);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2773);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3396);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χευμαι.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 471);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1196);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2006);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2774);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3397);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3844);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1197);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2007);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2775);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -διαμεμηνυμαι.
pub(crate) fn n4102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4085);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4177);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προμεμηνυμαι.
pub(crate) fn n4103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4086);
	s.is_empty()
}

/// The word εκκεκρουμαι.
pub(crate) fn n4104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4087);
	s.is_empty()
}

/// Words ending -αποκεκρουμαι.
pub(crate) fn n4105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4088);
	s.is_empty()
}

/// Words ending -προκεκρουμαι.
pub(crate) fn n4106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4089);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4178);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word διακεκρυμαι.
pub(crate) fn n4107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4090);
	s.is_empty()
}

/// Words ending -κατακεκρυμαι.
pub(crate) fn n4108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4091);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4179);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επικεκρυμαι.
pub(crate) fn n4109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4092);
	s.is_empty()
}

/// Words ending -περικεκρυμαι.
pub(crate) fn n4110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4093);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 4180);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -βεβαρβαρωμαι.
pub(crate) fn n4111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4094);
	s.is_empty()
}

/// Words ending -κατεπετρωμαι.
pub(crate) fn n4112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4095);
	s.is_empty()
}

/// Words ending -πολελυτρωμαι.
pub(crate) fn n4113(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4096);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4181);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ελογχοβολημαι.
pub(crate) fn n4126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4109);
	s.is_empty()
}

/// Words ending -εκληρονομημαι.
pub(crate) fn n4129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4112);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 4189);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μπαδοδρομημαι.
pub(crate) fn n4130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4113);
	s.is_empty()
}

/// Words ending -τελεοδρομημαι.
pub(crate) fn n4131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4114);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4190);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 4216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αταπεφρονημαι.
pub(crate) fn n4134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4117);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 4192);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εριπεφρονημαι.
pub(crate) fn n4135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4118);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4193);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δελφοκτονημαι.
pub(crate) fn n4136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4119);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 4194);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ελαβρηγορημαι.
pub(crate) fn n4137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4120);
	s.is_empty()
}

/// Words ending -ροτετιμωρημαι.
pub(crate) fn n4138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4121);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4195);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ταπεπαιδεραστημαι.
pub(crate) fn n4140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4123);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4196);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 4217);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επαρουσιασμαι.
pub(crate) fn n4153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4136);
	s.is_empty()
}

/// Words ending -πικεχειμασμαι.
pub(crate) fn n4154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4137);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4200);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προητοιμασμαι.
pub(crate) fn n4155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4138);
	s.is_empty()
}

/// Words ending -μεταπεφρασμαι.
pub(crate) fn n4156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4139);
	s.is_empty()
}

/// The word συμπεφρασμαι.
pub(crate) fn n4157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4140);
	s.is_empty()
}

/// The word διεσκευασμαι.
pub(crate) fn n4158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4141);
	s.is_empty()
}

/// The word ανεσκευασμαι.
pub(crate) fn n4159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4142);
	s.is_empty()
}

/// Words ending -παρεσκευασμαι.
pub(crate) fn n4160(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4143);
	s.is_empty()
}

/// Words ending -κατεσκευασμαι.
pub(crate) fn n4161(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4144);
	s.is_empty()
}

/// Words ending -ερικεκλεισμαι.
pub(crate) fn n4169(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4152);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4202);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σεστρουθισμαι.
pub(crate) fn n4170(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4153);
	s.is_empty()
}

/// Words ending -τετειχικισμαι.
pub(crate) fn n4171(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4154);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 4203);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οκεκεφαλισμαι.
pub(crate) fn n4172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4155);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4204);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -διακεκομισμαι.
pub(crate) fn n4173(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4156);
	s.is_empty()
}

/// Words ending -ετακεκομισμαι.
pub(crate) fn n4174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4157);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 4205);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ροσκεκομισμαι.
pub(crate) fn n4175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4158);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 4206);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νακεχρονισμαι.
pub(crate) fn n4176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4159);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4207);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συνεχρονισμαι.
pub(crate) fn n4177(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4160);
	s.is_empty()
}

/// Words ending -λακωνισμαι.
pub(crate) fn n3313(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3298);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3775);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 4049);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιδεδορπισμαι.
pub(crate) fn n4178(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4161);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4208);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χαρακτηρισμαι.
pub(crate) fn n4179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4162);
	s.is_empty()
}

/// Words ending -τακεχειρισμαι.
pub(crate) fn n4180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4163);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 4209);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υποκεκορισμαι.
pub(crate) fn n4181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4164);
	s.is_empty()
}

/// Words ending -ωρισμαι.
pub(crate) fn n1109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1106);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1905);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1906);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2678);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3317);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2679);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3318);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1908);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3319);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2681);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3320);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3790);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 4057);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 4165);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νακεκουφισμαι.
pub(crate) fn n4183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4166);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4210);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εκτετειχισμαι.
pub(crate) fn n4184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4167);
	s.is_empty()
}

/// Words ending -ποτετειχισμαι.
pub(crate) fn n4185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4168);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 4211);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηγχονιζωισμαι.
pub(crate) fn n4186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4169);
	s.is_empty()
}

/// Words ending -κατακεκλυσμαι.
pub(crate) fn n4189(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4172);
	s.is_empty()
}

/// Words ending -οεπιβεβουλευμαι.
pub(crate) fn n4235(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4218);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 4220);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τεπιβεβουλευμαι.
pub(crate) fn n4236(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4219);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 4221);
			s.is_empty()
		}
		_ => false,
	}
}

