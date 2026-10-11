use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => super::t0::n1(s, out),
		_ => false,
	}
}

/// Words ending -α.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 2);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 16);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 76);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 283);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 800);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 1485);
													s.is_empty()
												}
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
							super::push(out, 77);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 284);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 801);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 1486);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 285);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 802);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 1487);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n2210(s, out),
														['μ', s @ ..] => super::t0::n2211(s, out),
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
									super::push(out, 286);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 803);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 1488);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 17);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 78);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 287);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 804);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 1489);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 18);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 79);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 288);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 805);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 1490);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n2212(s, out),
														_ => false,
													}
												}
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
							super::push(out, 80);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 289);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 806);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 1491);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n2213(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 19);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 81);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 290);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 807);
											s.is_empty()
										}
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
					super::push(out, 20);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 82);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 291);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 808);
											s.is_empty()
										}
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
					super::push(out, 21);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 83);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 292);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 809);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 1492);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 22);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 84);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 293);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 810);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 1493);
													match s {
														[] => true,
														['υ', s @ ..] => super::t0::n2214(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n3(s, out),
		['θ', s @ ..] => super::t0::n4(s, out),
		['κ', s @ ..] => super::t0::n5(s, out),
		['λ', s @ ..] => {
			super::push(out, 6);
			match s {
				[] => true,
				['α', s @ ..] => super::t0::n41(s, out),
				['η', s @ ..] => {
					super::push(out, 42);
					match s {
						[] => true,
						['θ', s @ ..] => super::t0::n206(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => super::t0::n7(s, out),
		['ν', s @ ..] => super::t0::n8(s, out),
		['ο', s @ ..] => super::t0::n9(s, out),
		['π', s @ ..] => super::t0::n10(s, out),
		['ρ', s @ ..] => super::t0::n11(s, out),
		['σ', s @ ..] => super::t0::n12(s, out),
		['τ', s @ ..] => super::t0::n13(s, out),
		['φ', s @ ..] => super::t0::n14(s, out),
		['χ', s @ ..] => super::t0::n15(s, out),
		_ => false,
	}
}

/// The word πεπραγα.
pub(crate) fn n2210(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2198);
	s.is_empty()
}

/// Words ending -μπεπραγα.
pub(crate) fn n2211(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2199);
	s.is_empty()
}

/// Words ending -απεπληγα.
pub(crate) fn n2212(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2200);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 2856);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εμπεπηγα.
pub(crate) fn n2213(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2201);
	s.is_empty()
}

/// Words ending -υνερρωγα.
pub(crate) fn n2214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2202);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 2857);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δα.
pub(crate) fn n3(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 23);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 85);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 294);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 86);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 295);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 24);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 87);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 296);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 811);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 1494);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2203);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 25);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 88);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 297);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 812);
									s.is_empty()
								}
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
			super::push(out, 26);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 89);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 298);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 813);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1495);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θα.
pub(crate) fn n4(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 4);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 27);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 90);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 299);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 814);
									s.is_empty()
								}
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
			super::push(out, 28);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 91);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 300);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 815);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 92);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 301);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 816);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1496);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 302);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 817);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1497);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 29);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 93);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 303);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 818);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1498);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2204);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 2205);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n2875(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 30);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 94);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 304);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 819);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 1499);
											s.is_empty()
										}
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
					super::push(out, 95);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 305);
							match s {
								[] => true,
								['λ', 'ε', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1500);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 1501);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2206);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 1502);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2207);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n2876(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 2208);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n2877(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ξ', s @ ..] => {
											super::push(out, 1503);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2209);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n2878(s, out),
														['π', s @ ..] => super::t0::n2879(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 1504);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 2210);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n2880(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 1505);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 1506);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2211);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 1507);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2212);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n2881(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 1508);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2213);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κα.
pub(crate) fn n5(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 5);
	match s {
		[] => true,
		['α', s @ ..] => super::t0::n31(s, out),
		['γ', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 113);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 364);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 899);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1596);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 2291);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n2943(s, out),
														_ => false,
													}
												}
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
							super::push(out, 365);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 900);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 366);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 901);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 1597);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2292);
													s.is_empty()
												}
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
							super::push(out, 367);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 902);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1598);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 368);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 903);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1599);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2293);
													s.is_empty()
												}
												['ο', s @ ..] => {
													super::push(out, 2294);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n2944(s, out),
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
									super::push(out, 904);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1600);
											s.is_empty()
										}
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
					super::push(out, 114);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 369);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 370);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 905);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 1601);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2295);
													match s {
														[] => true,
														['μ', s @ ..] => super::t0::n2945(s, out),
														_ => false,
													}
												}
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
							super::push(out, 371);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 906);
									s.is_empty()
								}
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
			super::push(out, 33);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 115);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 372);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 907);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 1602);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 1603);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2296);
													match s {
														[] => true,
														['δ', s @ ..] => super::t0::n2946(s, out),
														_ => false,
													}
												}
												['ν', s @ ..] => {
													super::push(out, 2297);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n2947(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => super::t0::n1609(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n116(s, out),
				['λ', s @ ..] => super::t0::n117(s, out),
				['μ', s @ ..] => super::t0::n118(s, out),
				['ν', s @ ..] => super::t0::n119(s, out),
				['ξ', s @ ..] => super::t0::n120(s, out),
				['ρ', s @ ..] => super::t0::n121(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n34(s, out),
		['ι', s @ ..] => super::t0::n35(s, out),
		['λ', s @ ..] => super::t0::n36(s, out),
		['ο', s @ ..] => super::t0::n37(s, out),
		['ρ', s @ ..] => super::t0::n38(s, out),
		['υ', s @ ..] => super::t0::n39(s, out),
		['ω', s @ ..] => super::t0::n40(s, out),
		_ => false,
	}
}

/// Words ending -αλα.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 712);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1407);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2129);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2771);
											match s {
												[] => true,
												['ε', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θηλα.
pub(crate) fn n206(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 206);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 713);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1408);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μα.
pub(crate) fn n7(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 7);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 43);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 207);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 714);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1409);
									s.is_empty()
								}
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
			super::push(out, 44);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 208);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 715);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1410);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -να.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 45);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 209);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 716);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1411);
									s.is_empty()
								}
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
			super::push(out, 46);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 210);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 717);
							match s {
								[] => true,
								['γ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2130);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2131);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2772);
													match s {
														[] => true,
														['δ', s @ ..] => super::t0::n3320(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['γ', s @ ..] => {
											super::push(out, 2132);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2773);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 2774);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n3321(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2133);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 2775);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3322(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2134);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2776);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3323(s, out),
														['υ', s @ ..] => super::t0::n3324(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 2777);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3325(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 211);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 718);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1412);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2135);
											s.is_empty()
										}
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
					super::push(out, 212);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 719);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1413);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2136);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2778);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3326(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2137);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2779);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 47);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 213);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 720);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1414);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2138);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2780);
													match s {
														[] => true,
														['κ', s @ ..] => super::t0::n3327(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οα.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 48);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 214);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 721);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1415);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2139);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 2140);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2781);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 2141);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 2782);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3328(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 49);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 215);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πα.
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 50);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 216);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 723);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1416);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 217);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 724);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1417);
									s.is_empty()
								}
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
			super::push(out, 51);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 218);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 725);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1418);
									match s {
										[] => true,
										['λ', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2783);
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 2784);
													match s {
														[] => true,
														['ι', s @ ..] => super::t0::n3329(s, out),
														['ρ', s @ ..] => super::t0::n3330(s, out),
														['τ', s @ ..] => super::t0::n3331(s, out),
														_ => false,
													}
												}
												['κ', s @ ..] => {
													super::push(out, 2785);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3332(s, out),
														_ => false,
													}
												}
												['λ', s @ ..] => {
													super::push(out, 2786);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3333(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 2787);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3334(s, out),
														['ρ', s @ ..] => super::t0::n3335(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 219);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 726);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1419);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2142);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 52);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 220);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 727);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 221);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 728);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1420);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2143);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 54);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 222);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 729);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1421);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2144);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρα.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 55);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 223);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 730);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 56);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 224);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 731);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1422);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 225);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 732);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1423);
									s.is_empty()
								}
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
			super::push(out, 57);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 226);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 733);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1424);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1425);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2145);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σα.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 58);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 227);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 734);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 1426);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2146);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 2788);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 228);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 735);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1427);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2147);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2789);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 59);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 229);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 736);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1428);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2148);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2790);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3336(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τα.
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 60);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 230);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 737);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1429);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φα.
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 61);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 231);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 738);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1430);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2149);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2791);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 232);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 739);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1431);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 2150);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 740);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1432);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 741);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1433);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2151);
											s.is_empty()
										}
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
					super::push(out, 234);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 742);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1434);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2152);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2792);
													s.is_empty()
												}
												['α', s @ ..] => super::t0::n2810(s, out),
												['γ', s @ ..] => super::t0::n2811(s, out),
												['ι', s @ ..] => super::t0::n2812(s, out),
												['ο', s @ ..] => super::t0::n2813(s, out),
												['σ', s @ ..] => super::t0::n2814(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => super::t0::n743(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n62(s, out),
		['η', s @ ..] => super::t0::n63(s, out),
		['ι', s @ ..] => super::t0::n64(s, out),
		['μ', s @ ..] => super::t0::n65(s, out),
		['ο', s @ ..] => super::t0::n66(s, out),
		['υ', s @ ..] => super::t0::n67(s, out),
		_ => false,
	}
}

/// Words ending -χα.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 249);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 760);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1452);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2171);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 2830);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3384(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 250);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 761);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1453);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 251);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 762);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1454);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2172);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2831);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 252);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 763);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 253);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 764);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1455);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2173);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 2832);
													s.is_empty()
												}
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
							super::push(out, 765);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1456);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 254);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 766);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 255);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 767);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1457);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2174);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2175);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 2833);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3385(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 2834);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3386(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 2835);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3387(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2176);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2836);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3388(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2177);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2837);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n3389(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 2178);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 2838);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t0::n3390(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 69);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 256);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 768);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 257);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 769);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n70(s, out),
		['ι', s @ ..] => super::t0::n71(s, out),
		['ο', s @ ..] => super::t0::n72(s, out),
		['ρ', s @ ..] => {
			super::push(out, 73);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 276);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 277);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 790);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 1475);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t0::n74(s, out),
		['ω', s @ ..] => super::t0::n75(s, out),
		_ => false,
	}
}

/// Words ending -τιπεπονθα.
pub(crate) fn n2875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2858);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 3385);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word ανεληλυθα.
pub(crate) fn n2876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2859);
	s.is_empty()
}

/// Words ending -συνεληλυθα.
pub(crate) fn n2877(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2860);
	s.is_empty()
}

/// The word εξεληλυθα.
pub(crate) fn n2878(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2861);
	s.is_empty()
}

/// Words ending -πεξεληλυθα.
pub(crate) fn n2879(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2862);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3386);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προεληλυθα.
pub(crate) fn n2880(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2863);
	s.is_empty()
}

/// Words ending -εισεληλυθα.
pub(crate) fn n2881(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2864);
	s.is_empty()
}

/// Words ending -ακα.
pub(crate) fn n31(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 31);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 96);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 306);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 820);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1509);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 2214);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2865);
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 2866);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 2867);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3405(s, out),
														_ => false,
													}
												}
												['κ', s @ ..] => {
													super::push(out, 2868);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 97);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 307);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 821);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1510);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2215);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 2869);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 308);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 822);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1511);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2216);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2870);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 98);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 309);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 823);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1512);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2217);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2871);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 99);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 310);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 824);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1513);
									s.is_empty()
								}
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
			super::push(out, 100);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 311);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 825);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 312);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 826);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 1514);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2218);
											s.is_empty()
										}
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
					super::push(out, 313);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 827);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 1515);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2219);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2872);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 314);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 828);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 829);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1516);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2220);
											s.is_empty()
										}
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
					super::push(out, 315);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 830);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1517);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2221);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2873);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3406(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => super::t0::n316(s, out),
				['ρ', s @ ..] => super::t0::n317(s, out),
				['σ', s @ ..] => super::t0::n318(s, out),
				['τ', s @ ..] => {
					super::push(out, 319);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 837);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1525);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => super::t0::n320(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t0::n102(s, out),
		['λ', s @ ..] => super::t0::n103(s, out),
		['μ', s @ ..] => super::t0::n104(s, out),
		['ν', s @ ..] => super::t0::n105(s, out),
		['ο', s @ ..] => super::t0::n106(s, out),
		['π', s @ ..] => super::t0::n107(s, out),
		['ρ', s @ ..] => super::t0::n108(s, out),
		['τ', s @ ..] => super::t0::n109(s, out),
		['υ', s @ ..] => super::t0::n110(s, out),
		['χ', s @ ..] => super::t0::n111(s, out),
		['ω', s @ ..] => super::t0::n112(s, out),
		_ => false,
	}
}

/// Words ending -εκερδαγκα.
pub(crate) fn n2943(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2926);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3410);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ποπεφαγκα.
pub(crate) fn n2944(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2927);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3411);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μεμολυγκα.
pub(crate) fn n2945(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2928);
	s.is_empty()
}

/// Words ending -διαδεδεκα.
pub(crate) fn n2946(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2929);
	s.is_empty()
}

/// Words ending -αναδεδεκα.
pub(crate) fn n2947(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2930);
	s.is_empty()
}

/// Words ending -νδεδεκα.
pub(crate) fn n1609(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1604);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 2298);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιεκα.
pub(crate) fn n116(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 116);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 908);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λεκα.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 909);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1605);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2299);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2932);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3412);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3748(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 375);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 910);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1606);
							match s {
								[] => true,
								['τ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2933);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2934);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3413);
													match s {
														[] => true,
														['δ', s @ ..] => super::t0::n3749(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2935);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3414);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3750(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2936);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3415);
													match s {
														[] => true,
														['υ', s @ ..] => super::t0::n3751(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 376);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 911);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1607);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 912);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1608);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2300);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2937);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μεκα.
pub(crate) fn n118(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 118);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 378);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 913);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1609);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νεκα.
pub(crate) fn n119(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 119);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 379);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξεκα.
pub(crate) fn n120(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 120);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 914);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρεκα.
pub(crate) fn n121(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 121);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 915);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1610);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηκα.
pub(crate) fn n34(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 34);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 122);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 382);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 916);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1611);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2301);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2938);
													match s {
														[] => true,
														['τ', s @ ..] => super::t0::n3434(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 383);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 917);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1612);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1613);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 2302);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 2939);
													match s {
														[] => true,
														['α', s @ ..] => super::t0::n3435(s, out),
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
									super::push(out, 1614);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2303);
											match s {
												[] => true,
												['α', s @ ..] => {
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
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 384);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 918);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1615);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2304);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 919);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1616);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2305);
											s.is_empty()
										}
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
					super::push(out, 385);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 920);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1617);
									s.is_empty()
								}
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
			super::push(out, 123);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 386);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 921);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 387);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 922);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 1618);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 388);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 923);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 1619);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 924);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1620);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 2306);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2941);
													match s {
														[] => true,
														['κ', 'ι', s @ ..] => super::t0::n3436(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => super::t0::n926(s, out),
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n389(s, out),
				['ο', s @ ..] => super::t0::n390(s, out),
				['ρ', s @ ..] => super::t0::n391(s, out),
				['υ', s @ ..] => super::t0::n392(s, out),
				['ω', s @ ..] => super::t0::n393(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n124(s, out),
		['ε', s @ ..] => super::t0::n125(s, out),
		['ζ', s @ ..] => super::t0::n126(s, out),
		['θ', s @ ..] => super::t0::n127(s, out),
		['ι', s @ ..] => super::t0::n128(s, out),
		['κ', s @ ..] => super::t0::n129(s, out),
		['λ', s @ ..] => super::t0::n130(s, out),
		['μ', s @ ..] => super::t0::n131(s, out),
		['ν', s @ ..] => super::t0::n132(s, out),
		['ξ', s @ ..] => super::t0::n133(s, out),
		['ο', s @ ..] => super::t0::n134(s, out),
		['π', s @ ..] => super::t0::n135(s, out),
		['ρ', s @ ..] => super::t1::n136(s, out),
		['σ', s @ ..] => super::t1::n137(s, out),
		['τ', s @ ..] => super::t1::n138(s, out),
		['υ', s @ ..] => super::t1::n139(s, out),
		['φ', s @ ..] => super::t1::n140(s, out),
		['χ', s @ ..] => super::t1::n141(s, out),
		['ψ', s @ ..] => super::t1::n142(s, out),
		_ => false,
	}
}

/// Words ending -ικα.
pub(crate) fn n35(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 35);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1136);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1869);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 506);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1137);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1870);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2544);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3133);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 507);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1138);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1871);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1139);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1872);
									s.is_empty()
								}
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
			super::push(out, 144);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 509);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1140);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1873);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2545);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3134);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 145);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 510);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1141);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1142);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1874);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2546);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3135);
													match s {
														[] => true,
														['ι', s @ ..] => super::t1::n3575(s, out),
														_ => false,
													}
												}
												['π', s @ ..] => {
													super::push(out, 3136);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n3576(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 512);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1143);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 513);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1144);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1875);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2547);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3137);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3577(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 514);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1145);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 515);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1146);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1876);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 2548);
											s.is_empty()
										}
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
					super::push(out, 516);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1147);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1877);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2549);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3138);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3578(s, out),
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
									super::push(out, 1878);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2550);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3139);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3579(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n147(s, out),
		['θ', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 526);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1160);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1887);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2559);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => super::t1::n527(s, out),
				['ι', s @ ..] => {
					super::push(out, 528);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n1163(s, out),
						_ => false,
					}
				}
				['ν', s @ ..] => super::t1::n529(s, out),
				['υ', s @ ..] => super::t1::n530(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t1::n149(s, out),
		['λ', s @ ..] => super::t1::n150(s, out),
		['μ', s @ ..] => super::t1::n151(s, out),
		['ν', s @ ..] => super::t1::n152(s, out),
		['ο', s @ ..] => super::t1::n153(s, out),
		['π', s @ ..] => super::t1::n154(s, out),
		['ρ', s @ ..] => super::t1::n155(s, out),
		['τ', s @ ..] => super::t1::n156(s, out),
		['φ', s @ ..] => super::t1::n157(s, out),
		['χ', s @ ..] => super::t1::n158(s, out),
		['ω', s @ ..] => super::t1::n159(s, out),
		_ => false,
	}
}

/// Words ending -λκα.
pub(crate) fn n36(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 36);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1248);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1972);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2641);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2642);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3201);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3623(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2643);
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
						['σ', s @ ..] => {
							super::push(out, 1249);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1973);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2644);
											s.is_empty()
										}
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
					super::push(out, 591);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1250);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1974);
									s.is_empty()
								}
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
			super::push(out, 161);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1251);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1975);
									s.is_empty()
								}
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
			super::push(out, 162);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1252);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οκα.
pub(crate) fn n37(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 37);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1253);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1976);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2645);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 164);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1254);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1255);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1977);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2646);
											s.is_empty()
										}
										['ν', s @ ..] => {
											super::push(out, 2647);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3203);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 165);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1256);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1978);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2648);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3204);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3624(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 166);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 598);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1257);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρκα.
pub(crate) fn n38(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 38);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 167);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1258);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1979);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1259);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1980);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2649);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3205);
													s.is_empty()
												}
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
							super::push(out, 1260);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1981);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2650);
											s.is_empty()
										}
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
					super::push(out, 601);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1261);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1982);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 602);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1262);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1983);
									s.is_empty()
								}
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
			super::push(out, 168);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 603);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1263);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 1984);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2651);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 169);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 604);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1264);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1265);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1266);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1267);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1985);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1268);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1986);
									s.is_empty()
								}
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
			super::push(out, 170);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 608);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 171);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1269);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1987);
									s.is_empty()
								}
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
			super::push(out, 172);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1270);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1988);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υκα.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 173);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1271);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1989);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2652);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3206);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3625(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 612);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1272);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1990);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2653);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3207);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3626(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 613);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1273);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1991);
									s.is_empty()
								}
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
			super::push(out, 174);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1274);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1992);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n175(s, out),
		['θ', s @ ..] => {
			super::push(out, 176);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 630);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1314);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2031);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2688);
											s.is_empty()
										}
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
					super::push(out, 631);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1315);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2032);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2689);
											s.is_empty()
										}
										['θ', s @ ..] => {
											super::push(out, 2690);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3239);
													s.is_empty()
												}
												_ => false,
											}
										}
										['φ', s @ ..] => {
											super::push(out, 2691);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3240);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 178);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['λ', s @ ..] => super::t1::n1318(s, out),
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n1319(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => super::t1::n634(s, out),
				['ω', s @ ..] => super::t1::n635(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n179(s, out),
		['ν', s @ ..] => super::t1::n180(s, out),
		['ο', s @ ..] => super::t1::n181(s, out),
		['ρ', s @ ..] => super::t1::n182(s, out),
		['σ', s @ ..] => super::t1::n183(s, out),
		['τ', s @ ..] => super::t1::n184(s, out),
		['φ', s @ ..] => super::t1::n185(s, out),
		['χ', s @ ..] => super::t1::n186(s, out),
		_ => false,
	}
}

/// Words ending -ωκα.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 651);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1336);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2055);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2714);
											s.is_empty()
										}
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
					super::push(out, 652);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1337);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2056);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2715);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3264);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 188);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1338);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2057);
									s.is_empty()
								}
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
			super::push(out, 189);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 654);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1339);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2058);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2059);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2716);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3265);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3669(s, out),
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
									super::push(out, 2060);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2717);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3266);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2718);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3267);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3670(s, out),
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
									super::push(out, 2061);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2719);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2062);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2720);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3268);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n3671(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2721);
											match s {
												[] => true,
												['π', s @ ..] => {
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
				['ι', s @ ..] => {
					super::push(out, 655);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1340);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2063);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 2722);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3270);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3672(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 656);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1341);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2064);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2723);
											match s {
												[] => true,
												['λ', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 657);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1342);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2065);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2724);
											s.is_empty()
										}
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
					super::push(out, 658);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1343);
							match s {
								[] => true,
								['π', s @ ..] => {
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
					super::push(out, 659);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1344);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ζ', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1345);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2067);
									s.is_empty()
								}
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
			super::push(out, 192);
			match s {
				[] => true,
				['σ', s @ ..] => super::t1::n661(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n193(s, out),
		['κ', s @ ..] => super::t1::n194(s, out),
		['λ', s @ ..] => super::t1::n195(s, out),
		['μ', s @ ..] => super::t1::n196(s, out),
		['ν', s @ ..] => super::t1::n197(s, out),
		['ξ', s @ ..] => super::t1::n198(s, out),
		['π', s @ ..] => super::t1::n199(s, out),
		['ρ', s @ ..] => super::t1::n200(s, out),
		['σ', s @ ..] => super::t1::n201(s, out),
		['τ', s @ ..] => super::t1::n202(s, out),
		['φ', s @ ..] => super::t1::n203(s, out),
		['χ', s @ ..] => super::t1::n204(s, out),
		_ => false,
	}
}

/// Words ending -διαγεγονα.
pub(crate) fn n3320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3302);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3668);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συγγεγονα.
pub(crate) fn n3321(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3303);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 3669);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εριγεγονα.
pub(crate) fn n3322(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3304);
	s.is_empty()
}

/// Words ending -απογεγονα.
pub(crate) fn n3323(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3305);
	s.is_empty()
}

/// Words ending -υπογεγονα.
pub(crate) fn n3324(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3306);
	s.is_empty()
}

/// Words ending -προγεγονα.
pub(crate) fn n3325(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3307);
	s.is_empty()
}

/// The word απεκτονα.
pub(crate) fn n3326(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3308);
	s.is_empty()
}

/// Words ending -κεκρατυνα.
pub(crate) fn n3327(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3309);
	s.is_empty()
}

/// Words ending -εισακηκοα.
pub(crate) fn n3328(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3310);
	s.is_empty()
}

/// Words ending -ιαλελοιπα.
pub(crate) fn n3329(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3311);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3670);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ραλελοιπα.
pub(crate) fn n3330(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3312);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3671);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3859);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ταλελοιπα.
pub(crate) fn n3331(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3313);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3672);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3860);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εκλελοιπα.
pub(crate) fn n3332(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3314);
	s.is_empty()
}

/// Words ending -ελλελοιπα.
pub(crate) fn n3333(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3315);
	s.is_empty()
}

/// Words ending -πολελοιπα.
pub(crate) fn n3334(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3316);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3673);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3674);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3861);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρολελοιπα.
pub(crate) fn n3335(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3317);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3675);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επιλελησα.
pub(crate) fn n3336(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3318);
	s.is_empty()
}

/// Words ending -αγεγραφα.
pub(crate) fn n2810(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2793);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 3319);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3676);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3320);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3677);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3862);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 3321);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3678);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γγεγραφα.
pub(crate) fn n2811(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2794);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3322);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 3323);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 3679);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3864);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 3680);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιγεγραφα.
pub(crate) fn n2812(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2795);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3324);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3681);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3325);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3682);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ογεγραφα.
pub(crate) fn n2813(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2796);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3326);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3683);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 3327);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3684);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σγεγραφα.
pub(crate) fn n2814(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2797);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3328);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3685);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 3329);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 3686);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 3867);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρραφα.
pub(crate) fn n743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 743);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1435);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2153);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2798);
							match s {
								[] => true,
								['σ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -εφα.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 235);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1436);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2154);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηφα.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 236);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 745);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1437);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 237);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 746);
					match s {
						[] => true,
						['ε', s @ ..] => {
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2155);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2799);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3331);
													s.is_empty()
												}
												['π', s @ ..] => {
													super::push(out, 3332);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3706(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 2800);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3333);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2156);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2801);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3334);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2802);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3335);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 2803);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2804);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3336);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 2157);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2805);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3337);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3707(s, out),
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
									super::push(out, 2158);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2806);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3338);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιφα.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1438);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2159);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 748);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1439);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2160);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2807);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3339);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3708(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 239);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1440);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 240);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1441);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2161);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2808);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2162);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2809);
											s.is_empty()
										}
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
					super::push(out, 751);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1442);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2163);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2810);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2811);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3340);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3709(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μφα.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 752);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1443);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2164);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2812);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2813);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3341);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3710(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 2814);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3342);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2815);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3343);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3711(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οφα.
pub(crate) fn n66(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 66);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 753);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1444);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2165);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2166);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2816);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3344);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2167);
									s.is_empty()
								}
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
			super::push(out, 243);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 754);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1445);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2168);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2817);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3345);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n3712(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 244);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 755);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1446);
							match s {
								[] => true,
								['τ', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2818);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2819);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3346);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3713(s, out),
														_ => false,
													}
												}
												['ν', s @ ..] => {
													super::push(out, 3347);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3714(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2820);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3348);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3715(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 2821);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3349);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3716(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2822);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3350);
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
													super::push(out, 3351);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3717(s, out),
														['υ', s @ ..] => super::t1::n3718(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3352);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3719(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1447);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2169);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υφα.
pub(crate) fn n67(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 67);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 245);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 756);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1448);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 246);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 757);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1449);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2170);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2824);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3353);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3720(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 247);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 758);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1450);
							match s {
								[] => true,
								['κ', s @ ..] => {
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2825);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3354);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3721(s, out),
														_ => false,
													}
												}
												['ν', s @ ..] => {
													super::push(out, 3355);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3722(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3356);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3723(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 3357);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3724(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['γ', s @ ..] => {
											super::push(out, 2826);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3358);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 3359);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3725(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 2827);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3360);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3726(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3361);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3727(s, out),
														_ => false,
													}
												}
												['φ', s @ ..] => {
													super::push(out, 3362);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3728(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 2828);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3363);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2829);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3364);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3729(s, out),
														['υ', s @ ..] => super::t1::n3730(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3365);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3731(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 248);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 759);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1451);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αδεδιδαχα.
pub(crate) fn n3384(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3366);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 3713);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3878);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ανατεταχα.
pub(crate) fn n3385(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3367);
	s.is_empty()
}

/// Words ending -αρατεταχα.
pub(crate) fn n3386(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3368);
	s.is_empty()
}

/// Words ending -ετατεταχα.
pub(crate) fn n3387(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3369);
	s.is_empty()
}

/// Words ending -επιτεταχα.
pub(crate) fn n3388(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3370);
	s.is_empty()
}

/// Words ending -συντεταχα.
pub(crate) fn n3389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3371);
	s.is_empty()
}

/// Words ending -ροστεταχα.
pub(crate) fn n3390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3372);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3714);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηχα.
pub(crate) fn n70(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 70);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 258);
			s.is_empty()
		}
		['λ', s @ ..] => {
			super::push(out, 259);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 770);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1458);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 260);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 771);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1459);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2179);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2839);
											s.is_empty()
										}
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
					super::push(out, 772);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1460);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 261);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 773);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 262);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 774);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1461);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 263);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 775);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 776);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1462);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2180);
									s.is_empty()
								}
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
			super::push(out, 264);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 777);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1463);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 265);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 778);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1464);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2181);
									s.is_empty()
								}
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
			super::push(out, 266);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 779);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιχα.
pub(crate) fn n71(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 71);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 267);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 780);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1466);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2182);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2840);
											match s {
												[] => true,
												['ε', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 268);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 781);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1467);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2183);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2841);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3374);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3734(s, out),
														['υ', s @ ..] => super::t1::n3735(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 782);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1468);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2184);
									s.is_empty()
								}
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
			super::push(out, 269);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 783);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1469);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2185);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2186);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2842);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3375);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3736(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 270);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 784);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οχα.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 785);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1471);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 272);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 786);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1472);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2187);
									match s {
										[] => true,
										['ρ', s @ ..] => {
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
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 274);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 787);
					match s {
						[] => true,
						['ε', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2188);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2189);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2844);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3376);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2190);
									match s {
										[] => true,
										['α', s @ ..] => {
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
				['π', s @ ..] => {
					super::push(out, 788);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1473);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2191);
									s.is_empty()
								}
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
			super::push(out, 275);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 789);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1474);
							match s {
								[] => true,
								['ε', s @ ..] => {
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2846);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3377);
													s.is_empty()
												}
												['υ', s @ ..] => {
													super::push(out, 3378);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3737(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 2847);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3379);
													s.is_empty()
												}
												['υ', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υχα.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 278);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 791);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1476);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 792);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1477);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2192);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2848);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2849);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3381);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2850);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3382);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 279);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 793);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1478);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2193);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 794);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1479);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2194);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				['ω', s @ ..] => {
					super::push(out, 795);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1480);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2195);
									s.is_empty()
								}
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
			super::push(out, 280);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 796);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1481);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωχα.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 281);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 797);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1482);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2196);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2852);
											s.is_empty()
										}
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
					super::push(out, 798);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1483);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 282);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 799);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1484);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2197);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2853);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2854);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3383);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3738(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 2855);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιβεβιβακα.
pub(crate) fn n3405(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3387);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3720);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εενυπνιακα.
pub(crate) fn n3406(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3388);
	s.is_empty()
}

/// Words ending -πιακα.
pub(crate) fn n316(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 316);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 831);
			match s {
				[] => true,
				['π', '\u{1}', s @ ..] => {
					super::push(out, 1518);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 832);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1519);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2222);
							match s {
								[] => true,
								['κ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ριακα.
pub(crate) fn n317(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 317);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 833);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1520);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2223);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2875);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1521);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2224);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2876);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σιακα.
pub(crate) fn n318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 318);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 834);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1522);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2225);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2877);
									s.is_empty()
								}
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
			super::push(out, 835);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1523);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2226);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2878);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3389);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 836);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1524);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2227);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2879);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3390);
											match s {
												[] => true,
												['ε', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χιακα.
pub(crate) fn n320(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 320);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 838);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1526);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2228);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2880);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3391);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1527);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κακα.
pub(crate) fn n102(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 102);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 840);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1528);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2229);
									s.is_empty()
								}
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
			super::push(out, 322);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 841);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1529);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2230);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2881);
											s.is_empty()
										}
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
					super::push(out, 842);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1530);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2231);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2882);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2883);
											s.is_empty()
										}
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
					super::push(out, 843);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 844);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1531);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2232);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λακα.
pub(crate) fn n103(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 103);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 324);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 845);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1532);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2233);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 325);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 846);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1533);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 326);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 847);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1534);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2234);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 848);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1535);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2235);
									s.is_empty()
								}
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
			super::push(out, 327);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 849);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1536);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 850);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1537);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2236);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μακα.
pub(crate) fn n104(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 104);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 851);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1538);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2237);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2885);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 330);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 852);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1539);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2238);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2886);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3392);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3741(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 853);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1540);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2239);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2887);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3393);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 854);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1541);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2240);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2888);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 2889);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3394);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3742(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 855);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1542);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 331);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 856);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1543);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2241);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2242);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 332);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 857);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1544);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2243);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2891);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 333);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 858);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νακα.
pub(crate) fn n105(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 105);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 334);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 859);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1545);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2244);
									s.is_empty()
								}
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
			super::push(out, 335);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 860);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1546);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2245);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2892);
											s.is_empty()
										}
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
					super::push(out, 861);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1547);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2246);
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 2893);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 336);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 862);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1548);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2247);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2894);
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
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 863);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1549);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2248);
									s.is_empty()
								}
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
			super::push(out, 337);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 864);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1550);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2249);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2895);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οακα.
pub(crate) fn n106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 106);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 338);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 865);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1551);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πακα.
pub(crate) fn n107(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 107);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 339);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 866);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1552);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2250);
									s.is_empty()
								}
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
			super::push(out, 340);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 867);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1553);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2251);
									s.is_empty()
								}
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
			super::push(out, 341);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 868);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1554);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2252);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2896);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1555);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2253);
									match s {
										[] => true,
										['σ', s @ ..] => {
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
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 342);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 869);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1556);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2254);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2898);
											match s {
												[] => true,
												['π', s @ ..] => {
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
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 343);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρακα.
pub(crate) fn n108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 108);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 344);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 870);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1557);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2255);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 2256);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2899);
											match s {
												[] => true,
												['α', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 871);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1558);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 872);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1559);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 873);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1560);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2257);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2900);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 2901);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3398);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3743(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 874);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1561);
							match s {
								[] => true,
								['τ', s @ ..] => {
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
		['ι', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1562);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2259);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2902);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3399);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 348);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1563);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1564);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2260);
									s.is_empty()
								}
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
			super::push(out, 349);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 878);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1565);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 350);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1566);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2261);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2903);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 351);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 880);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1567);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2262);
									match s {
										[] => true,
										['τ', s @ ..] => {
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
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 352);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 881);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1568);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2263);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2905);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3400);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3744(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 353);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1569);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2264);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2906);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1570);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2265);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2907);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1571);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2266);
									match s {
										[] => true,
										['π', s @ ..] => {
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
				['φ', s @ ..] => {
					super::push(out, 883);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1572);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2267);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2909);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3401);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τακα.
pub(crate) fn n109(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 109);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 354);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 884);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1573);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2268);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1574);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2269);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2270);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1575);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2271);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2910);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1576);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2272);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1577);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2273);
									match s {
										[] => true,
										['π', s @ ..] => {
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
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 355);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1578);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 356);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1579);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2274);
									s.is_empty()
								}
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
			super::push(out, 357);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1580);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2275);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2912);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3402);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 358);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1581);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2276);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2913);
											s.is_empty()
										}
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
					super::push(out, 890);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1582);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 359);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1583);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2277);
									match s {
										[] => true,
										['μ', s @ ..] => {
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
				['ε', s @ ..] => {
					super::push(out, 892);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1584);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1585);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2278);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2915);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3403);
													s.is_empty()
												}
												['ο', s @ ..] => {
													super::push(out, 3404);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3745(s, out),
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
									super::push(out, 2279);
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
						['ν', s @ ..] => {
							super::push(out, 1586);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2280);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2281);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 1587);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2282);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1588);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2283);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2917);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3405);
													s.is_empty()
												}
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
							super::push(out, 1589);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2284);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2285);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2918);
											s.is_empty()
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 2286);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2919);
											s.is_empty()
										}
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
					super::push(out, 893);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1590);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2287);
									s.is_empty()
								}
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
			super::push(out, 360);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 894);
					match s {
						[] => true,
						['τ', s @ ..] => super::t1::n1596(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υακα.
pub(crate) fn n110(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 110);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 361);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 895);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1592);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2289);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2921);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3406);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2922);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3407);
													s.is_empty()
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 2923);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3408);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3746(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2924);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3409);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3747(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 896);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1593);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2290);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2925);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χακα.
pub(crate) fn n111(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 111);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 362);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 897);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1594);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωακα.
pub(crate) fn n112(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 112);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 363);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 898);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1595);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ετακεκαλεκα.
pub(crate) fn n3748(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3729);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 3883);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -διατετελεκα.
pub(crate) fn n3749(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3730);
	s.is_empty()
}

/// Words ending -επιτετελεκα.
pub(crate) fn n3750(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3731);
	s.is_empty()
}

/// Words ending -υποτετελεκα.
pub(crate) fn n3751(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3732);
	s.is_empty()
}

/// Words ending -ταλελαβηκα.
pub(crate) fn n3434(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3416);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3733);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3884);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -αμφιβεβηκα.
pub(crate) fn n3435(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3417);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3734);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ικεχορηγηκα.
pub(crate) fn n3436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3418);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3735);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3885);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τηγηκα.
pub(crate) fn n926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 925);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1621);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2307);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2942);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3419);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3736);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιγηκα.
pub(crate) fn n389(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 389);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 926);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1622);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2308);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ογηκα.
pub(crate) fn n390(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 390);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 927);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1623);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2309);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2943);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 3420);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3737);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1624);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2310);
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ργηκα.
pub(crate) fn n391(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 391);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 928);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1625);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 1626);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2311);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2945);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3421);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 929);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1627);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2312);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2946);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3422);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2313);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2947);
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
						['τ', s @ ..] => {
							super::push(out, 2314);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2948);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3424);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3738);
													match s {
														[] => true,
														['λ', s @ ..] => super::t1::n3905(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 930);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1628);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υγηκα.
pub(crate) fn n392(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 392);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 931);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1629);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2315);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2949);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3425);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3739);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 932);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1630);
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
		['τ', s @ ..] => {
			super::push(out, 933);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1631);
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
		_ => false,
	}
}

/// Words ending -ωγηκα.
pub(crate) fn n393(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 393);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 934);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1632);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2318);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2950);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3426);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δηκα.
pub(crate) fn n124(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 124);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1633);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2319);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2951);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1634);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2320);
									s.is_empty()
								}
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
			super::push(out, 396);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1635);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2321);
									s.is_empty()
								}
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
			super::push(out, 397);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1636);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2322);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2952);
											s.is_empty()
										}
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
					super::push(out, 939);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1637);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1638);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2323);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2953);
											match s {
												[] => true,
												['π', s @ ..] => {
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
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 940);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1639);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2324);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 2954);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3429);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n3759(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εηκα.
pub(crate) fn n125(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 125);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 942);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζηκα.
pub(crate) fn n126(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 126);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 400);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 943);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1641);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2325);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2955);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3430);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 401);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 402);
			s.is_empty()
		}
		['ω', s @ ..] => {
			super::push(out, 403);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -θηκα.
pub(crate) fn n127(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 127);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 944);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1642);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 945);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1643);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2326);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1644);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2327);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2956);
											match s {
												[] => true,
												['φ', s @ ..] => {
													super::push(out, 3431);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3760(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 946);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1645);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2328);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2957);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 947);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1646);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 1647);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2329);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1648);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2330);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2958);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1649);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2331);
									s.is_empty()
								}
								['ρ', s @ ..] => {
									super::push(out, 2332);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2959);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1650);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2333);
									s.is_empty()
								}
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
			super::push(out, 406);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 948);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1651);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2334);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 2960);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3433);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3761(s, out),
														_ => false,
													}
												}
												['ι', s @ ..] => {
													super::push(out, 3434);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3762(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 3435);
													match s {
														[] => true,
														['ο', s @ ..] => super::t1::n3763(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 407);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 949);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 408);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 950);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1652);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2335);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2961);
											s.is_empty()
										}
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
					super::push(out, 951);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 409);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 952);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1653);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2336);
									s.is_empty()
								}
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
			super::push(out, 410);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 953);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1654);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2337);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2962);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 411);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 954);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1655);
							match s {
								[] => true,
								['ω', s @ ..] => {
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
		['υ', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 955);
					match s {
						[] => true,
						['λ', s @ ..] => super::t1::n1661(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιηκα.
pub(crate) fn n128(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 128);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 956);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1657);
							match s {
								[] => true,
								['π', s @ ..] => {
									match s {
										[] => true,
										['ι', s @ ..] => {
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3437);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3764(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 3438);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3765(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['μ', s @ ..] => {
											super::push(out, 2964);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3439);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2965);
											match s {
												[] => true,
												['χ', s @ ..] => {
													super::push(out, 3440);
													s.is_empty()
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 2966);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3441);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3766(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1658);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2340);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2967);
											match s {
												[] => true,
												['γ', 'η', '\u{1}', s @ ..] => {
													super::push(out, 3442);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2341);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2968);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 414);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 957);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1659);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2342);
									s.is_empty()
								}
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
			super::push(out, 415);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1660);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2343);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κηκα.
pub(crate) fn n129(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 129);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 959);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1661);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2344);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2969);
											match s {
												[] => true,
												['ο', s @ ..] => {
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
						['υ', s @ ..] => {
							super::push(out, 1662);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2345);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2970);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3444);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2971);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3445);
													s.is_empty()
												}
												['χ', s @ ..] => {
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
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 417);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1663);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2346);
									s.is_empty()
								}
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
			super::push(out, 418);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1664);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2347);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				['ν', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1665);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2348);
									s.is_empty()
								}
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
			super::push(out, 419);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1666);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2349);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2973);
											match s {
												[] => true,
												['ρ', s @ ..] => {
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
		['σ', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 964);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1667);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 965);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1668);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2350);
									match s {
										[] => true,
										['β', s @ ..] => {
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
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 421);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 966);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 967);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1669);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 968);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1671);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2351);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 969);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1672);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 970);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1673);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2352);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2975);
											s.is_empty()
										}
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
					super::push(out, 971);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1674);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2353);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ληκα.
pub(crate) fn n130(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 130);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 972);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1675);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2354);
									s.is_empty()
								}
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
			super::push(out, 423);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 973);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1676);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2355);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2356);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2976);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3448);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 2977);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3449);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2978);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3450);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3767(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2979);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3451);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3768(s, out),
														_ => false,
													}
												}
												['ε', s @ ..] => {
													super::push(out, 3452);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3769(s, out),
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
									super::push(out, 2357);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2980);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3453);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 2358);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2981);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 2359);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2982);
											s.is_empty()
										}
										['υ', s @ ..] => {
											super::push(out, 2983);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3454);
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
										['π', s @ ..] => {
											super::push(out, 2984);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3455);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2985);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3456);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2361);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2986);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3457);
													match s {
														[] => true,
														['υ', s @ ..] => super::t1::n3770(s, out),
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
									super::push(out, 2362);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 2987);
											match s {
												[] => true,
												['ε', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 974);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1677);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2363);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2988);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 975);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1679);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 976);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1680);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2364);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1681);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2365);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2989);
											s.is_empty()
										}
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
					super::push(out, 977);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1682);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 978);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n1688(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', 'η', '\u{1}', s @ ..] => {
			super::push(out, 426);
			s.is_empty()
		}
		['ι', s @ ..] => super::t1::n427(s, out),
		['κ', s @ ..] => super::t1::n428(s, out),
		['ο', s @ ..] => super::t1::n429(s, out),
		['π', s @ ..] => super::t1::n430(s, out),
		['τ', 'ε', 'τ', '\u{1}', s @ ..] => super::t1::n431(s, out),
		_ => false,
	}
}

/// Words ending -μηκα.
pub(crate) fn n131(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 131);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 432);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 988);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1694);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2383);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 989);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1695);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2384);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3003);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3468);
													s.is_empty()
												}
												['α', s @ ..] => {
													super::push(out, 3469);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 3470);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3775(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 433);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 990);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1696);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2385);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3004);
											match s {
												[] => true,
												['π', s @ ..] => {
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3776(s, out),
														['μ', s @ ..] => super::t1::n3777(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 991);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1697);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2386);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 992);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1698);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1699);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2387);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3005);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 434);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 993);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 994);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1700);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2388);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3006);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 3471);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3778(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1701);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2389);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3007);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3472);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 435);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1702);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2390);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3008);
											s.is_empty()
										}
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
					super::push(out, 996);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1703);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2391);
									s.is_empty()
								}
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
			super::push(out, 436);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 997);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1704);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2392);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3009);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3473);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3779(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 437);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1705);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 438);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 999);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1706);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2393);
									match s {
										[] => true,
										['τ', s @ ..] => super::t1::n3028(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n439(s, out),
		['ρ', s @ ..] => super::t1::n440(s, out),
		['σ', s @ ..] => super::t1::n441(s, out),
		['τ', s @ ..] => super::t1::n442(s, out),
		['υ', s @ ..] => super::t1::n443(s, out),
		_ => false,
	}
}

/// Words ending -νηκα.
pub(crate) fn n132(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 132);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 444);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1009);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1720);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2411);
									match s {
										[] => true,
										['π', s @ ..] => {
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
					super::push(out, 1010);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1721);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2412);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3029);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
				['π', s @ ..] => {
					super::push(out, 1011);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1722);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2413);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1012);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1723);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2414);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1013);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1724);
							match s {
								[] => true,
								['μ', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 445);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1014);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1725);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2416);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 3030);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3487);
													s.is_empty()
												}
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
							super::push(out, 1726);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2417);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3031);
											s.is_empty()
										}
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
					super::push(out, 1015);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1727);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2418);
									s.is_empty()
								}
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
			super::push(out, 446);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1016);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1728);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2419);
									s.is_empty()
								}
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
			super::push(out, 447);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1017);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1729);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2420);
									s.is_empty()
								}
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
			super::push(out, 448);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1018);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1730);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2421);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 1019);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1731);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2422);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3032);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3488);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3786(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1020);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1732);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2423);
									match s {
										[] => true,
										['π', s @ ..] => {
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
				['θ', s @ ..] => {
					super::push(out, 1021);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1733);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2424);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 1022);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1734);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2425);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3034);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 3035);
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
			super::push(out, 449);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1023);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1735);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 450);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1024);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1736);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1025);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1737);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2426);
									match s {
										[] => true,
										['μ', s @ ..] => {
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
				['υ', s @ ..] => {
					super::push(out, 1026);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 451);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1027);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1738);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1028);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1739);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n452(s, out),
		['π', s @ ..] => {
			super::push(out, 453);
			match s {
				[] => true,
				['ι', s @ ..] => super::t1::n1040(s, out),
				['υ', 'ρ', 'γ', 'η', '\u{1}', s @ ..] => {
					super::push(out, 1040);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n454(s, out),
		['ω', s @ ..] => super::t1::n455(s, out),
		_ => false,
	}
}

/// Words ending -ξηκα.
pub(crate) fn n133(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 133);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 456);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1046);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οηκα.
pub(crate) fn n134(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 134);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 457);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1047);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1757);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2444);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3054);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3504);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3793(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 458);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1048);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1758);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1049);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1759);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2445);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3055);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3505);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3056);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3506);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ι', s @ ..] => {
									super::push(out, 2446);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3057);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3507);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 2447);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3058);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2448);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3059);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3508);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 3060);
											match s {
												[] => true,
												['π', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πηκα.
pub(crate) fn n135(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 135);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 459);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1050);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1760);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2449);
									s.is_empty()
								}
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
			super::push(out, 460);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1051);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1761);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2450);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3061);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 461);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1052);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1762);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2451);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1763);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2452);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3062);
											match s {
												[] => true,
												['θ', s @ ..] => {
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
				['τ', s @ ..] => {
					super::push(out, 1054);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1764);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2453);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3063);
											match s {
												[] => true,
												['π', s @ ..] => {
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
		['υ', s @ ..] => {
			super::push(out, 462);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1765);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2454);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1766);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2455);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3064);
											s.is_empty()
										}
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
					super::push(out, 1057);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1767);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2456);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3065);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 463);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1768);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2457);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1769);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2458);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

