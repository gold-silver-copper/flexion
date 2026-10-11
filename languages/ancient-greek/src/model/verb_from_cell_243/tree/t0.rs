use flexion::Path;

/// Every word.
pub(crate) fn n0(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 0);
	match s {
		['α', s @ ..] => super::t0::n1(s, out),
		['ν', s @ ..] => super::t0::n2(s, out),
		_ => false,
	}
}

/// Words ending -α.
pub(crate) fn n1(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 24);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 105);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 444);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 1162);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 2000);
													match s {
														[] => true,
														['κ', s @ ..] => super::t0::n2732(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 4);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 25);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 106);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 445);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1163);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2001);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 5);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 26);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 107);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 446);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1164);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 6);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 27);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 108);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 447);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 28);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 109);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 448);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 1165);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2002);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 449);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1166);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2003);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ξ', s @ ..] => {
									super::push(out, 450);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1167);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 451);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1168);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 452);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1169);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 2004);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 7);
			match s {
				[] => true,
				['γ', s @ ..] => super::t0::n29(s, out),
				['η', s @ ..] => super::t0::n30(s, out),
				['ω', s @ ..] => {
					super::push(out, 31);
					match s {
						[] => true,
						['δ', s @ ..] => super::t0::n117(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => super::t0::n8(s, out),
		['μ', s @ ..] => super::t0::n9(s, out),
		['ν', s @ ..] => super::t0::n10(s, out),
		['ξ', s @ ..] => super::t0::n11(s, out),
		['π', s @ ..] => super::t0::n12(s, out),
		['ρ', s @ ..] => super::t0::n13(s, out),
		['σ', s @ ..] => super::t0::n14(s, out),
		['τ', s @ ..] => super::t0::n15(s, out),
		['υ', s @ ..] => super::t0::n16(s, out),
		['χ', s @ ..] => super::t0::n17(s, out),
		['ψ', s @ ..] => super::t0::n18(s, out),
		_ => false,
	}
}

/// Words ending -ν.
pub(crate) fn n2(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 19);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 80);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 396);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1087);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 1888);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 2608);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 20);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 81);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 397);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1088);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1889);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2609);
													s.is_empty()
												}
												['σ', s @ ..] => {
													super::push(out, 2610);
													match s {
														[] => true,
														['ι', s @ ..] => super::t0::n3139(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 1890);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2611);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 1891);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 2612);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ξ', s @ ..] => {
									super::push(out, 1089);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 1892);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2613);
													s.is_empty()
												}
												['π', s @ ..] => {
													super::push(out, 2614);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3140(s, out),
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
									super::push(out, 1090);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 1893);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2615);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3141(s, out),
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
									super::push(out, 1091);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1894);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1092);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 1895);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2616);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 1896);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2617);
													match s {
														[] => true,
														['υ', s @ ..] => super::t0::n3142(s, out),
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
									super::push(out, 1093);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
											super::push(out, 1897);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 1898);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2618);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3143(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => super::t0::n1899(s, out),
										_ => false,
									}
								}
								['τ', s @ ..] => super::t0::n1094(s, out),
								['φ', s @ ..] => super::t0::n1095(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => super::t0::n82(s, out),
				['τ', s @ ..] => super::t0::n83(s, out),
				_ => false,
			}
		}
		['ο', s @ ..] => super::t0::n21(s, out),
		['υ', s @ ..] => super::t0::n22(s, out),
		['ω', s @ ..] => super::t0::n23(s, out),
		_ => false,
	}
}

/// Words ending -κτυπησαα.
pub(crate) fn n2732(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2732);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3229);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γκα.
pub(crate) fn n29(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 29);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 110);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 453);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1170);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2005);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2006);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2733);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2734);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3230);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3479(s, out),
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
									super::push(out, 2007);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2735);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3231);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 2736);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3232);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2008);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2737);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3233);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2009);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2738);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2010);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2739);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 2011);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2740);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
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
pub(crate) fn n30(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 30);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 111);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 112);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 454);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1171);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 455);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1172);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1173);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2012);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2013);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2741);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3234);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3480(s, out),
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
										['σ', s @ ..] => {
											super::push(out, 2742);
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
							super::push(out, 1174);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2015);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2743);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1175);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2016);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2017);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2744);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3236);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['υ', s @ ..] => {
									super::push(out, 2018);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1176);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2019);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2745);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						['σ', s @ ..] => {
							super::push(out, 1177);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2020);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2746);
											match s {
												[] => true,
												['π', s @ ..] => {
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
						['τ', s @ ..] => {
							super::push(out, 1178);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2021);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2747);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 113);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 456);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1179);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2022);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 457);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 458);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1180);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 114);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 459);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 115);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 460);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1181);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 116);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 461);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 462);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -δωκα.
pub(crate) fn n117(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 117);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 463);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1182);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2023);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2748);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3239);
											s.is_empty()
										}
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
					super::push(out, 1184);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2024);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2749);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2750);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 2025);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λα.
pub(crate) fn n8(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 8);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 32);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 118);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 464);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1186);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 119);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 465);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 120);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 121);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 466);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1187);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2026);
											s.is_empty()
										}
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
					super::push(out, 122);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 467);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 1188);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2027);
											s.is_empty()
										}
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
					super::push(out, 123);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 468);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1189);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2028);
											s.is_empty()
										}
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
					super::push(out, 124);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 469);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1190);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ψ', s @ ..] => {
					super::push(out, 125);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 470);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 33);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 126);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 471);
							match s {
								[] => true,
								['γ', 'η', 'ρ', s @ ..] => {
									super::push(out, 1191);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2029);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 2751);
													s.is_empty()
												}
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
							super::push(out, 472);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 127);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 473);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 34);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 128);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 474);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1192);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μα.
pub(crate) fn n9(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 9);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 35);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 129);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 475);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 36);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 130);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 476);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1193);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 477);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1194);
									s.is_empty()
								}
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
			super::push(out, 37);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 131);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 478);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1195);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 2030);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2752);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
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
pub(crate) fn n10(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 10);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 38);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 132);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 479);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1196);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2031);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2753);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 133);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 480);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1197);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2032);
											s.is_empty()
										}
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
					super::push(out, 134);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 481);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1198);
									s.is_empty()
								}
								_ => false,
							}
						}
						['λ', s @ ..] => {
							super::push(out, 482);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 1199);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2033);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 483);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1200);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 484);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1201);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2034);
											s.is_empty()
										}
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
					super::push(out, 135);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 485);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1202);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 2035);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2754);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3241(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 136);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 486);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1203);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2036);
											s.is_empty()
										}
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
					super::push(out, 137);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 487);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1204);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2037);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 488);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 1205);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2038);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2755);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 138);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 489);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1206);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2039);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2756);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 139);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 490);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1207);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2040);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 2757);
													s.is_empty()
												}
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
							super::push(out, 491);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1208);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 492);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1209);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 493);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 1210);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2041);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 494);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1211);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 495);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1212);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => super::t0::n496(s, out),
						['φ', s @ ..] => {
							super::push(out, 497);
							match s {
								[] => true,
								['υ', s @ ..] => super::t0::n1214(s, out),
								_ => false,
							}
						}
						['ω', s @ ..] => super::t0::n498(s, out),
						_ => false,
					}
				}
				['φ', s @ ..] => super::t0::n140(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n39(s, out),
		['ι', s @ ..] => super::t0::n40(s, out),
		['υ', s @ ..] => super::t0::n41(s, out),
		_ => false,
	}
}

/// Words ending -ξα.
pub(crate) fn n11(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 11);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 42);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 165);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 528);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1243);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 166);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 529);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 1244);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2062);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 2772);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 167);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 530);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1245);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2063);
											s.is_empty()
										}
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
					super::push(out, 168);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 531);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 1246);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 1247);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2064);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2773);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3247(s, out),
														_ => false,
													}
												}
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
							super::push(out, 532);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1248);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2065);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2774);
													match s {
														[] => true,
														['ξ', s @ ..] => super::t0::n3248(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', 'α', s @ ..] => {
											super::push(out, 2066);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 2067);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 533);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 169);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 534);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 170);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 535);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1249);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2068);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2775);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3249(s, out),
														_ => false,
													}
												}
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
							super::push(out, 536);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1250);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2069);
											s.is_empty()
										}
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
					super::push(out, 171);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 537);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 1251);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2070);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 538);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1252);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2071);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 2776);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n3250(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n172(s, out),
				['σ', s @ ..] => {
					super::push(out, 173);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 546);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1257);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => super::t0::n174(s, out),
				['υ', s @ ..] => super::t0::n175(s, out),
				['φ', s @ ..] => super::t0::n176(s, out),
				_ => false,
			}
		}
		['γ', s @ ..] => super::t0::n43(s, out),
		['ε', s @ ..] => super::t0::n44(s, out),
		['η', s @ ..] => super::t0::n45(s, out),
		['ι', s @ ..] => super::t0::n46(s, out),
		['λ', s @ ..] => super::t0::n47(s, out),
		['ο', s @ ..] => super::t0::n48(s, out),
		['ρ', s @ ..] => super::t0::n49(s, out),
		['υ', s @ ..] => super::t0::n50(s, out),
		['ω', s @ ..] => super::t0::n51(s, out),
		_ => false,
	}
}

/// Words ending -πα.
pub(crate) fn n12(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 12);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 52);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 226);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 635);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 1347);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2135);
											match s {
												[] => true,
												['π', s @ ..] => {
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
pub(crate) fn n13(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 13);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 53);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 227);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 228);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 636);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1348);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2136);
											s.is_empty()
										}
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
					super::push(out, 229);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 637);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 230);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 638);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 231);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 639);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 232);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 640);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 233);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 641);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 234);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 642);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1349);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 643);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1350);
									s.is_empty()
								}
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
			super::push(out, 54);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 235);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 644);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1351);
									s.is_empty()
								}
								_ => false,
							}
						}
						['δ', s @ ..] => {
							super::push(out, 645);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1352);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2137);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 646);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 647);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 1353);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2138);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 648);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1354);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2139);
											s.is_empty()
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 1355);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2140);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 55);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 236);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 649);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1356);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 237);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 650);
							s.is_empty()
						}
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
pub(crate) fn n14(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 14);
	match s {
		[] => true,
		['α', s @ ..] => super::t0::n56(s, out),
		['ε', s @ ..] => {
			super::push(out, 57);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 258);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 723);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1428);
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
				['δ', s @ ..] => {
					super::push(out, 259);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 724);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1429);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 725);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1430);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2205);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['ζ', s @ ..] => {
					super::push(out, 260);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 726);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1431);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 1432);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2206);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 261);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 727);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1433);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2207);
											s.is_empty()
										}
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
					super::push(out, 262);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 728);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1434);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2208);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 2849);
													s.is_empty()
												}
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
							super::push(out, 729);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 1435);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 263);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 730);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1436);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
											super::push(out, 2209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 731);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 1437);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2210);
											match s {
												[] => true,
												['π', 'α', s @ ..] => {
													super::push(out, 2850);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3276(s, out),
														_ => false,
													}
												}
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
							super::push(out, 732);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1438);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1439);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2211);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 264);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 733);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 265);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 734);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 1440);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2212);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 1441);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 266);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 735);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1442);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 267);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 736);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1443);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 1444);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2213);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 737);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 1445);
									match s {
										[] => true,
										['ε', s @ ..] => super::t0::n2214(s, out),
										_ => false,
									}
								}
								['τ', s @ ..] => super::t0::n1446(s, out),
								_ => false,
							}
						}
						['τ', s @ ..] => super::t0::n738(s, out),
						_ => false,
					}
				}
				['τ', s @ ..] => super::t0::n268(s, out),
				['χ', s @ ..] => super::t0::n269(s, out),
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n58(s, out),
		['ι', s @ ..] => super::t0::n59(s, out),
		['λ', s @ ..] => super::t0::n60(s, out),
		['ο', s @ ..] => super::t0::n61(s, out),
		['ρ', s @ ..] => super::t0::n62(s, out),
		['σ', s @ ..] => super::t0::n63(s, out),
		['υ', s @ ..] => super::t0::n64(s, out),
		['ω', s @ ..] => super::t0::n65(s, out),
		_ => false,
	}
}

/// Words ending -τα.
pub(crate) fn n15(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 15);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 66);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 356);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1025);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υα.
pub(crate) fn n16(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 16);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 67);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 357);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1026);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1831);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χα.
pub(crate) fn n17(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 17);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 68);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 358);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1027);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1832);
									s.is_empty()
								}
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
			super::push(out, 69);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 359);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1028);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1833);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2570);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ψα.
pub(crate) fn n18(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 18);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 70);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 360);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1029);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 361);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1030);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1834);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1031);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1835);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2571);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3115);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 362);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1032);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 363);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 364);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1033);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1836);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 365);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1034);
							s.is_empty()
						}
						['ε', s @ ..] => {
							super::push(out, 1035);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1036);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 366);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1037);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1837);
									s.is_empty()
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
						['γ', s @ ..] => {
							super::push(out, 1038);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1838);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2572);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3116);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3426(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1039);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1839);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2573);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3117);
													s.is_empty()
												}
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
							super::push(out, 1040);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 1840);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 71);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 368);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1041);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 1841);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2575);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['β', s @ ..] => {
							super::push(out, 1042);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 1842);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2576);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3118);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3427(s, out),
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
									super::push(out, 1843);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2577);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3119);
													s.is_empty()
												}
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
							super::push(out, 1043);
							s.is_empty()
						}
						['κ', s @ ..] => {
							super::push(out, 1044);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1844);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2578);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3120);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 369);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1045);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1845);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n370(s, out),
				['τ', s @ ..] => {
					super::push(out, 371);
					match s {
						[] => true,
						['σ', s @ ..] => super::t0::n1052(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => super::t0::n72(s, out),
		['ι', s @ ..] => super::t0::n73(s, out),
		['λ', s @ ..] => super::t0::n74(s, out),
		['μ', s @ ..] => super::t0::n75(s, out),
		['ο', s @ ..] => super::t0::n76(s, out),
		['ρ', s @ ..] => super::t0::n77(s, out),
		['υ', s @ ..] => super::t0::n78(s, out),
		['ω', s @ ..] => super::t0::n79(s, out),
		_ => false,
	}
}

/// Words ending -ισανεβην.
pub(crate) fn n3139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3138);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3430);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επεξεβην.
pub(crate) fn n3140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3139);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3431);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word προεβην.
pub(crate) fn n3141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3140);
	s.is_empty()
}

/// Words ending -υπερεβην.
pub(crate) fn n3142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3141);
	s.is_empty()
}

/// The word εισεβην.
pub(crate) fn n3143(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3142);
	s.is_empty()
}

/// Words ending -οσεβην.
pub(crate) fn n1899(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1899);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 2619);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3143);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -τεβην.
pub(crate) fn n1094(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1094);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1900);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2620);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φεβην.
pub(crate) fn n1095(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1095);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1901);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2621);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3144);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λην.
pub(crate) fn n82(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 82);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 398);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1902);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -την.
pub(crate) fn n83(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 83);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 399);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1903);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2622);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3145);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3433);
													s.is_empty()
												}
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
							super::push(out, 1904);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2623);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3146);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ον.
pub(crate) fn n21(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 21);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 84);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 400);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1098);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 1905);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 2624);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3147);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3436(s, out),
														['π', s @ ..] => super::t0::n3437(s, out),
														_ => false,
													}
												}
												['υ', s @ ..] => {
													super::push(out, 3148);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 2625);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3149);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3438(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 2626);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3150);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ρ', s @ ..] => {
											super::push(out, 2627);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3151);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3439(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 2628);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3152);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t0::n3440(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2629);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3153);
													match s {
														[] => true,
														['κ', s @ ..] => super::t0::n3441(s, out),
														_ => false,
													}
												}
												['ε', s @ ..] => {
													super::push(out, 3154);
													match s {
														[] => true,
														['μ', s @ ..] => super::t0::n3442(s, out),
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
									super::push(out, 1906);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2630);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3155);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 85);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 401);
					match s {
						[] => true,
						['γ', s @ ..] => super::t0::n1099(s, out),
						['ρ', s @ ..] => {
							super::push(out, 1100);
							match s {
								[] => true,
								['τ', s @ ..] => super::t0::n1908(s, out),
								_ => false,
							}
						}
						['φ', s @ ..] => super::t0::n1101(s, out),
						_ => false,
					}
				}
				['ι', s @ ..] => super::t0::n402(s, out),
				['υ', s @ ..] => super::t0::n403(s, out),
				_ => false,
			}
		}
		['δ', s @ ..] => super::t0::n86(s, out),
		['θ', s @ ..] => super::t0::n87(s, out),
		['ι', s @ ..] => super::t0::n88(s, out),
		['κ', s @ ..] => super::t0::n89(s, out),
		['λ', s @ ..] => super::t0::n90(s, out),
		['μ', s @ ..] => super::t0::n91(s, out),
		['ν', s @ ..] => super::t0::n92(s, out),
		['ξ', s @ ..] => super::t0::n93(s, out),
		['π', s @ ..] => super::t0::n94(s, out),
		['ρ', s @ ..] => super::t0::n95(s, out),
		['σ', s @ ..] => super::t0::n96(s, out),
		['τ', s @ ..] => super::t0::n97(s, out),
		['υ', s @ ..] => super::t0::n98(s, out),
		['φ', s @ ..] => super::t0::n99(s, out),
		['χ', s @ ..] => super::t0::n100(s, out),
		_ => false,
	}
}

/// Words ending -υν.
pub(crate) fn n22(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 22);
	match s {
		[] => true,
		['φ', s @ ..] => {
			super::push(out, 101);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 440);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1157);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1158);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 1994);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2726);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ων.
pub(crate) fn n23(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 23);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 102);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 441);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1159);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 103);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 442);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 104);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 443);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1161);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 1995);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 1996);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2727);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 1997);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2728);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3225);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 1998);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2729);
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
								['τ', s @ ..] => {
									super::push(out, 1999);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2730);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3227);
													match s {
														[] => true,
														['ο', s @ ..] => super::t0::n3478(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ε', s @ ..] => {
											super::push(out, 2731);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -περιηνεγκα.
pub(crate) fn n3479(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3477);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3578);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word επενεθηκα.
pub(crate) fn n3480(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3478);
	s.is_empty()
}

/// Words ending -πελευκανα.
pub(crate) fn n3241(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3240);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3479);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τρανα.
pub(crate) fn n496(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 496);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1213);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2042);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2758);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υφρανα.
pub(crate) fn n1214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1214);
	s.is_empty()
}

/// Words ending -ωρανα.
pub(crate) fn n498(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 498);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1215);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φανα.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 499);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηνα.
pub(crate) fn n39(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 39);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 141);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 500);
					s.is_empty()
				}
				['ω', s @ ..] => {
					super::push(out, 501);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1217);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 142);
			s.is_empty()
		}
		['μ', s @ ..] => {
			super::push(out, 143);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 502);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1218);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2044);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 503);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1219);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2045);
									s.is_empty()
								}
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
			super::push(out, 144);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 145);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 146);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 504);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1220);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 147);
			s.is_empty()
		}
		['φ', s @ ..] => {
			super::push(out, 148);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 505);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1221);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2046);
									s.is_empty()
								}
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
			super::push(out, 149);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 506);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1222);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2047);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2759);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ινα.
pub(crate) fn n40(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 40);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 150);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 507);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1223);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2048);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 2760);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 151);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 508);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1224);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 509);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1225);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 510);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1226);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2049);
									s.is_empty()
								}
								['ξ', s @ ..] => {
									super::push(out, 2050);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2761);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3241);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2051);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2762);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3242);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2052);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1227);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2053);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 2763);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3243);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 2764);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3244);
													match s {
														[] => true,
														['κ', s @ ..] => super::t0::n3482(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 152);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 511);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1228);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 153);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 512);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1229);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 154);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 513);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνα.
pub(crate) fn n41(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 41);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 155);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 514);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1230);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2054);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2765);
											s.is_empty()
										}
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
					super::push(out, 515);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1231);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 516);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1232);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2055);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2766);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 156);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 517);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1233);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2056);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2767);
											s.is_empty()
										}
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
					super::push(out, 518);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1234);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 157);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 519);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 158);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 520);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1235);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2057);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2768);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 159);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 521);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 160);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 522);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1236);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2058);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2769);
											s.is_empty()
										}
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
					super::push(out, 523);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1237);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 161);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 524);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1238);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2059);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2770);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1239);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 162);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 525);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1240);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2060);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2771);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3245);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 526);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 163);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 527);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1242);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2061);
									s.is_empty()
								}
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
			super::push(out, 164);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word εσταλαξα.
pub(crate) fn n3247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3246);
	s.is_empty()
}

/// Words ending -ξυνηλλαξα.
pub(crate) fn n3248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3247);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3481);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word εστεναξα.
pub(crate) fn n3249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3248);
	s.is_empty()
}

/// Words ending -συνηρπαξα.
pub(crate) fn n3250(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3249);
	s.is_empty()
}

/// Words ending -ραξα.
pub(crate) fn n172(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 172);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1253);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2777);
									s.is_empty()
								}
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
			super::push(out, 540);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 541);
			s.is_empty()
		}
		['κ', s @ ..] => {
			super::push(out, 542);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1254);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2073);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 543);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1255);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1256);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2074);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 545);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ταξα.
pub(crate) fn n174(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 174);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 547);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1258);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1259);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 1260);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2075);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2778);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1261);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2076);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2779);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3250);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3482);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
				['ε', s @ ..] => {
					super::push(out, 1262);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1263);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2078);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2780);
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υαξα.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1264);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2079);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φαξα.
pub(crate) fn n176(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 176);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1265);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2080);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 552);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1266);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2081);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -γξα.
pub(crate) fn n43(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 43);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 177);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 553);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1267);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2082);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1268);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 178);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 554);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1269);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 555);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1270);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 179);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 556);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1271);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 180);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 557);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1272);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εξα.
pub(crate) fn n44(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 44);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 181);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 558);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1273);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1274);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2084);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 559);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1275);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 560);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1276);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2085);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2781);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3252);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3485(s, out),
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
									super::push(out, 2086);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2782);
											s.is_empty()
										}
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
					super::push(out, 561);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1277);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 182);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 562);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 183);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 563);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1278);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2087);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2783);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3253);
													match s {
														[] => true,
														['π', s @ ..] => super::t0::n3486(s, out),
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
									super::push(out, 2088);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2784);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2089);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2785);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2090);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2786);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3254);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 564);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1279);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 565);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1280);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2091);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2092);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2787);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3255);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 566);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1281);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2093);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 567);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 184);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 568);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1282);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 185);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 569);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηξα.
pub(crate) fn n45(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 45);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 186);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 570);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1283);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 187);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 571);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1284);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 188);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 572);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1285);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2094);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 573);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1286);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2095);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2788);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2096);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2789);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2097);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2790);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3256);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 189);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 574);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1287);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 575);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1288);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 190);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 576);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1289);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 191);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 577);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1290);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1291);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2098);
									s.is_empty()
								}
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
			super::push(out, 192);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 578);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1292);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2099);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 579);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1293);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2100);
									match s {
										[] => true,
										['υ', s @ ..] => {
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
		['σ', s @ ..] => {
			super::push(out, 193);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 580);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1294);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2101);
									s.is_empty()
								}
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
			super::push(out, 194);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 581);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1295);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 582);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1296);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2102);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2792);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3257);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 195);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 583);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 196);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 584);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιξα.
pub(crate) fn n46(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 46);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 197);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 198);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 585);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1297);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 199);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 586);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1298);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2103);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2793);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2104);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2794);
											s.is_empty()
										}
										['ε', s @ ..] => {
											super::push(out, 2795);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3258);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 2796);
											s.is_empty()
										}
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
					super::push(out, 587);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1299);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 588);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1300);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2105);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 2797);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2106);
									s.is_empty()
								}
								['σ', s @ ..] => {
									super::push(out, 2107);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1301);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2108);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 589);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1302);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2109);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2798);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 200);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 590);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1303);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2110);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 591);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1304);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 201);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 592);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1305);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2111);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2799);
											s.is_empty()
										}
										_ => false,
									}
								}
								['η', s @ ..] => {
									super::push(out, 2112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 593);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1306);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2113);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2800);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 202);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 594);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1307);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1308);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2114);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2801);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3260);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3487(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 203);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 595);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1309);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 204);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 596);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1310);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 597);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1311);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2116);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2802);
											s.is_empty()
										}
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
					super::push(out, 598);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1312);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2117);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 599);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1313);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 205);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 600);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1314);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2118);
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
						['ε', s @ ..] => {
							super::push(out, 1315);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λξα.
pub(crate) fn n47(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 47);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 206);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 601);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1316);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οξα.
pub(crate) fn n48(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 48);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 207);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 602);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρξα.
pub(crate) fn n49(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 49);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 208);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 603);
					s.is_empty()
				}
				['δ', s @ ..] => {
					super::push(out, 604);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1317);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 605);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1318);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2119);
									s.is_empty()
								}
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
			super::push(out, 209);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 606);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1319);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2120);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 607);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1320);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 608);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1321);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1322);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 609);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1323);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2121);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2804);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3261);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 210);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 610);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1324);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1325);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2122);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 2805);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 211);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 611);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1326);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υξα.
pub(crate) fn n50(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 50);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 212);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 612);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1327);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 613);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1328);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 213);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 614);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1329);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2123);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2806);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 214);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 615);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1330);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 215);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 616);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 216);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 617);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1331);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2124);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 618);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1332);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 619);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1333);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2125);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 620);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1334);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 621);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1335);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2126);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 622);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 217);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 623);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1336);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2127);
									s.is_empty()
								}
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
			super::push(out, 218);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 624);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1337);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2128);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 625);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1338);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 626);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1339);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ψ', s @ ..] => {
			super::push(out, 219);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 627);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1340);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2129);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωξα.
pub(crate) fn n51(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 51);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 220);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 628);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1341);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 221);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 629);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1342);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2130);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2807);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2131);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 2808);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 222);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 630);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1343);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2132);
									s.is_empty()
								}
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
			super::push(out, 223);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 631);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1344);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2133);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 632);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1345);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 224);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 633);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1346);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2134);
									s.is_empty()
								}
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
			super::push(out, 225);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 634);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ασα.
pub(crate) fn n56(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 56);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 238);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 651);
					s.is_empty()
				}
				_ => false,
			}
		}
		['β', s @ ..] => {
			super::push(out, 239);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 652);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1357);
							match s {
								[] => true,
								['ε', 'σ', s @ ..] => {
									super::push(out, 2141);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
											super::push(out, 2810);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 2811);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3263);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t0::n3488(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 240);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 653);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1358);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2142);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2812);
											s.is_empty()
										}
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
					super::push(out, 654);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 655);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 241);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 656);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1359);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2143);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2813);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3264);
													s.is_empty()
												}
												['ι', s @ ..] => {
													super::push(out, 3265);
													match s {
														[] => true,
														['δ', s @ ..] => super::t0::n3489(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 657);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1360);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 242);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 658);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1361);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2145);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 659);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1362);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2146);
									s.is_empty()
								}
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
			super::push(out, 243);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 660);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1363);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2147);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 661);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1364);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 244);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 662);
					match s {
						[] => true,
						['ε', '\u{1}', s @ ..] => {
							super::push(out, 1365);
							s.is_empty()
						}
						_ => false,
					}
				}
				['δ', s @ ..] => {
					super::push(out, 663);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1366);
							match s {
								[] => true,
								['χ', s @ ..] => {
									super::push(out, 2148);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 2814);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1367);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2149);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 2815);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3266);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 664);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1368);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1369);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2150);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['κ', s @ ..] => {
					super::push(out, 665);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1370);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2151);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2816);
											s.is_empty()
										}
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
					super::push(out, 666);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1371);
							match s {
								[] => true,
								['α', 'γ', 'η', '\u{1}', s @ ..] => {
									super::push(out, 2152);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 667);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1372);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2153);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2817);
											s.is_empty()
										}
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
					super::push(out, 668);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 669);
					s.is_empty()
				}
				['π', s @ ..] => {
					super::push(out, 670);
					match s {
						[] => true,
						['ο', s @ ..] => super::t0::n1373(s, out),
						_ => false,
					}
				}
				['ρ', s @ ..] => super::t0::n671(s, out),
				['σ', s @ ..] => {
					super::push(out, 672);
					s.is_empty()
				}
				['τ', s @ ..] => super::t0::n673(s, out),
				['χ', s @ ..] => super::t0::n674(s, out),
				_ => false,
			}
		}
		['κ', s @ ..] => super::t0::n245(s, out),
		['λ', s @ ..] => super::t0::n246(s, out),
		['μ', s @ ..] => super::t0::n247(s, out),
		['ν', s @ ..] => super::t0::n248(s, out),
		['ξ', s @ ..] => super::t0::n249(s, out),
		['ο', s @ ..] => {
			super::push(out, 250);
			match s {
				[] => true,
				['θ', s @ ..] => super::t0::n694(s, out),
				_ => false,
			}
		}
		['π', s @ ..] => super::t0::n251(s, out),
		['ρ', s @ ..] => super::t0::n252(s, out),
		['σ', s @ ..] => super::t0::n253(s, out),
		['τ', s @ ..] => super::t0::n254(s, out),
		['υ', s @ ..] => super::t0::n255(s, out),
		['χ', s @ ..] => super::t0::n256(s, out),
		['ω', s @ ..] => super::t0::n257(s, out),
		_ => false,
	}
}

/// The word απετελεσα.
pub(crate) fn n3276(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3275);
	s.is_empty()
}

/// Words ending -εκορεσα.
pub(crate) fn n2214(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2214);
	s.is_empty()
}

/// Words ending -τορεσα.
pub(crate) fn n1446(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1446);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 2215);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2852);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τρεσα.
pub(crate) fn n738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 738);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1447);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2216);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τεσα.
pub(crate) fn n268(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 268);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 739);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1448);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χεσα.
pub(crate) fn n269(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 269);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 740);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1449);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 741);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ησα.
pub(crate) fn n58(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 58);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 270);
			s.is_empty()
		}
		['β', s @ ..] => {
			super::push(out, 271);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 742);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1450);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2217);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 2853);
											s.is_empty()
										}
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
					super::push(out, 743);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1451);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 744);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1452);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2218);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 2854);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3276);
													match s {
														[] => true,
														['ε', s @ ..] => super::t0::n3494(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 745);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1453);
							s.is_empty()
						}
						['φ', s @ ..] => {
							super::push(out, 1454);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 272);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 746);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 747);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1455);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1456);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2219);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 2855);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3277);
													match s {
														[] => true,
														['σ', s @ ..] => super::t0::n3495(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 748);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1457);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2220);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2856);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						['σ', s @ ..] => {
							super::push(out, 1458);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2221);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
				['λ', s @ ..] => {
					super::push(out, 749);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1459);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2222);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 750);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1460);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2223);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2224);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 2858);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3279);
													s.is_empty()
												}
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
							super::push(out, 1461);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2225);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
					super::push(out, 751);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1462);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2226);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1463);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2227);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 752);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1464);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2228);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 2860);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 273);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 753);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1465);
							s.is_empty()
						}
						['λ', s @ ..] => {
							super::push(out, 1466);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2229);
									s.is_empty()
								}
								_ => false,
							}
						}
						['μ', 'ο', '\u{1}', s @ ..] => {
							super::push(out, 1467);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => super::t0::n754(s, out),
				['η', s @ ..] => super::t0::n755(s, out),
				['ι', s @ ..] => super::t0::n756(s, out),
				['υ', s @ ..] => super::t0::n757(s, out),
				['ω', s @ ..] => {
					super::push(out, 758);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t0::n274(s, out),
		['ζ', s @ ..] => super::t0::n275(s, out),
		['η', s @ ..] => {
			super::push(out, 276);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 765);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => super::t0::n277(s, out),
		['ι', s @ ..] => super::t0::n278(s, out),
		['κ', s @ ..] => super::t0::n279(s, out),
		['λ', s @ ..] => super::t1::n280(s, out),
		['μ', s @ ..] => super::t1::n281(s, out),
		['ν', s @ ..] => super::t1::n282(s, out),
		['ξ', s @ ..] => super::t1::n283(s, out),
		['ο', s @ ..] => super::t1::n284(s, out),
		['π', s @ ..] => super::t1::n285(s, out),
		['ρ', s @ ..] => super::t1::n286(s, out),
		['σ', s @ ..] => super::t1::n287(s, out),
		['τ', s @ ..] => super::t1::n288(s, out),
		['υ', s @ ..] => super::t1::n289(s, out),
		['φ', s @ ..] => super::t1::n290(s, out),
		['χ', s @ ..] => super::t1::n291(s, out),
		['ψ', s @ ..] => super::t1::n292(s, out),
		_ => false,
	}
}

/// Words ending -ισα.
pub(crate) fn n59(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 59);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 293);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 875);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1663);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2427);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 876);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1664);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 877);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1665);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2428);
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
						['ρ', s @ ..] => {
							super::push(out, 1666);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2429);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3009);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3358);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 878);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1667);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2430);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 879);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1668);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2431);
									s.is_empty()
								}
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
			super::push(out, 294);
			s.is_empty()
		}
		['γ', s @ ..] => {
			super::push(out, 295);
			s.is_empty()
		}
		['δ', s @ ..] => {
			super::push(out, 296);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 880);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1669);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2432);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3010);
											s.is_empty()
										}
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
					super::push(out, 881);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1670);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 882);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1671);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2433);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3011);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3359);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3538(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1672);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2434);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3012);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 297);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 883);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1673);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2435);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2436);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3013);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3361);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3539(s, out),
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
									super::push(out, 2437);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3014);
											s.is_empty()
										}
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
					super::push(out, 884);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1674);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 885);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1675);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2438);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 886);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1676);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2439);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3015);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1677);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2440);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 887);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1678);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 888);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1679);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 298);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 889);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1680);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['θ', s @ ..] => {
			super::push(out, 299);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 890);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 891);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1681);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2442);
									s.is_empty()
								}
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
			super::push(out, 300);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 892);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 301);
			match s {
				[] => true,
				['α', s @ ..] => super::t1::n893(s, out),
				['β', s @ ..] => super::t1::n894(s, out),
				['η', s @ ..] => super::t1::n895(s, out),
				['υ', s @ ..] => super::t1::n896(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n302(s, out),
		['ν', s @ ..] => super::t1::n303(s, out),
		['ο', s @ ..] => super::t1::n304(s, out),
		['π', s @ ..] => super::t1::n305(s, out),
		['ρ', s @ ..] => super::t1::n306(s, out),
		['τ', s @ ..] => super::t1::n307(s, out),
		['φ', s @ ..] => super::t1::n308(s, out),
		['χ', s @ ..] => super::t1::n309(s, out),
		['ω', s @ ..] => super::t1::n310(s, out),
		_ => false,
	}
}

/// Words ending -λσα.
pub(crate) fn n60(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 60);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 311);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 924);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1715);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 312);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οσα.
pub(crate) fn n61(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 61);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 313);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 925);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1716);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ω', s @ ..] => {
					super::push(out, 926);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1717);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2468);
									match s {
										[] => true,
										['σ', s @ ..] => {
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
		['ρ', s @ ..] => {
			super::push(out, 314);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 927);
					s.is_empty()
				}
				['τ', s @ ..] => {
					super::push(out, 928);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1718);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2469);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3034);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3368);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρσα.
pub(crate) fn n62(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 62);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 315);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 316);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 929);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1719);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 930);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1720);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2470);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2471);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3035);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ξ', s @ ..] => {
									super::push(out, 2472);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 317);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 318);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 931);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1721);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 932);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1722);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 319);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 933);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1723);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σσα.
pub(crate) fn n63(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 63);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 320);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 934);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 321);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 935);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1724);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2473);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υσα.
pub(crate) fn n64(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 64);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 322);
			s.is_empty()
		}
		['α', s @ ..] => {
			super::push(out, 323);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 936);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1725);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2474);
									match s {
										[] => true,
										['α', s @ ..] => {
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
				['λ', s @ ..] => {
					super::push(out, 937);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1726);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2475);
									s.is_empty()
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 1727);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2476);
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
				['π', s @ ..] => {
					super::push(out, 938);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1728);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 939);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1729);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2477);
									s.is_empty()
								}
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
			super::push(out, 324);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 940);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1730);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2478);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3040);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3369);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 325);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 941);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1731);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2479);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3041);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3370);
													s.is_empty()
												}
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
							super::push(out, 1732);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2480);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1733);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2481);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1734);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2482);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3042);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3371);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => super::t1::n326(s, out),
		['η', s @ ..] => {
			super::push(out, 327);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 956);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 328);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 957);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 329);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 958);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1770);
							s.is_empty()
						}
						_ => false,
					}
				}
				['λ', s @ ..] => {
					super::push(out, 959);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1771);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2512);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3078);
											s.is_empty()
										}
										['θ', s @ ..] => {
											super::push(out, 3079);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3406);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3561(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['φ', s @ ..] => {
											super::push(out, 3080);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3407);
													s.is_empty()
												}
												['ε', s @ ..] => {
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
				['ω', s @ ..] => {
					super::push(out, 960);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1772);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2513);
									s.is_empty()
								}
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
			super::push(out, 330);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 961);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1773);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2514);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 962);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1774);
							s.is_empty()
						}
						['ν', s @ ..] => {
							super::push(out, 1775);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2515);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2516);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3081);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
							super::push(out, 1776);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 963);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n1777(s, out),
						_ => false,
					}
				}
				['κ', s @ ..] => super::t1::n964(s, out),
				['φ', s @ ..] => super::t1::n965(s, out),
				['ω', s @ ..] => super::t1::n966(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n331(s, out),
		['ν', s @ ..] => super::t1::n332(s, out),
		['ξ', s @ ..] => super::t1::n333(s, out),
		['ο', s @ ..] => super::t1::n334(s, out),
		['π', s @ ..] => super::t1::n335(s, out),
		['ρ', s @ ..] => super::t1::n336(s, out),
		['σ', s @ ..] => super::t1::n337(s, out),
		['τ', s @ ..] => super::t1::n338(s, out),
		['χ', s @ ..] => super::t1::n339(s, out),
		_ => false,
	}
}

/// Words ending -ωσα.
pub(crate) fn n65(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 65);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 340);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 988);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1801);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2542);
									s.is_empty()
								}
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
			super::push(out, 341);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 989);
					s.is_empty()
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 342);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 343);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 990);
					s.is_empty()
				}
				['θ', s @ ..] => {
					super::push(out, 991);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 992);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1802);
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
				['π', s @ ..] => {
					super::push(out, 993);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1803);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2544);
									s.is_empty()
								}
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
			super::push(out, 344);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 994);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1804);
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
				['ε', s @ ..] => {
					super::push(out, 995);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1805);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 345);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 996);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1806);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['ι', s @ ..] => {
			super::push(out, 346);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 997);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 347);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 998);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1807);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2547);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3100);
											s.is_empty()
										}
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
					super::push(out, 999);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1808);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2548);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3101);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 348);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1000);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1809);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2549);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3102);
											s.is_empty()
										}
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
					super::push(out, 1001);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1810);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2550);
									s.is_empty()
								}
								['π', s @ ..] => {
									super::push(out, 2551);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3103);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3418);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1002);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2552);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3104);
											s.is_empty()
										}
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
					super::push(out, 1003);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1812);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2553);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1004);
					s.is_empty()
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 349);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1005);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1813);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2554);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3105);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3419);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 350);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1006);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 1007);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1008);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1814);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2555);
									match s {
										[] => true,
										['ε', 'τ', s @ ..] => {
											super::push(out, 3106);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3420);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3564(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1009);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2556);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3107);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1816);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2557);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3108);
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
				['μ', s @ ..] => {
					super::push(out, 1010);
					match s {
						[] => true,
						['υ', s @ ..] => super::t1::n1817(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1011);
					s.is_empty()
				}
				['υ', s @ ..] => super::t1::n1012(s, out),
				_ => false,
			}
		}
		['ρ', s @ ..] => super::t1::n351(s, out),
		['σ', s @ ..] => super::t1::n352(s, out),
		['τ', s @ ..] => super::t1::n353(s, out),
		['φ', s @ ..] => super::t1::n354(s, out),
		['χ', s @ ..] => super::t1::n355(s, out),
		_ => false,
	}
}

/// Words ending -προεγραψα.
pub(crate) fn n3426(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3424);
	s.is_empty()
}

/// The word αναβλεψα.
pub(crate) fn n3427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3425);
	s.is_empty()
}

/// Words ending -ρεψα.
pub(crate) fn n370(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 370);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1046);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1846);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1047);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 1048);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1848);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2579);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3121);
									s.is_empty()
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2580);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3122);
									s.is_empty()
								}
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
			super::push(out, 1049);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1849);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2581);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2582);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3123);
									match s {
										[] => true,
										['μ', s @ ..] => {
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
		['ρ', s @ ..] => {
			super::push(out, 1050);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1850);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2583);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3124);
									s.is_empty()
								}
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
			super::push(out, 1051);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1851);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2584);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3125);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 2585);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3126);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2586);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3127);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3427);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 2587);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1852);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2588);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -στεψα.
pub(crate) fn n1052(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1052);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1853);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2589);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3128);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηψα.
pub(crate) fn n72(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 72);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 372);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1053);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 373);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1054);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1855);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 374);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1055);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1856);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1056);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1857);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 375);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1057);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 376);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1058);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1858);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιψα.
pub(crate) fn n73(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 73);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 377);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1059);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1859);
							s.is_empty()
						}
						['η', s @ ..] => {
							super::push(out, 1860);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2590);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1060);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1861);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1863);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2591);
									s.is_empty()
								}
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
			super::push(out, 378);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1864);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2592);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3129);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2593);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3130);
											s.is_empty()
										}
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
					super::push(out, 1064);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1865);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 379);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 380);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1066);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1867);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2594);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2595);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3131);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2596);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3132);
											s.is_empty()
										}
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
					super::push(out, 1067);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1868);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2597);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3133);
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
								['ν', s @ ..] => {
									super::push(out, 2598);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3134);
											match s {
												[] => true,
												['σ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -λψα.
pub(crate) fn n74(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 74);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 381);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1068);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1869);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 382);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1870);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 383);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1070);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1871);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2599);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μψα.
pub(crate) fn n75(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 75);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 384);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1872);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2600);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2601);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3135);
											s.is_empty()
										}
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
					super::push(out, 1072);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1873);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1874);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2602);
									s.is_empty()
								}
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
			super::push(out, 385);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1074);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1875);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2603);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1075);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1876);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2604);
									match s {
										[] => true,
										['ρ', 'π', '\u{1}', s @ ..] => {
											super::push(out, 3136);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οψα.
pub(crate) fn n76(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 76);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 386);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1076);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1877);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2605);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3137);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρψα.
pub(crate) fn n77(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 77);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 387);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1077);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1878);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 388);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1078);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 389);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υψα.
pub(crate) fn n78(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 78);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 390);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1880);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 391);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1080);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1881);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 392);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1081);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1882);
							s.is_empty()
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1883);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2606);
									s.is_empty()
								}
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
			super::push(out, 393);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1083);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1884);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 394);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1084);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1885);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1085);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1886);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2607);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωψα.
pub(crate) fn n79(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 79);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 395);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1086);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1887);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ανελαβον.
pub(crate) fn n3436(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3434);
	s.is_empty()
}

/// Words ending -πανελαβον.
pub(crate) fn n3437(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3435);
	s.is_empty()
}

/// Words ending -προελαβον.
pub(crate) fn n3438(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3436);
	s.is_empty()
}

/// Words ending -παρελαβον.
pub(crate) fn n3439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3437);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3564);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ροσελαβον.
pub(crate) fn n3440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3438);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3565);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κατελαβον.
pub(crate) fn n3441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3439);
	s.is_empty()
}

/// Words ending -μετελαβον.
pub(crate) fn n3442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3440);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3566);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -γαγον.
pub(crate) fn n1099(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1099);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1907);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2631);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3156);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2632);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3157);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3441);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3567);
											match s {
												[] => true,
												['α', s @ ..] => {
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
						['ε', s @ ..] => {
							super::push(out, 3158);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 2633);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 2634);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3160);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3442);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ε', s @ ..] => {
							super::push(out, 3161);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 3162);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3443);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 2635);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3163);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3444);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 2636);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 3164);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3445);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3568);
											s.is_empty()
										}
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
					super::push(out, 2637);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3165);
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

/// Words ending -τραγον.
pub(crate) fn n1908(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1908);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2638);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -φαγον.
pub(crate) fn n1101(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1101);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1909);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 2639);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιγον.
pub(crate) fn n402(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 402);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1102);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1910);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υγον.
pub(crate) fn n403(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 403);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1103);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1911);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2640);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1104);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1912);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2641);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3166);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 2642);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3167);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3447);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2643);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3168);
									match s {
										[] => true,
										['κ', s @ ..] => {
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

/// Words ending -δον.
pub(crate) fn n86(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 86);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 404);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1105);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1913);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2644);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1914);
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
		['ι', s @ ..] => {
			super::push(out, 405);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1107);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1915);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1916);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2646);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3169);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3449);
													s.is_empty()
												}
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
							super::push(out, 1917);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2647);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1918);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2648);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3170);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1919);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2649);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3171);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1920);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2650);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3172);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1921);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2651);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3173);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 406);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1108);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1922);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2652);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3174);
											s.is_empty()
										}
										['π', s @ ..] => {
											super::push(out, 3175);
											match s {
												[] => true,
												['α', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θον.
pub(crate) fn n87(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 87);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 407);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1109);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1923);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2653);
									match s {
										[] => true,
										['ε', '\u{1}', s @ ..] => {
											super::push(out, 3176);
											s.is_empty()
										}
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
					super::push(out, 1110);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1924);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2654);
									match s {
										[] => true,
										['τ', 'ε', 'μ', '\u{1}', s @ ..] => {
											super::push(out, 3177);
											s.is_empty()
										}
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
					super::push(out, 1111);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1925);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1112);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1926);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1113);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1927);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2655);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3178);
											match s {
												[] => true,
												['α', s @ ..] => {
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
				['ρ', s @ ..] => {
					super::push(out, 1114);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1928);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2656);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3179);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 408);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1115);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1929);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2657);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3180);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3452);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 409);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1116);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1930);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2658);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3181);
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
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1117);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1931);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1932);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2659);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3182);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1933);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2660);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2661);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3183);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 1934);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2662);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3184);
											match s {
												[] => true,
												['υ', s @ ..] => {
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
						['ο', s @ ..] => {
							super::push(out, 1935);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2663);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3185);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1936);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2664);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3186);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 2665);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3187);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1937);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2666);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3188);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3455);
													s.is_empty()
												}
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
							super::push(out, 1938);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2667);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3189);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
						['τ', s @ ..] => {
							super::push(out, 1939);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2668);
									match s {
										[] => true,
										['μ', s @ ..] => {
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
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n410(s, out),
		_ => false,
	}
}

/// Words ending -ιον.
pub(crate) fn n88(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 88);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 411);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1119);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1120);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κον.
pub(crate) fn n89(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 89);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 412);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1941);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 413);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1942);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2671);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 3193);
											match s {
												[] => true,
												['ε', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 414);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1943);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 415);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1944);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λον.
pub(crate) fn n90(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 90);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 416);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1945);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2672);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3194);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3460);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2673);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3195);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3461);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['σ', s @ ..] => {
									super::push(out, 2674);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3196);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3462);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3572(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3197);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3463);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3573(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 417);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1946);
							s.is_empty()
						}
						['θ', s @ ..] => {
							super::push(out, 1947);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2675);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3198);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 1948);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2676);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 1949);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2677);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2678);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3199);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ξ', s @ ..] => {
							super::push(out, 1950);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2679);
									s.is_empty()
								}
								_ => false,
							}
						}
						['φ', s @ ..] => {
							super::push(out, 1951);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2680);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2681);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μον.
pub(crate) fn n91(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 91);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 418);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1952);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1953);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2682);
									s.is_empty()
								}
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
			super::push(out, 419);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1954);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2683);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3200);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3464);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3574(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
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
pub(crate) fn n92(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 92);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 420);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1955);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2684);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3201);
											s.is_empty()
										}
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
					super::push(out, 1131);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1956);
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
				['χ', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1957);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2686);
									s.is_empty()
								}
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
			super::push(out, 421);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1133);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1958);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2687);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξον.
pub(crate) fn n93(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 93);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 422);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1134);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1959);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2688);
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
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 423);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1135);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 1960);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2689);
									match s {
										[] => true,
										['ε', s @ ..] => {
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

/// Words ending -πον.
pub(crate) fn n94(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 94);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 424);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1136);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 1961);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2690);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1962);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2691);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3204);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1963);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2692);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3205);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 2693);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2694);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1964);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2695);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3206);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 1965);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2696);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3207);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3465);
													s.is_empty()
												}
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
							super::push(out, 1966);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2697);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3208);
											s.is_empty()
										}
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
					super::push(out, 1137);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1967);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2698);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3209);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 1968);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 425);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1138);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1969);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2699);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1139);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1970);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2700);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρον.
pub(crate) fn n95(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 95);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 426);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1140);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1971);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2701);
									s.is_empty()
								}
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
			super::push(out, 427);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1141);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1972);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2702);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				['π', s @ ..] => {
					super::push(out, 1142);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1973);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 428);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1143);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1974);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2703);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1144);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 1975);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2704);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σον.
pub(crate) fn n96(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 96);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 429);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1145);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1976);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 430);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1146);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1977);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τον.
pub(crate) fn n97(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 97);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 431);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1147);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1978);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2705);
									s.is_empty()
								}
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
			super::push(out, 432);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1148);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1979);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2706);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3211);
											match s {
												[] => true,
												['ν', s @ ..] => {
													super::push(out, 3466);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3575(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
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
pub(crate) fn n98(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 98);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 433);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1149);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1980);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φον.
pub(crate) fn n99(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 99);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 434);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1981);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 435);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χον.
pub(crate) fn n100(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 100);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 436);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1982);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2707);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3212);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3467);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 437);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1153);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1983);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2708);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3213);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3468);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3576(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 438);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1154);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1984);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 1985);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2709);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3214);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3469);
													s.is_empty()
												}
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
							super::push(out, 1986);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2710);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3215);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 1987);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2711);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3216);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['π', s @ ..] => {
							super::push(out, 1988);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2712);
									s.is_empty()
								}
								['ε', s @ ..] => {
									super::push(out, 2713);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 1989);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2714);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3217);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3470);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 2715);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3218);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3471);
													s.is_empty()
												}
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
							super::push(out, 1990);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2716);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3219);
											match s {
												[] => true,
												['π', s @ ..] => {
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
						['τ', s @ ..] => {
							super::push(out, 1991);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2717);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3220);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ε', s @ ..] => {
									super::push(out, 2718);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2719);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1992);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2720);
									match s {
										[] => true,
										['π', s @ ..] => {
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
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 439);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1156);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1993);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2721);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 2722);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3222);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3473);
													s.is_empty()
												}
												_ => false,
											}
										}
										['υ', s @ ..] => {
											super::push(out, 3223);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3474);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3577(s, out),
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
									super::push(out, 2723);
									s.is_empty()
								}
								['ρ', s @ ..] => super::t1::n2724(s, out),
								['σ', s @ ..] => {
									super::push(out, 2725);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οκατεγνων.
pub(crate) fn n3478(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3476);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 3577);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3616);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κατεκτεινα.
pub(crate) fn n3482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3480);
	s.is_empty()
}

/// Words ending -περιεπλεξα.
pub(crate) fn n3485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3483);
	s.is_empty()
}

/// Words ending -περιεθρεξα.
pub(crate) fn n3486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3484);
	s.is_empty()
}

/// Words ending -εγκατεμιξα.
pub(crate) fn n3487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3485);
	s.is_empty()
}

/// The word εισεβιβασα.
pub(crate) fn n3488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3486);
	s.is_empty()
}

/// Words ending -διεσκεδασα.
pub(crate) fn n3489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3487);
	s.is_empty()
}

/// Words ending -οπιασα.
pub(crate) fn n1373(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1373);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2154);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2818);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 3267);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ριασα.
pub(crate) fn n671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 671);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1374);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2155);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2819);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3268);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3488);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3579);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3619(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1375);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2156);
					s.is_empty()
				}
				['ε', s @ ..] => {
					super::push(out, 2157);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2820);
							s.is_empty()
						}
						['μ', s @ ..] => {
							super::push(out, 2821);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τιασα.
pub(crate) fn n673(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 673);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1376);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2158);
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

/// Words ending -χιασα.
pub(crate) fn n674(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 674);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1377);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 1378);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κασα.
pub(crate) fn n245(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 245);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 675);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2159);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2160);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λασα.
pub(crate) fn n246(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 246);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1382);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2161);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2162);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2163);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2164);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2823);
									s.is_empty()
								}
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
			super::push(out, 679);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1386);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1387);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2165);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['θ', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1388);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1389);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2166);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2824);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2167);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2825);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
		['ο', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1390);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2168);
							s.is_empty()
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1391);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2169);
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
		['π', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1392);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2170);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μασα.
pub(crate) fn n247(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 247);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1393);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2171);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2827);
									s.is_empty()
								}
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
			super::push(out, 685);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1394);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2172);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2828);
									s.is_empty()
								}
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
			super::push(out, 686);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1395);
					s.is_empty()
				}
				['ο', s @ ..] => {
					super::push(out, 1396);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2173);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1397);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2174);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 687);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1398);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2175);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 688);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1399);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2176);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 689);
			s.is_empty()
		}
		['χ', s @ ..] => {
			super::push(out, 690);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -νασα.
pub(crate) fn n248(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 248);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 691);
			s.is_empty()
		}
		['ο', s @ ..] => {
			super::push(out, 692);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ξασα.
pub(crate) fn n249(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 249);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 693);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1400);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2177);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2829);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θοασα.
pub(crate) fn n694(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 694);
	s.is_empty()
}

/// Words ending -πασα.
pub(crate) fn n251(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 251);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 695);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1401);
					s.is_empty()
				}
				['κ', s @ ..] => {
					super::push(out, 1402);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2178);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2830);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2179);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1404);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2180);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['ρ', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1405);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2181);
							match s {
								[] => true,
								['δ', s @ ..] => {
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
		['σ', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1406);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2182);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2833);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3270);
											match s {
												[] => true,
												['π', s @ ..] => {
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
						['ν', s @ ..] => {
							super::push(out, 2183);
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
		['ω', s @ ..] => {
			super::push(out, 699);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρασα.
pub(crate) fn n252(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 252);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 700);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1407);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2184);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 701);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1408);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1409);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2185);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 703);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1410);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2186);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2835);
									s.is_empty()
								}
								['τ', s @ ..] => {
									super::push(out, 2836);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3271);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
						_ => false,
					}
				}
				['θ', s @ ..] => {
					super::push(out, 1411);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2187);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1412);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2188);
							s.is_empty()
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1413);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2189);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1414);
					match s {
						[] => true,
						['π', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2190);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 705);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1415);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2191);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1416);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2192);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1417);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2193);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2837);
									s.is_empty()
								}
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
			super::push(out, 708);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1418);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2194);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2838);
									s.is_empty()
								}
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
			super::push(out, 709);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1419);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ω', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1420);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2195);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2839);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3272);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σασα.
pub(crate) fn n253(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 253);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1421);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2196);
							match s {
								[] => true,
								['κ', s @ ..] => {
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

/// Words ending -τασα.
pub(crate) fn n254(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 254);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 712);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1422);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2197);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2841);
									s.is_empty()
								}
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
			super::push(out, 713);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1423);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2198);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2842);
									s.is_empty()
								}
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
			super::push(out, 714);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 715);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1424);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2199);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2843);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
		['ρ', s @ ..] => {
			super::push(out, 716);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 717);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1425);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2200);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2844);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υασα.
pub(crate) fn n255(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 255);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 718);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1426);
					match s {
						[] => true,
						['σ', 'ε', 'π', s @ ..] => {
							super::push(out, 2201);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2845);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3274);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χασα.
pub(crate) fn n256(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 256);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 719);
			s.is_empty()
		}
		['σ', s @ ..] => {
			super::push(out, 720);
			s.is_empty()
		}
		['υ', s @ ..] => {
			super::push(out, 721);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1427);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2202);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2846);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωασα.
pub(crate) fn n257(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 257);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 722);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εκολυμβησα.
pub(crate) fn n3494(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3492);
	s.is_empty()
}

/// Words ending -στρατηγησα.
pub(crate) fn n3495(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3493);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3580);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εδησα.
pub(crate) fn n754(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 754);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1468);
			s.is_empty()
		}
		['π', s @ ..] => {
			super::push(out, 1469);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2230);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2861);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ηδησα.
pub(crate) fn n755(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 755);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1470);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2231);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1471);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2232);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
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

/// Words ending -ιδησα.
pub(crate) fn n756(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 756);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1472);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2233);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2863);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3281);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υδησα.
pub(crate) fn n757(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 757);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1473);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2234);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2864);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3282);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3495);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1474);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2235);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2865);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3283);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3496);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εησα.
pub(crate) fn n274(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 274);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 759);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1475);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2236);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2866);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3284);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2237);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2867);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 2238);
							match s {
								[] => true,
								['α', s @ ..] => {
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
		['λ', s @ ..] => {
			super::push(out, 760);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1476);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ζησα.
pub(crate) fn n275(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 275);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 761);
			match s {
				[] => true,
				['β', s @ ..] => {
					super::push(out, 1477);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2239);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2869);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 762);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1478);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2240);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2870);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1479);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2241);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 763);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1480);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2242);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['ω', s @ ..] => {
			super::push(out, 764);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1481);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -θησα.
pub(crate) fn n277(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 277);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 766);
			s.is_empty()
		}
		['η', s @ ..] => {
			super::push(out, 767);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1482);
					s.is_empty()
				}
				['γ', s @ ..] => {
					super::push(out, 1483);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2243);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2872);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1484);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2244);
							match s {
								[] => true,
								['ε', s @ ..] => {
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
		['ι', s @ ..] => {
			super::push(out, 768);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1485);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2245);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2874);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3286);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 769);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1486);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2246);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2875);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3287);
											s.is_empty()
										}
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
					super::push(out, 1487);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 770);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1488);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2247);
							s.is_empty()
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1489);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2248);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 771);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1490);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2249);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['ρ', s @ ..] => {
			super::push(out, 772);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1491);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 773);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1492);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2250);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2877);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3288);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 774);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1493);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2251);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 775);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1494);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2252);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2878);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιησα.
pub(crate) fn n278(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 278);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 776);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1495);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2253);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2879);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3289);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 777);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1496);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2254);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 2255);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 2880);
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 3290);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3497);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3583(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 778);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 1497);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2256);
							match s {
								[] => true,
								['ε', s @ ..] => {
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
		['τ', s @ ..] => {
			super::push(out, 779);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1498);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2257);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2882);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3291);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κησα.
pub(crate) fn n279(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 279);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 780);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1499);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2258);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2883);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2259);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2884);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3292);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['υ', s @ ..] => {
							super::push(out, 2260);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2885);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3293);
											s.is_empty()
										}
										['ο', s @ ..] => {
											super::push(out, 3294);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3498);
													s.is_empty()
												}
												['χ', s @ ..] => {
													super::push(out, 3499);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 781);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1500);
					match s {
						[] => true,
						['ε', '\u{1}', s @ ..] => {
							super::push(out, 2261);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 782);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1501);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2262);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1502);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2263);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2886);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2264);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2887);
									match s {
										[] => true,
										['π', s @ ..] => {
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
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 783);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1503);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2265);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2888);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3296);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3500);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3584(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 784);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1504);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2266);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2889);
									match s {
										[] => true,
										['ο', 'π', 'ε', s @ ..] => {
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3501);
													s.is_empty()
												}
												['ξ', s @ ..] => {
													super::push(out, 3502);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3585(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 785);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1505);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2267);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2890);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3297);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 786);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1506);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2268);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['ω', s @ ..] => {
			super::push(out, 787);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1507);
					match s {
						[] => true,
						['α', s @ ..] => {
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

