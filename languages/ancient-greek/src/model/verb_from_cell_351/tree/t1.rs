use flexion::Path;

/// Words ending -ρηκα.
pub(crate) fn n136(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 136);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 464);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1060);
					s.is_empty()
				}
				['υ', s @ ..] => {
					super::push(out, 1061);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1770);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2459);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3066);
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
				['χ', s @ ..] => {
					super::push(out, 1062);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1771);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['γ', s @ ..] => {
			super::push(out, 465);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1063);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 1772);
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
		['ε', s @ ..] => {
			super::push(out, 466);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1064);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1773);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2461);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1065);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1774);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2462);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3067);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3513);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3794(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1775);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2463);
									s.is_empty()
								}
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
			super::push(out, 467);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1066);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1776);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2464);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3068);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3514);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3795(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1067);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1777);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 1068);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1778);
							s.is_empty()
						}
						['υ', s @ ..] => {
							super::push(out, 1779);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2465);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ξ', s @ ..] => {
					super::push(out, 1069);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1780);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1070);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1781);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2466);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1071);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1782);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 468);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1072);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1783);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2467);
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
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1073);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1784);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 1785);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1786);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2468);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3070);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3515);
													s.is_empty()
												}
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
							super::push(out, 1787);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2469);
									s.is_empty()
								}
								_ => false,
							}
						}
						['χ', s @ ..] => {
							super::push(out, 1788);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2470);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3071);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3516);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3796(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['π', s @ ..] => {
											super::push(out, 3072);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3517);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => super::t1::n469(s, out),
		['π', s @ ..] => {
			super::push(out, 470);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1079);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1797);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 471);
			match s {
				[] => true,
				['α', 'θ', 'ε', 'τ', '\u{1}', s @ ..] => {
					super::push(out, 1080);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1081);
					s.is_empty()
				}
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 472);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1082);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1798);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2480);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3081);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => super::t1::n473(s, out),
		['ω', s @ ..] => super::t1::n474(s, out),
		_ => false,
	}
}

/// Words ending -σηκα.
pub(crate) fn n137(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 137);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 475);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1092);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1811);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2491);
									s.is_empty()
								}
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
			super::push(out, 476);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1093);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1812);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2492);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1094);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1813);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 477);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1095);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 1814);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2493);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3090);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 478);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1096);
					match s {
						[] => true,
						['ε', 'π', 'μ', s @ ..] => {
							super::push(out, 1815);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2494);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τηκα.
pub(crate) fn n138(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 138);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 479);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1097);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1816);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2495);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3091);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3531);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3806(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1817);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2496);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1098);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1818);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2497);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3092);
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
								['ο', s @ ..] => {
									super::push(out, 2498);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3093);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 3533);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3094);
											match s {
												[] => true,
												['υ', s @ ..] => {
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
				['τ', s @ ..] => {
					super::push(out, 1099);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1819);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2499);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3095);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3535);
													match s {
														[] => true,
														['λ', s @ ..] => super::t1::n3807(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 480);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1100);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1820);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1821);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2500);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3096);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3536);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3808(s, out),
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
									super::push(out, 2501);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3097);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 2502);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3098);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3537);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3809(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1101);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1822);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2503);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3099);
											match s {
												[] => true,
												['μ', s @ ..] => {
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
				['ρ', s @ ..] => {
					super::push(out, 1102);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1823);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2504);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3100);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', 'ε', 'κ', '\u{1}', s @ ..] => {
							super::push(out, 1824);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1103);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1825);
							s.is_empty()
						}
						['ο', s @ ..] => {
							super::push(out, 1826);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2505);
									match s {
										[] => true,
										['α', s @ ..] => {
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
		['η', s @ ..] => {
			super::push(out, 481);
			match s {
				[] => true,
				['ζ', s @ ..] => {
					super::push(out, 1104);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1827);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2506);
									s.is_empty()
								}
								['ν', s @ ..] => super::t1::n2521(s, out),
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => super::t1::n1106(s, out),
				['ξ', s @ ..] => {
					super::push(out, 1106);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n1834(s, out),
						_ => false,
					}
				}
				['π', s @ ..] => super::t1::n1108(s, out),
				_ => false,
			}
		}
		['ι', s @ ..] => super::t1::n482(s, out),
		['κ', 'α', 'ν', 'α', 'γ', 'η', '\u{1}', s @ ..] => super::t1::n483(s, out),
		['ν', s @ ..] => super::t1::n484(s, out),
		['ο', s @ ..] => super::t1::n485(s, out),
		['π', s @ ..] => super::t1::n486(s, out),
		['ρ', s @ ..] => super::t1::n487(s, out),
		['σ', s @ ..] => super::t1::n488(s, out),
		['υ', s @ ..] => super::t1::n489(s, out),
		['ω', s @ ..] => super::t1::n490(s, out),
		_ => false,
	}
}

/// Words ending -υηκα.
pub(crate) fn n139(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 139);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 491);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1121);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1848);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 492);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1122);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 1849);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 493);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1123);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1850);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2527);
									match s {
										[] => true,
										['υ', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φηκα.
pub(crate) fn n140(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 140);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 494);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1124);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1851);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2528);
									s.is_empty()
								}
								['ω', s @ ..] => {
									super::push(out, 2529);
									match s {
										[] => true,
										['ζ', s @ ..] => {
											super::push(out, 3120);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3549);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3820(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 495);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1125);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1852);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2530);
									s.is_empty()
								}
								_ => false,
							}
						}
						['τ', s @ ..] => {
							super::push(out, 1853);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2531);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1126);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 1854);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χηκα.
pub(crate) fn n141(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 141);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 496);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1127);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1855);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2532);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3121);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3550);
													s.is_empty()
												}
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
							super::push(out, 1856);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2533);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3122);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 2534);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3123);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 497);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1128);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1857);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2535);
									s.is_empty()
								}
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
			super::push(out, 498);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1129);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1858);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2536);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3124);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3551);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3821(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 499);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1130);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1859);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2537);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3125);
											match s {
												[] => true,
												['ο', s @ ..] => {
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
						['δ', s @ ..] => {
							super::push(out, 1860);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2538);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3126);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3553);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3822(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1861);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2539);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3127);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 500);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1131);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1862);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 1863);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 501);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1132);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1864);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2540);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3128);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3554);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3823(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1133);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1865);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2541);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3129);
											s.is_empty()
										}
										['α', s @ ..] => {
											super::push(out, 3130);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 3131);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							super::push(out, 1866);
							s.is_empty()
						}
						['σ', s @ ..] => {
							super::push(out, 1867);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2542);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3132);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3555);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3824(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1868);
							match s {
								[] => true,
								['η', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -ψηκα.
pub(crate) fn n142(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 142);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 502);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1134);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 503);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 504);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1135);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιαλελογικα.
pub(crate) fn n3575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3556);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3806);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υπλελογικα.
pub(crate) fn n3576(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3557);
	s.is_empty()
}

/// Words ending -ελαμπαδικα.
pub(crate) fn n3577(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3558);
	s.is_empty()
}

/// Words ending -νδραποδικα.
pub(crate) fn n3578(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3559);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 3807);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εμπεποδικα.
pub(crate) fn n3579(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3560);
	s.is_empty()
}

/// Words ending -εικα.
pub(crate) fn n147(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 147);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 517);
			s.is_empty()
		}
		['ε', s @ ..] => {
			super::push(out, 518);
			s.is_empty()
		}
		['θ', s @ ..] => {
			super::push(out, 519);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1148);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1879);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1149);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1880);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2551);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3140);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3561);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3827(s, out),
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
									super::push(out, 2552);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3141);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3562);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['κ', s @ ..] => {
									super::push(out, 2553);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3142);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
													super::push(out, 3563);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2554);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3143);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 520);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1150);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1881);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2555);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3144);
											s.is_empty()
										}
										['ι', s @ ..] => {
											super::push(out, 3145);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3564);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3828(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3146);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3565);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3829(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 521);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1151);
					s.is_empty()
				}
				['η', s @ ..] => {
					super::push(out, 1152);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1882);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2556);
									match s {
										[] => true,
										['ξ', s @ ..] => {
											super::push(out, 3147);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3566);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1153);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1883);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 522);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1154);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1884);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2557);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 2558);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3148);
											match s {
												[] => true,
												['α', s @ ..] => {
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
				['σ', s @ ..] => {
					super::push(out, 1155);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1885);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 523);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1156);
					s.is_empty()
				}
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 524);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1157);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1886);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 525);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1158);
					s.is_empty()
				}
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

/// Words ending -εθικα.
pub(crate) fn n527(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 527);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1161);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1888);
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
		_ => false,
	}
}

/// Words ending -ειθικα.
pub(crate) fn n1163(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1162);
	s.is_empty()
}

/// Words ending -νθικα.
pub(crate) fn n529(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 529);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1163);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υθικα.
pub(crate) fn n530(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 530);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1164);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1889);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2561);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3149);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3568);
											match s {
												[] => true,
												['σ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -κικα.
pub(crate) fn n149(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 149);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 531);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1165);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1890);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2562);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				['ν', s @ ..] => {
					super::push(out, 1166);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1891);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2563);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3151);
											s.is_empty()
										}
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
							super::push(out, 1892);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2564);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3152);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ω', s @ ..] => {
							super::push(out, 1893);
							match s {
								[] => true,
								['θ', s @ ..] => {
									super::push(out, 2565);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3153);
											match s {
												[] => true,
												['τ', s @ ..] => {
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
		['η', s @ ..] => {
			super::push(out, 532);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1167);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1894);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2566);
									s.is_empty()
								}
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
			super::push(out, 533);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1168);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1895);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2567);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['χ', s @ ..] => {
					super::push(out, 1169);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 1896);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2568);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3154);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3570);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n3831(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 534);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1170);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 535);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1171);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1897);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2569);
									s.is_empty()
								}
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
			super::push(out, 536);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1172);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λικα.
pub(crate) fn n150(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 150);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 537);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1173);
					s.is_empty()
				}
				['φ', s @ ..] => {
					super::push(out, 1174);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1898);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2570);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3155);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3571);
													match s {
														[] => true,
														['ο', s @ ..] => super::t1::n3832(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1899);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2571);
									s.is_empty()
								}
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
			super::push(out, 538);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1175);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1900);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2572);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3156);
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
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 539);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1176);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1901);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2573);
									s.is_empty()
								}
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
			super::push(out, 540);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1177);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1902);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2574);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3157);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3573);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3833(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 541);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1178);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 1903);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2575);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3158);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 542);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1179);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1904);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2576);
									s.is_empty()
								}
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
			super::push(out, 543);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1180);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μικα.
pub(crate) fn n151(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 151);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 544);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1181);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1905);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2577);
									s.is_empty()
								}
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
			super::push(out, 545);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1182);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1906);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2578);
									s.is_empty()
								}
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
			super::push(out, 546);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1183);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1907);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2579);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3159);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3574);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3834(s, out),
														_ => false,
													}
												}
												['τ', s @ ..] => {
													super::push(out, 3575);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3835(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 3160);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3576);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3836(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1184);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1908);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2580);
									s.is_empty()
								}
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
			super::push(out, 547);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1185);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νικα.
pub(crate) fn n152(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 152);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 548);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1186);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1909);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2581);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3161);
											s.is_empty()
										}
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
					super::push(out, 1187);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 1910);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2582);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1188);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1911);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2583);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3162);
											s.is_empty()
										}
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
					super::push(out, 1189);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1912);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2584);
									match s {
										[] => true,
										['μ', s @ ..] => {
											super::push(out, 3163);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3577);
													s.is_empty()
												}
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
							super::push(out, 1913);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 2585);
									s.is_empty()
								}
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
			super::push(out, 549);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1190);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1914);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 550);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1191);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1915);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1192);
					s.is_empty()
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 551);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1193);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1916);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2586);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3164);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 552);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1194);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1917);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 553);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1195);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1918);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2587);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3165);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3578);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1196);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1919);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2588);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3166);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3579);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1197);
					match s {
						[] => true,
						['χ', 'ε', s @ ..] => {
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2589);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3167);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ν', s @ ..] => {
									super::push(out, 2590);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3168);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3580);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1198);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1920);
							match s {
								[] => true,
								['η', s @ ..] => {
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
		['π', s @ ..] => {
			super::push(out, 554);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1199);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1921);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2592);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3169);
											s.is_empty()
										}
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
					super::push(out, 1200);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 1922);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2593);
									s.is_empty()
								}
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
			super::push(out, 555);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1201);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1923);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2594);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3170);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οικα.
pub(crate) fn n153(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 153);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 556);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1202);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 1924);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2595);
									s.is_empty()
								}
								['ι', s @ ..] => {
									super::push(out, 2596);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3171);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3581);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 2597);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3172);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3582);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 557);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1203);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1925);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πικα.
pub(crate) fn n154(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 154);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 558);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1204);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1926);
							match s {
								[] => true,
								['ῥ', s @ ..] => {
									super::push(out, 2598);
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 559);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1205);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1927);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2599);
									match s {
										[] => true,
										['η', s @ ..] => {
											super::push(out, 3174);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 560);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1206);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1928);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 1929);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2600);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3175);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 561);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1207);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1930);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2601);
									s.is_empty()
								}
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
			super::push(out, 562);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1208);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1931);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2602);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		['σ', s @ ..] => {
			super::push(out, 563);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1209);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1932);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρικα.
pub(crate) fn n155(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 155);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 564);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1210);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1933);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2603);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3177);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3583);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1211);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1934);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2604);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3178);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3584);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 565);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1212);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 1935);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2605);
									s.is_empty()
								}
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
			super::push(out, 566);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1213);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1936);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2606);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1214);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1937);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2607);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3179);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3585);
													s.is_empty()
												}
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
							super::push(out, 1938);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2608);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3180);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3586);
													s.is_empty()
												}
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
							super::push(out, 1939);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2609);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3181);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3587);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 567);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1215);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1940);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2610);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3182);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3588);
													match s {
														[] => true,
														['χ', s @ ..] => super::t1::n3837(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1941);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2611);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3183);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3589);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 568);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1216);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 1942);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2612);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3184);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3590);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n3838(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 569);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1217);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1943);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2613);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3185);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3591);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ν', s @ ..] => {
											super::push(out, 3186);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3592);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3187);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3593);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3839(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 570);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1218);
					match s {
						[] => true,
						['ε', s @ ..] => super::t1::n1950(s, out),
						_ => false,
					}
				}
				_ => false,
			}
		}
		['π', s @ ..] => super::t1::n571(s, out),
		['υ', s @ ..] => super::t1::n572(s, out),
		['φ', s @ ..] => super::t1::n573(s, out),
		['χ', s @ ..] => super::t1::n574(s, out),
		['ω', s @ ..] => super::t1::n575(s, out),
		_ => false,
	}
}

/// Words ending -τικα.
pub(crate) fn n156(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 156);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 576);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1231);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1955);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2626);
									s.is_empty()
								}
								_ => false,
							}
						}
						['η', s @ ..] => {
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2627);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3193);
											s.is_empty()
										}
										_ => false,
									}
								}
								['χ', s @ ..] => {
									super::push(out, 2628);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3194);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3598);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 577);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1232);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1956);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2629);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3195);
											s.is_empty()
										}
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
					super::push(out, 1233);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 1957);
							s.is_empty()
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 1234);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1958);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 578);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1235);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 1959);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2630);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ε', s @ ..] => {
					super::push(out, 1236);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 1960);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 579);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1237);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1961);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2631);
									match s {
										[] => true,
										['ῥ', s @ ..] => {
											super::push(out, 3196);
											s.is_empty()
										}
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
					super::push(out, 1238);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 1962);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 2632);
									s.is_empty()
								}
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
			super::push(out, 580);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1239);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1963);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2633);
									s.is_empty()
								}
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
			super::push(out, 581);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1240);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 1964);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2634);
									s.is_empty()
								}
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
			super::push(out, 582);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1241);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1965);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2635);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3197);
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
		_ => false,
	}
}

/// Words ending -φικα.
pub(crate) fn n157(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 157);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 583);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 1242);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1966);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 584);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1243);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1967);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2636);
									s.is_empty()
								}
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
			super::push(out, 585);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1244);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 1968);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2637);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3198);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3600);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3841(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χικα.
pub(crate) fn n158(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 158);
	match s {
		[] => true,
		['η', 'τ', 'α', 'κ', 'ε', '\u{1}', s @ ..] => {
			super::push(out, 586);
			s.is_empty()
		}
		['ι', s @ ..] => {
			super::push(out, 587);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1245);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 1969);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2638);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3199);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3601);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3842(s, out),
														_ => false,
													}
												}
												['ο', s @ ..] => {
													super::push(out, 3602);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3843(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωικα.
pub(crate) fn n159(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 159);
	match s {
		[] => true,
		['ζ', s @ ..] => {
			super::push(out, 588);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1246);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 1970);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2639);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3200);
											match s {
												[] => true,
												['γ', s @ ..] => {
													super::push(out, 3603);
													match s {
														[] => true,
														['η', s @ ..] => super::t1::n3844(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 589);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1247);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1971);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2640);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ανατεταλκα.
pub(crate) fn n3623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3604);
	s.is_empty()
}

/// Words ending -κλελυτροκα.
pub(crate) fn n3624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3605);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3826);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ανακεκαυκα.
pub(crate) fn n3625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3606);
	s.is_empty()
}

/// Words ending -απολελαυκα.
pub(crate) fn n3626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3607);
	s.is_empty()
}

/// Words ending -ευκα.
pub(crate) fn n175(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 175);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 615);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1275);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 1993);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2655);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1276);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 1994);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['δ', s @ ..] => {
			super::push(out, 616);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1277);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 1995);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2656);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 3208);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3608);
													s.is_empty()
												}
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
							super::push(out, 1996);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2657);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3209);
											match s {
												[] => true,
												['λ', s @ ..] => {
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
				['ε', s @ ..] => {
					super::push(out, 1278);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 1997);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2658);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['η', s @ ..] => {
					super::push(out, 1279);
					s.is_empty()
				}
				['ι', s @ ..] => {
					super::push(out, 1280);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 1998);
							match s {
								[] => true,
								['π', 'ε', 'π', s @ ..] => {
									match s {
										[] => true,
										['\u{1}', s @ ..] => {
											super::push(out, 3210);
											s.is_empty()
										}
										['κ', s @ ..] => {
											super::push(out, 3211);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3610);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3212);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3611);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3846(s, out),
														_ => false,
													}
												}
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
							super::push(out, 1999);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2659);
									s.is_empty()
								}
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
			super::push(out, 617);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1281);
					s.is_empty()
				}
				['μ', s @ ..] => {
					super::push(out, 1282);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2000);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2660);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3213);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3612);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1283);
					s.is_empty()
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 618);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1284);
					s.is_empty()
				}
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 619);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1285);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2001);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2661);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3214);
											s.is_empty()
										}
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
					super::push(out, 1286);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2002);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2662);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1287);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2003);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2663);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2004);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2664);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3215);
											match s {
												[] => true,
												['β', s @ ..] => super::t1::n3632(s, out),
												_ => false,
											}
										}
										_ => false,
									}
								}
								['δ', s @ ..] => {
									super::push(out, 2665);
									match s {
										[] => true,
										['ε', s @ ..] => super::t1::n3234(s, out),
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ω', s @ ..] => super::t1::n1290(s, out),
				_ => false,
			}
		}
		['μ', s @ ..] => super::t1::n620(s, out),
		['ν', s @ ..] => super::t1::n621(s, out),
		['ξ', s @ ..] => super::t1::n622(s, out),
		['π', s @ ..] => super::t1::n623(s, out),
		['ρ', s @ ..] => super::t1::n624(s, out),
		['σ', s @ ..] => super::t1::n625(s, out),
		['τ', s @ ..] => super::t1::n626(s, out),
		['φ', s @ ..] => super::t1::n627(s, out),
		['χ', s @ ..] => super::t1::n628(s, out),
		_ => false,
	}
}

/// Words ending -λελυκα.
pub(crate) fn n1318(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1316);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 2033);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2692);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3241);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3627);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3628);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ν', s @ ..] => {
					super::push(out, 2693);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3242);
							match s {
								[] => true,
								['ξ', s @ ..] => {
									super::push(out, 3629);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3834);
											s.is_empty()
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 3630);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3835);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3631);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3836);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3922);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2694);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3243);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3632);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['τ', s @ ..] => {
					super::push(out, 2695);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3244);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3633);
									s.is_empty()
								}
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
			super::push(out, 2034);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2696);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3245);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3634);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3837);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3923);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 2697);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3246);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		['κ', s @ ..] => {
			super::push(out, 2035);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2698);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 2036);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2699);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3247);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3636);
									s.is_empty()
								}
								['ν', s @ ..] => {
									super::push(out, 3637);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3838);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 3638);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3839);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3924);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['π', s @ ..] => {
									super::push(out, 3639);
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
				['ρ', s @ ..] => {
					super::push(out, 2700);
					match s {
						[] => true,
						['π', s @ ..] => {
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

/// Words ending -εκλυκα.
pub(crate) fn n1319(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1317);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2037);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2701);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 3249);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3640);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -πλυκα.
pub(crate) fn n634(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 634);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1318);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2038);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2702);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3250);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3641);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωλυκα.
pub(crate) fn n635(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 635);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 1319);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2039);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2703);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μυκα.
pub(crate) fn n179(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 179);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 636);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1320);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2040);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2704);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3251);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3642);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 637);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1321);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2041);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 638);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1322);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2042);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2705);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3252);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νυκα.
pub(crate) fn n180(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 180);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 639);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1323);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2043);
							s.is_empty()
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1324);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2044);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2706);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3253);
											match s {
												[] => true,
												['ι', s @ ..] => {
													super::push(out, 3643);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3862(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3254);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3644);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3863(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ουκα.
pub(crate) fn n181(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 181);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 640);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1325);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2045);
							match s {
								[] => true,
								['κ', s @ ..] => {
									match s {
										[] => true,
										['γ', s @ ..] => {
											super::push(out, 3255);
											match s {
												[] => true,
												['υ', s @ ..] => {
													super::push(out, 3645);
													match s {
														[] => true,
														['σ', s @ ..] => super::t1::n3864(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['κ', s @ ..] => {
											super::push(out, 3256);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3646);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3865(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['ο', s @ ..] => {
											super::push(out, 3257);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3647);
													match s {
														[] => true,
														['α', s @ ..] => super::t1::n3866(s, out),
														_ => false,
													}
												}
												['ρ', s @ ..] => {
													super::push(out, 3648);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3867(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										['σ', s @ ..] => {
											super::push(out, 3258);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3649);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3868(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρυκα.
pub(crate) fn n182(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 182);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 641);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1326);
					match s {
						[] => true,
						['θ', s @ ..] => {
							super::push(out, 2046);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 642);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1327);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2047);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2707);
									match s {
										[] => true,
										['δ', s @ ..] => {
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

/// Words ending -συκα.
pub(crate) fn n183(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 183);
	match s {
		[] => true,
		['σ', s @ ..] => {
			super::push(out, 643);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1328);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2048);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2708);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3260);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τυκα.
pub(crate) fn n184(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 184);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 644);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1329);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 645);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1330);
					match s {
						[] => true,
						['ξ', s @ ..] => {
							super::push(out, 2049);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2709);
									s.is_empty()
								}
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
			super::push(out, 646);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1331);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φυκα.
pub(crate) fn n185(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 185);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 647);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1332);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2050);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 2710);
									match s {
										[] => true,
										['σ', s @ ..] => {
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

/// Words ending -χυκα.
pub(crate) fn n186(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 186);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 648);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1333);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2051);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2052);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2711);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3262);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2053);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2712);
									s.is_empty()
								}
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
			super::push(out, 649);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1334);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2054);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2713);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3263);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 650);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1335);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μεταδεδωκα.
pub(crate) fn n3669(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3650);
	s.is_empty()
}

/// Words ending -αντιδεδωκα.
pub(crate) fn n3670(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3651);
	s.is_empty()
}

/// Words ending -ταποδεδωκα.
pub(crate) fn n3671(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3652);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 3850);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ραφανιδωκα.
pub(crate) fn n3672(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3653);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 3851);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σθωκα.
pub(crate) fn n661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 661);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1346);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2068);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2725);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3272);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιωκα.
pub(crate) fn n193(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 193);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 662);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1347);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2069);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2726);
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
		['β', s @ ..] => {
			super::push(out, 663);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1348);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2070);
							s.is_empty()
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
				['θ', s @ ..] => {
					super::push(out, 1349);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2071);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2727);
									s.is_empty()
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
						['ω', s @ ..] => {
							super::push(out, 2072);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2728);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['μ', s @ ..] => {
					super::push(out, 1351);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2073);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 2729);
									s.is_empty()
								}
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
			super::push(out, 665);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1352);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2074);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2730);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
		['ξ', s @ ..] => {
			super::push(out, 666);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1353);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2075);
							s.is_empty()
						}
						['π', s @ ..] => {
							super::push(out, 2076);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2731);
									s.is_empty()
								}
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
			super::push(out, 667);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1354);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2077);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 668);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1355);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2078);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 2732);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3275);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 3654);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 669);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1356);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2079);
							match s {
								[] => true,
								['β', s @ ..] => {
									super::push(out, 2733);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3276);
											match s {
												[] => true,
												['β', s @ ..] => {
													super::push(out, 3655);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κωκα.
pub(crate) fn n194(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 194);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 670);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1357);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2080);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2734);
									s.is_empty()
								}
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
			super::push(out, 671);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1358);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2081);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2735);
									s.is_empty()
								}
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
			super::push(out, 672);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1359);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2082);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2736);
									match s {
										[] => true,
										['ν', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λωκα.
pub(crate) fn n195(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 195);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 673);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1360);
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
		['β', s @ ..] => {
			super::push(out, 674);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1361);
					match s {
						[] => true,
						['ε', 'μ', '\u{1}', s @ ..] => {
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
			super::push(out, 675);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1362);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2085);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 2737);
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
				['ζ', s @ ..] => {
					super::push(out, 1363);
					s.is_empty()
				}
				['ν', s @ ..] => {
					super::push(out, 1364);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 676);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 1365);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2086);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 677);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1366);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2087);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2738);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				_ => false,
			}
		}
		['λ', s @ ..] => {
			super::push(out, 678);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1367);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2088);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 2739);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3280);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3656);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3871(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 679);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1368);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2089);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 680);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1369);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2090);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2740);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1370);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2091);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2741);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -μωκα.
pub(crate) fn n196(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 196);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 681);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1371);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2092);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 682);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1372);
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
		['ο', s @ ..] => {
			super::push(out, 683);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1373);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2094);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νωκα.
pub(crate) fn n197(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 197);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 684);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1374);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2095);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 2742);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3282);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3657);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 685);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1375);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2096);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2097);
							s.is_empty()
						}
						['τ', s @ ..] => {
							super::push(out, 2098);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 686);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1376);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2099);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2743);
									s.is_empty()
								}
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
			super::push(out, 687);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1377);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2100);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2744);
									s.is_empty()
								}
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
			super::push(out, 688);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1378);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2101);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2745);
									match s {
										[] => true,
										['κ', s @ ..] => {
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
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 689);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 1379);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 690);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1380);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2102);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2746);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3284);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3658);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξωκα.
pub(crate) fn n198(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 198);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 691);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1381);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2103);
							match s {
								[] => true,
								['λ', s @ ..] => {
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

/// Words ending -πωκα.
pub(crate) fn n199(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 199);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 692);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1382);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 693);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρωκα.
pub(crate) fn n200(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 200);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 694);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1383);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2104);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 2748);
									s.is_empty()
								}
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
			super::push(out, 695);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1384);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2105);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['δ', s @ ..] => {
			super::push(out, 696);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1385);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2106);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 697);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1386);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2107);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2750);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3285);
											match s {
												[] => true,
												['η', s @ ..] => {
													super::push(out, 3659);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1387);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2108);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2751);
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
		['η', s @ ..] => {
			super::push(out, 698);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1388);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2109);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2752);
									match s {
										[] => true,
										['π', s @ ..] => {
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
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 699);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1389);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2110);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2753);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3288);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3660);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3872(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2111);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1390);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2112);
							s.is_empty()
						}
						_ => false,
					}
				}
				['υ', s @ ..] => {
					super::push(out, 1391);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2113);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2754);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3289);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3661);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3873(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 700);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1392);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2114);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 2755);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['γ', s @ ..] => {
					super::push(out, 1393);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2115);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 2756);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['π', s @ ..] => {
					super::push(out, 1394);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2116);
							match s {
								[] => true,
								['π', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -σωκα.
pub(crate) fn n201(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 201);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 701);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1395);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2117);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 2758);
									s.is_empty()
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2118);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τωκα.
pub(crate) fn n202(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 202);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 702);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1396);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2119);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2759);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3290);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3662);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 703);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1397);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2120);
							match s {
								[] => true,
								['ε', s @ ..] => {
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
		['π', s @ ..] => {
			super::push(out, 704);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1398);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2121);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 2761);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3291);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3663);
													s.is_empty()
												}
												_ => false,
											}
										}
										['τ', s @ ..] => {
											super::push(out, 3292);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3664);
													match s {
														[] => true,
														['κ', s @ ..] => super::t1::n3874(s, out),
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
									super::push(out, 2762);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3293);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3665);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3875(s, out),
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
									super::push(out, 2763);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3294);
											s.is_empty()
										}
										_ => false,
									}
								}
								['μ', s @ ..] => {
									super::push(out, 2764);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3295);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3666);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 705);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1399);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2122);
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
		['σ', s @ ..] => {
			super::push(out, 706);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1400);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2123);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2766);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3296);
											s.is_empty()
										}
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
					super::push(out, 1401);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2124);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2767);
									match s {
										[] => true,
										['μ', s @ ..] => {
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
		_ => false,
	}
}

/// Words ending -φωκα.
pub(crate) fn n203(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 203);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 707);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1402);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2125);
							match s {
								[] => true,
								['ε', 'μ', 'α', s @ ..] => {
									super::push(out, 2768);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3298);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3667);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3876(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χωκα.
pub(crate) fn n204(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 204);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 708);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1403);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2126);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2769);
									match s {
										[] => true,
										['λ', s @ ..] => {
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
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 709);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1404);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2127);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ι', s @ ..] => {
			super::push(out, 710);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1405);
					s.is_empty()
				}
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 711);
			match s {
				[] => true,
				['ψ', s @ ..] => {
					super::push(out, 1406);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2128);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2770);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επανειληφα.
pub(crate) fn n3706(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3687);
	s.is_empty()
}

/// Words ending -προσειληφα.
pub(crate) fn n3707(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3688);
	s.is_empty()
}

/// Words ending -αποτεθλιφα.
pub(crate) fn n3708(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3689);
	s.is_empty()
}

/// Words ending -διατετριφα.
pub(crate) fn n3709(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3690);
	s.is_empty()
}

/// Words ending -αραπεπομφα.
pub(crate) fn n3710(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3691);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3868);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προπεπομφα.
pub(crate) fn n3711(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3692);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 3869);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3929);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υποκεκλοφα.
pub(crate) fn n3712(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3693);
	s.is_empty()
}

/// Words ending -διατετροφα.
pub(crate) fn n3713(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3694);
	s.is_empty()
}

/// Words ending -ανατετροφα.
pub(crate) fn n3714(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3695);
	s.is_empty()
}

/// Words ending -επιτετροφα.
pub(crate) fn n3715(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3696);
	s.is_empty()
}

/// Words ending -ρεκτετροφα.
pub(crate) fn n3716(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3697);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3870);
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

/// Words ending -αποτετροφα.
pub(crate) fn n3717(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3698);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3871);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υποτετροφα.
pub(crate) fn n3718(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3699);
	s.is_empty()
}

/// Words ending -προτετροφα.
pub(crate) fn n3719(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3700);
	s.is_empty()
}

/// Words ending -ποκεκαλυφα.
pub(crate) fn n3720(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3701);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3872);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 3931);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -διακεκρυφα.
pub(crate) fn n3721(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3702);
	s.is_empty()
}

/// Words ending -ανακεκρυφα.
pub(crate) fn n3722(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3703);
	s.is_empty()
}

/// Words ending -αρακεκρυφα.
pub(crate) fn n3723(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3704);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3873);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ατακεκρυφα.
pub(crate) fn n3724(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3705);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3874);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -συγκεκρυφα.
pub(crate) fn n3725(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3706);
	s.is_empty()
}

/// Words ending -επικεκρυφα.
pub(crate) fn n3726(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3707);
	s.is_empty()
}

/// Words ending -ερικεκρυφα.
pub(crate) fn n3727(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3708);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3875);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μφικεκρυφα.
pub(crate) fn n3728(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3709);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3876);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αποκεκρυφα.
pub(crate) fn n3729(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3710);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3877);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υποκεκρυφα.
pub(crate) fn n3730(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3711);
	s.is_empty()
}

/// Words ending -προκεκρυφα.
pub(crate) fn n3731(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3712);
	s.is_empty()
}

/// Words ending -αποδεδειχα.
pub(crate) fn n3734(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3715);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3879);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -υποδεδειχα.
pub(crate) fn n3735(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3716);
	s.is_empty()
}

/// Words ending -καταμεμιχα.
pub(crate) fn n3736(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3717);
	s.is_empty()
}

/// Words ending -συνενηνοχα.
pub(crate) fn n3737(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3718);
	s.is_empty()
}

/// Words ending -αδιδεδιωχα.
pub(crate) fn n3738(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3719);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 3880);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3932);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πικεχειμακα.
pub(crate) fn n3741(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3722);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3881);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προητοιμακα.
pub(crate) fn n3742(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3723);
	s.is_empty()
}

/// Words ending -αταγεγηρακα.
pub(crate) fn n3743(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3724);
	s.is_empty()
}

/// Words ending -μεταπεφρακα.
pub(crate) fn n3744(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3725);
	s.is_empty()
}

/// Words ending -ποκαθεστακα.
pub(crate) fn n3745(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3726);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3882);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -τευτακα.
pub(crate) fn n1596(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1591);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 2288);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2920);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word επεσκευακα.
pub(crate) fn n3746(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3727);
	s.is_empty()
}

/// Words ending -κατεσκευακα.
pub(crate) fn n3747(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3728);
	s.is_empty()
}

/// Words ending -λελητουργηκα.
pub(crate) fn n3905(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3886);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3933);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 3948);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τετραγωδηκα.
pub(crate) fn n3759(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3740);
	s.is_empty()
}

/// Words ending -εφιλομαθηκα.
pub(crate) fn n3760(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3741);
	s.is_empty()
}

/// Words ending -ραβεβοηθηκα.
pub(crate) fn n3761(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3742);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3887);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3934);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πιβεβοηθηκα.
pub(crate) fn n3762(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3743);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3888);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οσβεβοηθηκα.
pub(crate) fn n3763(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3744);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 3889);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3935);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -λουθηκα.
pub(crate) fn n1661(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1656);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 2339);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 2963);
					match s {
						[] => true,
						['η', s @ ..] => {
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

/// Words ending -εριπεποιηκα.
pub(crate) fn n3764(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3745);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3890);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ντιπεποιηκα.
pub(crate) fn n3765(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3746);
	s.is_empty()
}

/// Words ending -ροσπεποιηκα.
pub(crate) fn n3766(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3747);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3891);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παραβεβληκα.
pub(crate) fn n3767(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3748);
	s.is_empty()
}

/// Words ending -καταβεβληκα.
pub(crate) fn n3768(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3749);
	s.is_empty()
}

/// Words ending -μεταβεβληκα.
pub(crate) fn n3769(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3750);
	s.is_empty()
}

/// Words ending -υπερβεβληκα.
pub(crate) fn n3770(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3751);
	s.is_empty()
}

/// Words ending -εκηληκα.
pub(crate) fn n1688(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1683);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 2366);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιληκα.
pub(crate) fn n427(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 427);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 979);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1684);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2367);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 2990);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1685);
					match s {
						[] => true,
						['ω', s @ ..] => {
							super::push(out, 2368);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 980);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1686);
					s.is_empty()
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 981);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1687);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2369);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -κληκα.
pub(crate) fn n428(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 428);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 982);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1688);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2370);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2371);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 2991);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3459);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 2992);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3460);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3752);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 2993);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3461);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
						['γ', s @ ..] => {
							super::push(out, 2372);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2994);
									s.is_empty()
								}
								['υ', s @ ..] => {
									super::push(out, 2995);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3462);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2373);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 2996);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3463);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['κ', s @ ..] => {
							super::push(out, 2374);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 2997);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2375);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 2998);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3464);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['σ', s @ ..] => {
							super::push(out, 2376);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 2999);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3465);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3754);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 983);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1689);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οληκα.
pub(crate) fn n429(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 429);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 984);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1690);
					match s {
						[] => true,
						['χ', s @ ..] => {
							super::push(out, 2377);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3000);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3466);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3755);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3911(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 985);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1691);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2378);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 986);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1692);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2379);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πληκα.
pub(crate) fn n430(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 430);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 987);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1693);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2380);
							s.is_empty()
						}
						['α', s @ ..] => {
							super::push(out, 2381);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3001);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3467);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['μ', s @ ..] => {
							super::push(out, 2382);
							match s {
								[] => true,
								['ε', s @ ..] => {
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
		_ => false,
	}
}

/// The word τετληκα.
pub(crate) fn n431(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 431);
	s.is_empty()
}

/// Words ending -ριδεδραμηκα.
pub(crate) fn n3775(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3756);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3893);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -απεπολεμηκα.
pub(crate) fn n3776(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3757);
	match s {
		[] => true,
		['τ', s @ ..] => {
			super::push(out, 3894);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 3936);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 3949);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -μπεπολεμηκα.
pub(crate) fn n3777(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3758);
	s.is_empty()
}

/// Words ending -εβλασφημηκα.
pub(crate) fn n3778(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3759);
	match s {
		[] => true,
		['β', s @ ..] => {
			super::push(out, 3895);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προτετιμηκα.
pub(crate) fn n3779(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3760);
	s.is_empty()
}

/// Words ending -τετολμηκα.
pub(crate) fn n3028(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3010);
	s.is_empty()
}

/// Words ending -ομηκα.
pub(crate) fn n439(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 439);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1000);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1707);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2394);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3011);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1708);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2395);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 3012);
									s.is_empty()
								}
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
					super::push(out, 1709);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 2396);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1002);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1710);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2397);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1711);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2398);
							match s {
								[] => true,
								['ω', s @ ..] => {
									super::push(out, 3013);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2399);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3014);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3474);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3761);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3915(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1003);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 1712);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2400);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3015);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3475);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3762);
													match s {
														[] => true,
														['μ', s @ ..] => super::t1::n3916(s, out),
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
									super::push(out, 3016);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3476);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3763);
													match s {
														[] => true,
														['τ', s @ ..] => super::t1::n3917(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1713);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2401);
							match s {
								[] => true,
								['τ', s @ ..] => {
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
		['τ', s @ ..] => {
			super::push(out, 1004);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1714);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2402);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3018);
									match s {
										[] => true,
										['φ', s @ ..] => {
											super::push(out, 3477);
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
						_ => false,
					}
				}
				['ρ', s @ ..] => {
					super::push(out, 1715);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2403);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3019);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3478);
											match s {
												[] => true,
												['κ', s @ ..] => {
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
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ρμηκα.
pub(crate) fn n440(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 440);
	match s {
		[] => true,
		['ω', s @ ..] => {
			super::push(out, 1005);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -σμηκα.
pub(crate) fn n441(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 441);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1006);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1716);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2404);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3020);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3479);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τμηκα.
pub(crate) fn n442(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 442);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1007);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1717);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2405);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3021);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3480);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ι', s @ ..] => {
							super::push(out, 2406);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3022);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3481);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ρ', s @ ..] => {
									super::push(out, 3023);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3482);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3766);
													s.is_empty()
												}
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
							super::push(out, 2407);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3024);
									s.is_empty()
								}
								_ => false,
							}
						}
						['ν', s @ ..] => {
							super::push(out, 2408);
							match s {
								[] => true,
								['υ', s @ ..] => {
									super::push(out, 3025);
									match s {
										[] => true,
										['σ', s @ ..] => {
											super::push(out, 3483);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ο', s @ ..] => {
							super::push(out, 2409);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3026);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3484);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υμηκα.
pub(crate) fn n443(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 443);
	match s {
		[] => true,
		['θ', s @ ..] => {
			super::push(out, 1008);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1718);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2410);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3027);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3485);
											s.is_empty()
										}
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
					super::push(out, 1719);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -επιδεδινηκα.
pub(crate) fn n3786(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3767);
	s.is_empty()
}

/// Words ending -ονηκα.
pub(crate) fn n452(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 452);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1029);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1740);
					match s {
						[] => true,
						['γ', s @ ..] => {
							super::push(out, 2427);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3037);
									match s {
										[] => true,
										['ω', s @ ..] => {
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
			super::push(out, 1030);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1741);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2428);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
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
		['κ', s @ ..] => {
			super::push(out, 1031);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1742);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2429);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3039);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3491);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3768);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1032);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1743);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2430);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3040);
									s.is_empty()
								}
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
			super::push(out, 1033);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1744);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2431);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3041);
									s.is_empty()
								}
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
			super::push(out, 1034);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1745);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2432);
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3042);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3043);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3492);
											s.is_empty()
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 3044);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3493);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1035);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1746);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2433);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3045);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3494);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3769);
													s.is_empty()
												}
												_ => false,
											}
										}
										['ι', s @ ..] => {
											super::push(out, 3495);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3770);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3918(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2434);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3046);
									match s {
										[] => true,
										['ε', s @ ..] => {
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
		['τ', s @ ..] => {
			super::push(out, 1036);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1747);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2435);
							match s {
								[] => true,
								['φ', s @ ..] => {
									super::push(out, 3047);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3497);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3771);
													match s {
														[] => true,
														['δ', s @ ..] => super::t1::n3919(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1037);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1748);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2436);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3048);
									match s {
										[] => true,
										['δ', s @ ..] => {
											super::push(out, 3498);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1038);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1749);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2437);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3049);
									match s {
										[] => true,
										['α', s @ ..] => {
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

/// Words ending -ιπνηκα.
pub(crate) fn n1040(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1039);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1750);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2438);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3050);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3500);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υνηκα.
pub(crate) fn n454(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 454);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1041);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1751);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2439);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['κ', s @ ..] => {
			super::push(out, 1042);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1752);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2440);
							s.is_empty()
						}
						_ => false,
					}
				}
				['σ', s @ ..] => {
					super::push(out, 1753);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2441);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3051);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3501);
											match s {
												[] => true,
												['ε', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωνηκα.
pub(crate) fn n455(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 455);
	match s {
		[] => true,
		['ν', s @ ..] => {
			super::push(out, 1043);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1754);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2442);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3052);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3502);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1044);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1755);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2443);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3053);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3503);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3773);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1045);
			match s {
				[] => true,
				['ω', s @ ..] => {
					super::push(out, 1756);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -καταβεβοηκα.
pub(crate) fn n3793(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3774);
	s.is_empty()
}

/// Words ending -πεπροτερηκα.
pub(crate) fn n3794(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3775);
	s.is_empty()
}

/// Words ending -ελαγοθηρηκα.
pub(crate) fn n3795(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3776);
	s.is_empty()
}

/// Words ending -εγκεχειρηκα.
pub(crate) fn n3796(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3777);
	s.is_empty()
}

/// Words ending -ορηκα.
pub(crate) fn n469(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 469);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1074);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1789);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2471);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3073);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3518);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3778);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['β', s @ ..] => {
									super::push(out, 3074);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3519);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3779);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3920(s, out),
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
									super::push(out, 3075);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3520);
											match s {
												[] => true,
												['μ', s @ ..] => {
													super::push(out, 3780);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3921(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2472);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3076);
									match s {
										[] => true,
										['κ', s @ ..] => {
											super::push(out, 3521);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1075);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1790);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2473);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3077);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3522);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1076);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1791);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2474);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3078);
									s.is_empty()
								}
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
			super::push(out, 1077);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1792);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2475);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1793);
					match s {
						[] => true,
						['ζ', s @ ..] => {
							super::push(out, 2476);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3079);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3523);
											match s {
												[] => true,
												['ε', s @ ..] => {
													super::push(out, 3781);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3922(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1078);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1794);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2477);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3080);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3524);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3782);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1795);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2478);
							s.is_empty()
						}
						_ => false,
					}
				}
				['ο', s @ ..] => {
					super::push(out, 1796);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2479);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υρηκα.
pub(crate) fn n473(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 473);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1083);
			match s {
				[] => true,
				['\u{1}', s @ ..] => {
					super::push(out, 1799);
					s.is_empty()
				}
				['ξ', s @ ..] => {
					super::push(out, 1800);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2481);
							s.is_empty()
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1801);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2482);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ξ', s @ ..] => {
			super::push(out, 1084);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1802);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1085);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1803);
					match s {
						[] => true,
						['φ', s @ ..] => {
							super::push(out, 2483);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3082);
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
				_ => false,
			}
		}
		['τ', s @ ..] => {
			super::push(out, 1086);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1804);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2484);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3083);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωρηκα.
pub(crate) fn n474(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 474);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1087);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1805);
					s.is_empty()
				}
				_ => false,
			}
		}
		['ε', s @ ..] => {
			super::push(out, 1088);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1806);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2485);
							match s {
								[] => true,
								['τ', s @ ..] => {
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
		['μ', s @ ..] => {
			super::push(out, 1089);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1807);
					match s {
						[] => true,
						['τ', s @ ..] => {
							super::push(out, 2486);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3085);
									match s {
										[] => true,
										['τ', s @ ..] => {
											super::push(out, 3526);
											match s {
												[] => true,
												['ο', s @ ..] => {
													super::push(out, 3783);
													match s {
														[] => true,
														['ρ', s @ ..] => super::t1::n3923(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
				['ι', s @ ..] => {
					super::push(out, 1808);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2487);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3086);
									match s {
										[] => true,
										['α', s @ ..] => {
											super::push(out, 3527);
											match s {
												[] => true,
												['τ', s @ ..] => {
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
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1091);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1809);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2488);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3087);
									s.is_empty()
								}
								['ο', s @ ..] => {
									super::push(out, 3088);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3528);
											s.is_empty()
										}
										['ρ', s @ ..] => {
											super::push(out, 3529);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3785);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3924(s, out),
														_ => false,
													}
												}
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
							super::push(out, 2489);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3089);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3530);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3786);
													s.is_empty()
												}
												_ => false,
											}
										}
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
					super::push(out, 1810);
					match s {
						[] => true,
						['ν', 'ε', 'τ', 'σ', 'ε', '\u{1}', s @ ..] => {
							super::push(out, 2490);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εριπεπατηκα.
pub(crate) fn n3806(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3787);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3906);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -λινοστατηκα.
pub(crate) fn n3807(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3788);
	s.is_empty()
}

/// Words ending -ελογοθετηκα.
pub(crate) fn n3808(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3789);
	s.is_empty()
}

/// Words ending -ετοποθετηκα.
pub(crate) fn n3809(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3790);
	s.is_empty()
}

/// Words ending -νεζητηκα.
pub(crate) fn n2521(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 2507);
	match s {
		[] => true,
		['υ', s @ ..] => {
			super::push(out, 3102);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 3539);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ιητηκα.
pub(crate) fn n1106(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1105);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1828);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2508);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 3103);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εξητηκα.
pub(crate) fn n1834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1829);
	s.is_empty()
}

/// Words ending -πητηκα.
pub(crate) fn n1108(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1107);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1830);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ιτηκα.
pub(crate) fn n482(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 482);
	match s {
		[] => true,
		['ο', s @ ..] => {
			super::push(out, 1108);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 1831);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2509);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3104);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3540);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3791);
													match s {
														[] => true,
														['ε', s @ ..] => super::t1::n3926(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ηγανακτηκα.
pub(crate) fn n483(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 483);
	s.is_empty()
}

/// Words ending -ντηκα.
pub(crate) fn n484(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 484);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 1109);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 1832);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2510);
							match s {
								[] => true,
								['σ', s @ ..] => {
									super::push(out, 3105);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -οτηκα.
pub(crate) fn n485(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 485);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 1110);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 1833);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2511);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3106);
									match s {
										[] => true,
										['ο', s @ ..] => {
											super::push(out, 3541);
											match s {
												[] => true,
												['κ', s @ ..] => {
													super::push(out, 3792);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1111);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1834);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2512);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3107);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πτηκα.
pub(crate) fn n486(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 486);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1112);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 1835);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2513);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3108);
									s.is_empty()
								}
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
			super::push(out, 1113);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ρτηκα.
pub(crate) fn n487(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 487);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1114);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 1836);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2514);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1115);
			match s {
				[] => true,
				['ξ', s @ ..] => {
					super::push(out, 1837);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2515);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -στηκα.
pub(crate) fn n488(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 488);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1116);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 1838);
					match s {
						[] => true,
						['β', s @ ..] => {
							super::push(out, 2516);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3109);
									match s {
										[] => true,
										['β', s @ ..] => {
											super::push(out, 3542);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3793);
													match s {
														[] => true,
														['ν', s @ ..] => super::t1::n3927(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
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
					super::push(out, 1839);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2517);
							match s {
								[] => true,
								['δ', s @ ..] => {
									super::push(out, 3110);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3543);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3794);
													match s {
														[] => true,
														['π', s @ ..] => super::t1::n3928(s, out),
														_ => false,
													}
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1117);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 1840);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2518);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3111);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3544);
											match s {
												[] => true,
												['τ', s @ ..] => {
													super::push(out, 3795);
													s.is_empty()
												}
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
							super::push(out, 2519);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3112);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 1841);
					match s {
						[] => true,
						['δ', s @ ..] => {
							super::push(out, 2520);
							s.is_empty()
						}
						['ρ', s @ ..] => {
							super::push(out, 2521);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3113);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3545);
											match s {
												[] => true,
												['\u{1}', s @ ..] => {
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
				['ν', s @ ..] => {
					super::push(out, 1842);
					match s {
						[] => true,
						['υ', s @ ..] => {
							super::push(out, 2522);
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
				['ο', s @ ..] => {
					super::push(out, 1843);
					match s {
						[] => true,
						['ρ', s @ ..] => {
							super::push(out, 2523);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3115);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['φ', s @ ..] => {
					super::push(out, 1844);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2524);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3116);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3546);
											match s {
												[] => true,
												['ξ', s @ ..] => {
													super::push(out, 3797);
													match s {
														[] => true,
														['\u{1}', s @ ..] => super::t1::n3929(s, out),
														_ => false,
													}
												}
												['σ', s @ ..] => {
													super::push(out, 3798);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1118);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1845);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2525);
							match s {
								[] => true,
								['η', s @ ..] => {
									super::push(out, 3117);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3547);
											match s {
												[] => true,
												['α', s @ ..] => {
													super::push(out, 3799);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υτηκα.
pub(crate) fn n489(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 489);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 1119);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 1846);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2526);
							match s {
								[] => true,
								['γ', s @ ..] => {
									super::push(out, 3118);
									match s {
										[] => true,
										['ω', s @ ..] => {
											super::push(out, 3548);
											match s {
												[] => true,
												['ρ', s @ ..] => {
													super::push(out, 3800);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωτηκα.
pub(crate) fn n490(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 490);
	match s {
		[] => true,
		['ρ', s @ ..] => {
			super::push(out, 1120);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 1847);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word εζωγραφηκα.
pub(crate) fn n3820(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3801);
	s.is_empty()
}

/// Words ending -μεμαστιχηκα.
pub(crate) fn n3821(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3802);
	s.is_empty()
}

/// Words ending -αμπαδαρχηκα.
pub(crate) fn n3822(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3803);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 3911);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αμπαδουχηκα.
pub(crate) fn n3823(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3804);
	s.is_empty()
}

/// Words ending -δεδυστυχηκα.
pub(crate) fn n3824(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3805);
	s.is_empty()
}

/// Words ending -παρατεθεικα.
pub(crate) fn n3827(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3808);
	s.is_empty()
}

/// Words ending -ερικεκλεικα.
pub(crate) fn n3828(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3809);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3912);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -αποκεκλεικα.
pub(crate) fn n3829(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3810);
	s.is_empty()
}

/// Words ending -τετειχικικα.
pub(crate) fn n3831(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3812);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3913);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -οκεκεφαλικα.
pub(crate) fn n3832(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3813);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3914);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κατακεκλικα.
pub(crate) fn n3833(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3814);
	s.is_empty()
}

/// Words ending -διακεκομικα.
pub(crate) fn n3834(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3815);
	s.is_empty()
}

/// Words ending -ετακεκομικα.
pub(crate) fn n3835(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3816);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 3915);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ροσκεκομικα.
pub(crate) fn n3836(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3817);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3916);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χαρακτηρικα.
pub(crate) fn n3837(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3818);
	s.is_empty()
}

/// Words ending -τακεχειρικα.
pub(crate) fn n3838(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3819);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3917);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -κατακεκρικα.
pub(crate) fn n3839(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3820);
	s.is_empty()
}

/// Words ending -επορικα.
pub(crate) fn n1950(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1944);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 2614);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 3188);
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

/// Words ending -πρικα.
pub(crate) fn n571(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 571);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1219);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1945);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2615);
							match s {
								[] => true,
								['ι', s @ ..] => {
									super::push(out, 3189);
									match s {
										[] => true,
										['δ', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -υρικα.
pub(crate) fn n572(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 572);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 1220);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1946);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2616);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['μ', s @ ..] => {
			super::push(out, 1221);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1947);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2617);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ν', s @ ..] => {
			super::push(out, 1222);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 1948);
					match s {
						[] => true,
						['μ', s @ ..] => {
							super::push(out, 2618);
							match s {
								[] => true,
								['ε', s @ ..] => {
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

/// Words ending -φρικα.
pub(crate) fn n573(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 573);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1223);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 1949);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2619);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['η', s @ ..] => {
			super::push(out, 1224);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -χρικα.
pub(crate) fn n574(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 574);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 1225);
			match s {
				[] => true,
				['κ', s @ ..] => {
					super::push(out, 1950);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2620);
							s.is_empty()
						}
						['ι', s @ ..] => {
							super::push(out, 2621);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ωρικα.
pub(crate) fn n575(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 575);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 1226);
			s.is_empty()
		}
		['ν', s @ ..] => {
			super::push(out, 1227);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 1951);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2622);
							match s {
								[] => true,
								['ν', s @ ..] => {
									super::push(out, 3191);
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
		['ο', s @ ..] => {
			super::push(out, 1228);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 1952);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2623);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['φ', s @ ..] => {
			super::push(out, 1229);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 1953);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2624);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['χ', s @ ..] => {
			super::push(out, 1230);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 1954);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2625);
							match s {
								[] => true,
								['α', s @ ..] => {
									super::push(out, 3192);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3597);
											match s {
												[] => true,
												['δ', s @ ..] => {
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
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νακεκουφικα.
pub(crate) fn n3841(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3822);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3918);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εκτετειχικα.
pub(crate) fn n3842(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3823);
	s.is_empty()
}

/// Words ending -ποτετειχικα.
pub(crate) fn n3843(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3824);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3919);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ηγχονιζωικα.
pub(crate) fn n3844(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3825);
	s.is_empty()
}

/// Words ending -προπεπαιδευκα.
pub(crate) fn n3846(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3827);
	s.is_empty()
}

/// Words ending -βεβουλευκα.
pub(crate) fn n3632(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3613);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 3828);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 3920);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 3946);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3952);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3955);
											s.is_empty()
										}
										_ => false,
									}
								}
								['τ', s @ ..] => {
									super::push(out, 3953);
									match s {
										[] => true,
										['ν', s @ ..] => {
											super::push(out, 3956);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 3829);
			match s {
				[] => true,
				['υ', s @ ..] => {
					super::push(out, 3921);
					match s {
						[] => true,
						['σ', s @ ..] => {
							super::push(out, 3947);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εδουλευκα.
pub(crate) fn n3234(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3216);
	match s {
		[] => true,
		['δ', s @ ..] => {
			super::push(out, 3614);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ωλευκα.
pub(crate) fn n1290(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 1288);
	s.is_empty()
}

/// Words ending -μευκα.
pub(crate) fn n620(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 620);
	match s {
		[] => true,
		['ι', s @ ..] => {
			super::push(out, 1289);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2005);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2666);
							match s {
								[] => true,
								['ρ', s @ ..] => {
									super::push(out, 3217);
									match s {
										[] => true,
										['χ', s @ ..] => {
											super::push(out, 3615);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -νευκα.
pub(crate) fn n621(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 621);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1290);
			match s {
				[] => true,
				['χ', s @ ..] => {
					super::push(out, 2006);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2667);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3218);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3616);
											match s {
												[] => true,
												['λ', s @ ..] => {
													super::push(out, 3830);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
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
			super::push(out, 1291);
			match s {
				[] => true,
				['ν', s @ ..] => {
					super::push(out, 2007);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2668);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1292);
			match s {
				[] => true,
				['μ', s @ ..] => {
					super::push(out, 2008);
					s.is_empty()
				}
				_ => false,
			}
		}
		['π', s @ ..] => {
			super::push(out, 1293);
			match s {
				[] => true,
				['ε', s @ ..] => {
					super::push(out, 2009);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2669);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ρ', s @ ..] => {
			super::push(out, 1294);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2010);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2670);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3219);
									match s {
										[] => true,
										['π', s @ ..] => {
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
				_ => false,
			}
		}
		['υ', s @ ..] => {
			super::push(out, 1295);
			match s {
				[] => true,
				['δ', s @ ..] => {
					super::push(out, 2011);
					match s {
						[] => true,
						['ν', 'ι', 'κ', 'ε', 'κ', s @ ..] => {
							match s {
								[] => true,
								['\u{1}', s @ ..] => {
									super::push(out, 3220);
									s.is_empty()
								}
								['α', s @ ..] => {
									super::push(out, 3221);
									match s {
										[] => true,
										['ι', s @ ..] => {
											super::push(out, 3618);
											match s {
												[] => true,
												['δ', s @ ..] => {
													super::push(out, 3831);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['γ', s @ ..] => {
									super::push(out, 3222);
									match s {
										[] => true,
										['υ', s @ ..] => {
											super::push(out, 3619);
											match s {
												[] => true,
												['σ', s @ ..] => {
													super::push(out, 3832);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								['ο', s @ ..] => {
									super::push(out, 3223);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3620);
											match s {
												[] => true,
												['π', s @ ..] => {
													super::push(out, 3833);
													s.is_empty()
												}
												_ => false,
											}
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -ξευκα.
pub(crate) fn n622(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 622);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1296);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2012);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2671);
							match s {
								[] => true,
								['λ', s @ ..] => {
									super::push(out, 3224);
									s.is_empty()
								}
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
			super::push(out, 1297);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 2013);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -πευκα.
pub(crate) fn n623(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 623);
	s.is_empty()
}

/// Words ending -ρευκα.
pub(crate) fn n624(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 624);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1298);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2014);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2672);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3225);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3621);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1299);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2015);
					match s {
						[] => true,
						['\u{1}', s @ ..] => {
							super::push(out, 2673);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		['ο', s @ ..] => {
			super::push(out, 1300);
			match s {
				[] => true,
				['γ', s @ ..] => {
					super::push(out, 2016);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2674);
							match s {
								[] => true,
								['ο', s @ ..] => {
									super::push(out, 3226);
									match s {
										[] => true,
										['ρ', s @ ..] => {
											super::push(out, 3622);
											s.is_empty()
										}
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
					super::push(out, 2017);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2675);
							match s {
								[] => true,
								['κ', s @ ..] => {
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
		['τ', s @ ..] => {
			super::push(out, 1301);
			match s {
				[] => true,
				['α', s @ ..] => {
					super::push(out, 2018);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -σευκα.
pub(crate) fn n625(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 625);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1302);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2019);
					match s {
						[] => true,
						['ι', s @ ..] => {
							super::push(out, 2676);
							match s {
								[] => true,
								['τ', s @ ..] => {
									super::push(out, 3228);
									match s {
										[] => true,
										['ε', s @ ..] => {
											super::push(out, 3623);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -τευκα.
pub(crate) fn n626(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 626);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 1303);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2020);
					match s {
						[] => true,
						['κ', s @ ..] => {
							super::push(out, 2677);
							match s {
								[] => true,
								['ε', s @ ..] => {
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
		['ε', s @ ..] => {
			super::push(out, 1304);
			match s {
				[] => true,
				['π', s @ ..] => {
					super::push(out, 2021);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2678);
							match s {
								[] => true,
								['ρ', s @ ..] => {
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
		['η', s @ ..] => {
			super::push(out, 1305);
			match s {
				[] => true,
				['θ', s @ ..] => {
					super::push(out, 2022);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 2679);
							match s {
								[] => true,
								['μ', s @ ..] => {
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
			super::push(out, 1306);
			match s {
				[] => true,
				['λ', s @ ..] => {
					super::push(out, 2023);
					match s {
						[] => true,
						['ο', s @ ..] => {
							super::push(out, 2680);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3232);
									s.is_empty()
								}
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
			super::push(out, 1307);
			match s {
				[] => true,
				['η', s @ ..] => {
					super::push(out, 2024);
					match s {
						[] => true,
						['ν', s @ ..] => {
							super::push(out, 2681);
							match s {
								[] => true,
								['μ', s @ ..] => {
									super::push(out, 3233);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				['ι', s @ ..] => {
					super::push(out, 2025);
					match s {
						[] => true,
						['π', s @ ..] => {
							super::push(out, 2682);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3234);
									match s {
										[] => true,
										['π', s @ ..] => {
											super::push(out, 3624);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						['ρ', s @ ..] => {
							super::push(out, 2683);
							match s {
								[] => true,
								['η', s @ ..] => {
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
		['υ', s @ ..] => {
			super::push(out, 1308);
			match s {
				[] => true,
				['φ', s @ ..] => {
					super::push(out, 2026);
					match s {
						[] => true,
						['ε', s @ ..] => {
							super::push(out, 2684);
							match s {
								[] => true,
								['π', s @ ..] => {
									super::push(out, 3236);
									s.is_empty()
								}
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
			super::push(out, 1309);
			match s {
				[] => true,
				['σ', s @ ..] => {
					super::push(out, 2027);
					match s {
						[] => true,
						['η', s @ ..] => {
							super::push(out, 2685);
							s.is_empty()
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -φευκα.
pub(crate) fn n627(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 627);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 1310);
			match s {
				[] => true,
				['ι', s @ ..] => {
					super::push(out, 2028);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2686);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3237);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3625);
											s.is_empty()
										}
										_ => false,
									}
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -χευκα.
pub(crate) fn n628(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 628);
	match s {
		[] => true,
		['γ', s @ ..] => {
			super::push(out, 1311);
			match s {
				[] => true,
				['ο', s @ ..] => {
					super::push(out, 2029);
					match s {
						[] => true,
						['λ', s @ ..] => {
							super::push(out, 2687);
							match s {
								[] => true,
								['ε', s @ ..] => {
									super::push(out, 3238);
									match s {
										[] => true,
										['λ', s @ ..] => {
											super::push(out, 3626);
											s.is_empty()
										}
										_ => false,
									}
								}
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
			super::push(out, 1312);
			match s {
				[] => true,
				['ρ', s @ ..] => {
					super::push(out, 2030);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -διαμεμηνυκα.
pub(crate) fn n3862(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3843);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3925);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -προμεμηνυκα.
pub(crate) fn n3863(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3844);
	s.is_empty()
}

/// Words ending -συγκεκρουκα.
pub(crate) fn n3864(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3845);
	s.is_empty()
}

/// The word εκκεκρουκα.
pub(crate) fn n3865(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3846);
	s.is_empty()
}

/// Words ending -αποκεκρουκα.
pub(crate) fn n3866(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3847);
	s.is_empty()
}

/// Words ending -προκεκρουκα.
pub(crate) fn n3867(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3848);
	match s {
		[] => true,
		['\u{1}', s @ ..] => {
			super::push(out, 3926);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ροσκεκρουκα.
pub(crate) fn n3868(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3849);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3927);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -επαστιλλωκα.
pub(crate) fn n3871(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3852);
	s.is_empty()
}

/// Words ending -κατεπετρωκα.
pub(crate) fn n3872(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3853);
	s.is_empty()
}

/// Words ending -πολελυτρωκα.
pub(crate) fn n3873(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3854);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3928);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -καταπεπτωκα.
pub(crate) fn n3874(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3855);
	s.is_empty()
}

/// Words ending -περιπεπτωκα.
pub(crate) fn n3875(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3856);
	s.is_empty()
}

/// Words ending -μεταμεμορφωκα.
pub(crate) fn n3876(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3857);
	s.is_empty()
}

/// Words ending -ελογχοβοληκα.
pub(crate) fn n3911(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3892);
	s.is_empty()
}

/// Words ending -εκληρονομηκα.
pub(crate) fn n3915(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3896);
	match s {
		[] => true,
		['κ', s @ ..] => {
			super::push(out, 3937);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -μπαδοδρομηκα.
pub(crate) fn n3916(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3897);
	s.is_empty()
}

/// Words ending -τελεοδρομηκα.
pub(crate) fn n3917(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3898);
	match s {
		[] => true,
		['ε', s @ ..] => {
			super::push(out, 3938);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3950);
					s.is_empty()
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// Words ending -εριπεφρονηκα.
pub(crate) fn n3918(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3899);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3939);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -δελφοκτονηκα.
pub(crate) fn n3919(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3900);
	match s {
		[] => true,
		['η', s @ ..] => {
			super::push(out, 3940);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -ελαβρηγορηκα.
pub(crate) fn n3920(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3901);
	match s {
		[] => true,
		['λ', s @ ..] => {
			super::push(out, 3941);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -εμακρηγορηκα.
pub(crate) fn n3921(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3902);
	match s {
		[] => true,
		['μ', s @ ..] => {
			super::push(out, 3942);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -πεπεζοπορηκα.
pub(crate) fn n3922(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3903);
	s.is_empty()
}

/// Words ending -ροτετιμωρηκα.
pub(crate) fn n3923(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3904);
	match s {
		[] => true,
		['π', s @ ..] => {
			super::push(out, 3943);
			s.is_empty()
		}
		_ => false,
	}
}

/// The word προκεχωρηκα.
pub(crate) fn n3924(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3905);
	s.is_empty()
}

/// Words ending -επιπεφοιτηκα.
pub(crate) fn n3926(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3907);
	s.is_empty()
}

/// Words ending -ναβεβλαστηκα.
pub(crate) fn n3927(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3908);
	match s {
		[] => true,
		['α', s @ ..] => {
			super::push(out, 3944);
			s.is_empty()
		}
		_ => false,
	}
}

/// Words ending -παιδεραστηκα.
pub(crate) fn n3928(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3909);
	match s {
		[] => true,
		['ε', 'π', 'α', s @ ..] => {
			super::push(out, 3945);
			match s {
				[] => true,
				['τ', s @ ..] => {
					super::push(out, 3951);
					match s {
						[] => true,
						['α', s @ ..] => {
							super::push(out, 3954);
							match s {
								[] => true,
								['κ', s @ ..] => {
									super::push(out, 3957);
									s.is_empty()
								}
								_ => false,
							}
						}
						_ => false,
					}
				}
				_ => false,
			}
		}
		_ => false,
	}
}

/// The word ξυναφεστηκα.
pub(crate) fn n3929(s: &[char], out: &mut Path<'_>) -> bool {
	super::push(out, 3910);
	s.is_empty()
}

